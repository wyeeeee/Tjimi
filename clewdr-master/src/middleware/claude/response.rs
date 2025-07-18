use axum::{
    Json,
    body::{self, Body},
    response::{IntoResponse, Response, Sse, sse::Event},
};
use eventsource_stream::Eventsource;
use futures::{Stream, TryStreamExt};
use serde::Serialize;

use crate::{
    claude_web_state::ClaudeApiFormat,
    middleware::claude::ClaudeCodeContext,
    types::claude_message::{ContentBlockDelta, CreateMessageResponse, StreamEvent},
};

use super::ClaudeWebContext;

/// Transforms responses to ensure compatibility with the OpenAI API format
///
/// This middleware function analyzes responses and transforms them when necessary
/// to ensure compatibility between Claude and OpenAI API formats, particularly
/// for streaming responses. If the response is:
///
/// - From the Claude API format: No transformation needed
/// - Not streaming: No transformation needed
/// - Has a non-200 status code: No transformation needed
/// - OpenAI format and streaming: Transforms the stream to match OpenAI event format
///
/// # Arguments
///
/// * `resp` - The original response to be potentially transformed
///
/// # Returns
///
/// The original or transformed response as appropriate
pub async fn to_oai(resp: Response) -> impl IntoResponse {
    if let Some(f) = resp.extensions().get::<ClaudeWebContext>() {
        if ClaudeApiFormat::Claude == f.api_format || !f.stream || resp.status() != 200 {
            return resp;
        }
        let body = resp.into_body();
        let stream = body.into_data_stream().eventsource();
        let stream = transform_stream(stream);
        Sse::new(stream)
            .keep_alive(Default::default())
            .into_response()
    } else {
        let Some(ex) = resp.extensions().get::<ClaudeCodeContext>() else {
            return resp;
        };
        if ClaudeApiFormat::Claude == ex.api_format || !ex.stream || resp.status() != 200 {
            return resp;
        }
        let body = resp.into_body();
        let stream = body.into_data_stream().eventsource();
        let stream = transform_stream(stream);
        Sse::new(stream)
            .keep_alive(Default::default())
            .into_response()
    }
}

pub async fn add_usage_info(resp: Response) -> impl IntoResponse {
    let (mut usage, stream) = if let Some(f) = resp.extensions().get::<ClaudeWebContext>() {
        if ClaudeApiFormat::OpenAI == f.api_format || resp.status() != 200 {
            return resp;
        }
        (f.usage.to_owned(), f.stream)
    } else {
        let Some(ex) = resp.extensions().get::<ClaudeCodeContext>() else {
            return resp;
        };
        if ClaudeApiFormat::OpenAI == ex.api_format || resp.status() != 200 {
            return resp;
        }
        (ex.usage.to_owned(), ex.stream)
    };
    if !stream {
        let data = body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        let Ok(mut response) = serde_json::from_slice::<CreateMessageResponse>(&data) else {
            return Body::from(data).into_response();
        };
        let output_tokens = response.count_tokens();
        usage.output_tokens = output_tokens;
        response.usage = Some(usage);
        return Json(response).into_response();
    }
    let stream = resp
        .into_body()
        .into_data_stream()
        .eventsource()
        .map_ok(move |event| {
            let new_event = axum::response::sse::Event::default()
                .event(event.event)
                .id(event.id);
            let new_event = if let Some(retry) = event.retry {
                new_event.retry(retry)
            } else {
                new_event
            };
            let Ok(parsed) = serde_json::from_str::<StreamEvent>(&event.data) else {
                return new_event.data(event.data);
            };
            match parsed {
                StreamEvent::MessageStart { mut message } => {
                    message.usage = Some(usage.to_owned());
                    new_event
                        .json_data(StreamEvent::MessageStart { message })
                        .unwrap()
                }
                _ => new_event.data(event.data),
            }
        });

    Sse::new(stream)
        .keep_alive(Default::default())
        .into_response()
}

/// Represents the data structure for streaming events in OpenAI API format
/// Contains a choices array with deltas of content
#[derive(Debug, Serialize)]
struct StreamEventData {
    choices: Vec<StreamEventDelta>,
}

impl StreamEventData {
    /// Creates a new StreamEventData with the given content
    ///
    /// # Arguments
    /// * `content` - The event content to include
    ///
    /// # Returns
    /// A new StreamEventData instance with the content wrapped in choices array
    fn new(content: EventContent) -> Self {
        Self {
            choices: vec![StreamEventDelta { delta: content }],
        }
    }
}

/// Represents a delta update in a streaming response
/// Contains the content change for the current chunk
#[derive(Debug, Serialize)]
struct StreamEventDelta {
    delta: EventContent,
}

/// Content of an event, either regular content or reasoning (thinking mode)
/// Uses untagged enum to handle different response formats
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum EventContent {
    Content { content: String },
    Reasoning { reasoning_content: String },
}

/// Creates an SSE event with the given content in OpenAI format
///
/// # Arguments
/// * `content` - The event content to include
///
/// # Returns
/// A formatted SSE Event ready to be sent to the client
pub fn build_event(content: EventContent) -> Event {
    let event = Event::default();
    let data = StreamEventData::new(content);
    event.json_data(data).unwrap()
}

/// Transforms a Claude.ai event stream into an OpenAI-compatible event stream
///
/// Extracts content from Claude events and reformats them to match OpenAI's streaming format.
/// This function processes each event in the stream, identifying the delta content type
/// (text or thinking), and converting it to the appropriate OpenAI-compatible event format.
///
/// # Arguments
/// * `s` - The input stream of Claude.ai events
///
/// # Returns
/// A stream of OpenAI-compatible SSE events
///
/// # Type Parameters
/// * `I` - The input stream type
/// * `E` - The error type for the stream
pub fn transform_stream<I, E>(s: I) -> impl Stream<Item = Result<Event, E>>
where
    I: Stream<Item = Result<eventsource_stream::Event, E>>,
{
    s.try_filter_map(async |eventsource_stream::Event { data, .. }| {
        let Ok(parsed) = serde_json::from_str::<StreamEvent>(&data) else {
            return Ok(None);
        };
        let StreamEvent::ContentBlockDelta { delta, .. } = parsed else {
            return Ok(None);
        };
        match delta {
            ContentBlockDelta::TextDelta { text } => {
                Ok(Some(build_event(EventContent::Content { content: text })))
            }
            ContentBlockDelta::ThinkingDelta { thinking } => {
                Ok(Some(build_event(EventContent::Reasoning {
                    reasoning_content: thinking,
                })))
            }
            _ => Ok(None),
        }
    })
}
