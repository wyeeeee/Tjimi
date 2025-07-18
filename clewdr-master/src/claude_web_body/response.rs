use axum::{Json, body::Body, response::IntoResponse};
use bytes::Bytes;
use eventsource_stream::{EventStream, Eventsource};
use futures::{Stream, TryStreamExt};
use serde::Deserialize;
use serde_json::json;

use crate::{
    claude_web_state::{ClaudeApiFormat, ClaudeWebState},
    error::ClewdrError,
    services::cache::CACHE,
    types::claude_message::{ContentBlock, CreateMessageResponse, Message, Role},
    utils::print_out_text,
};

/// Merges server-sent events (SSE) from a stream into a single string
/// Extracts and concatenates completion data from events
///
/// # Arguments
/// * `stream` - Event stream to process
///
/// # Returns
/// Combined completion text from all events
pub async fn merge_sse(
    stream: EventStream<impl Stream<Item = Result<Bytes, wreq::Error>>>,
) -> Result<String, ClewdrError> {
    #[derive(Deserialize)]
    struct Data {
        completion: String,
    }
    Ok(stream
        .try_filter_map(async |event| {
            Ok(serde_json::from_str::<Data>(&event.data)
                .map(|data| data.completion)
                .ok())
        })
        .try_collect()
        .await?)
}

impl<S> From<S> for Message
where
    S: Into<String>,
{
    /// Converts a string into a Message with assistant role
    ///
    /// # Arguments
    /// * `str` - The text content for the message
    ///
    /// # Returns
    /// * `Message` - A message with assistant role and text content
    fn from(str: S) -> Self {
        Message::new_blocks(
            Role::Assistant,
            vec![ContentBlock::Text { text: str.into() }],
        )
    }
}

impl ClaudeWebState {
    /// Converts the response from the Claude Web into Claude API or OpenAI API format
    ///
    /// This method transforms streams of bytes from Claude's web response into the appropriate
    /// format based on the client's requested API format (Claude or OpenAI). It handles both
    /// streaming and non-streaming responses, and manages caching for responses.
    ///
    /// # Arguments
    /// * `input` - The response stream from the Claude Web API
    ///
    /// # Returns
    /// * `axum::response::Response` - Transformed response in the requested format
    pub async fn transform_response(
        &self,
        input: impl Stream<Item = Result<Bytes, wreq::Error>> + Send + 'static,
    ) -> Result<axum::response::Response, ClewdrError> {
        // response is used for caching
        if let Some((key, id)) = self.key {
            CACHE.push(input, key, id);
            // return whatever, not used
            return Ok(Body::empty().into_response());
        }
        // response is used for returning
        if self.stream {
            return Ok(Body::from_stream(input).into_response());
        }

        let stream = input.eventsource();
        let text = merge_sse(stream).await?;
        print_out_text(text.to_owned(), "non_stream.txt");
        match self.api_format {
            // Claude API format
            ClaudeApiFormat::Claude => Ok(Json(CreateMessageResponse::text(
                text,
                Default::default(),
                self.usage.to_owned(),
            ))
            .into_response()),
            // OpenAI API format
            ClaudeApiFormat::OpenAI => {
                let json = json!({
                    "id": "chatcmpl-12345",
                    "object": "chat.completion",
                    "created": 1234567890,
                    "model": "claude",
                    "choices": [{
                        "index": 0,
                        "message": {
                            "role": "assistant",
                            "content": text
                        },
                        "finish_reason": null
                    }],
                });
                Ok(Json(json).into_response())
            }
        }
    }
}
