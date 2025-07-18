use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::LazyLock,
    vec,
};

use axum::{
    Json,
    extract::{FromRequest, Request},
    response::IntoResponse,
};
use serde_json::{Value, json};

use crate::{
    claude_code_state::ClaudeCodeState,
    claude_web_state::{ClaudeApiFormat, ClaudeWebState},
    config::CLEWDR_CONFIG,
    error::ClewdrError,
    types::claude_message::{
        ContentBlock, CreateMessageParams, Message, MessageContent, Role, Usage,
    },
};

use super::to_oai;

/// A custom extractor that unifies different API formats
///
/// This extractor processes incoming requests, handling differences between
/// Claude and OpenAI API formats, and applies preprocessing to ensure consistent
/// handling throughout the application. It also detects and handles test messages
/// from client applications.
///
/// # Functionality
///
/// - Extracts and normalizes message parameters from different API formats
/// - Detects and processes "thinking mode" requests by modifying model names
/// - Identifies test messages and handles them appropriately
/// - Attempts to retrieve responses from cache before processing requests
/// - Provides format information via the FormatInfo extension
pub struct ClaudeWebPreprocess(pub CreateMessageParams, pub ClaudeWebContext);

/// Contains information about the API format and streaming status
///
/// This structure is passed through the request pipeline to inform
/// handlers and response processors about the API format being used
/// and whether the response should be streamed.
#[derive(Debug, Clone)]
pub struct ClaudeWebContext {
    /// Whether the response should be streamed
    pub stream: bool,
    /// The API format being used (Claude or OpenAI)
    pub api_format: ClaudeApiFormat,
    /// The stop sequence used for the request
    pub stop_sequences: Vec<String>,
    /// User information about input and output tokens
    pub usage: Usage,
}

/// Predefined test message in Claude format for connection testing
///
/// This is a standard test message sent by clients like SillyTavern
/// to verify connectivity. The system detects these messages and
/// responds with a predefined test response to confirm service availability.
static TEST_MESSAGE_CLAUDE: LazyLock<Message> = LazyLock::new(|| {
    Message::new_blocks(
        Role::User,
        vec![ContentBlock::Text {
            text: "Hi".to_string(),
        }],
    )
});

/// Predefined test message in OpenAI format for connection testing
static TEST_MESSAGE_OAI: LazyLock<Message> = LazyLock::new(|| Message::new_text(Role::User, "Hi"));

impl FromRequest<ClaudeWebState> for ClaudeWebPreprocess {
    type Rejection = ClewdrError;

    async fn from_request(req: Request, state: &ClaudeWebState) -> Result<Self, Self::Rejection> {
        let uri = req.uri().to_string();
        let Json(mut body) = Json::<CreateMessageParams>::from_request(req, &()).await?;

        // Handle thinking mode by modifying the model name
        if body.model.ends_with("-thinking") {
            body.model = body.model.trim_end_matches("-thinking").to_string();
            body.thinking = Some(Default::default());
        }
        if body.model.contains("sonnet-4") && !body.model.ends_with("-claude-ai") {
            // Special handling for Sonnet-4 models
            body.model = format!("{}-claude-ai", body.model);
        }

        // Check for test messages and respond appropriately
        if !body.stream.unwrap_or_default()
            && (body.messages == vec![TEST_MESSAGE_CLAUDE.to_owned()]
                || body.messages == vec![TEST_MESSAGE_OAI.to_owned()])
        {
            // Respond with a test message
            return Err(ClewdrError::TestMessage);
        }

        // Determine streaming status and API format
        let stream = body.stream.unwrap_or_default();
        let format = if uri.contains("chat/completions") {
            ClaudeApiFormat::OpenAI
        } else {
            ClaudeApiFormat::Claude
        };

        // Update state with format information
        let mut state = state.to_owned();
        state.api_format = format;
        state.stream = stream;
        let mut stop = body.stop_sequences.to_owned().unwrap_or_default();
        stop.extend_from_slice(body.stop.to_owned().unwrap_or_default().as_slice());
        stop.sort();
        stop.dedup();
        let input_tokens = body.count_tokens();
        let info = ClaudeWebContext {
            stream,
            api_format: format,
            stop_sequences: stop,
            usage: Usage {
                input_tokens,
                output_tokens: 0, // Placeholder for output token count
            },
        };

        // Try to retrieve from cache before processing
        if let Some(mut r) = state.try_from_cache(&body).await {
            r.extensions_mut().insert(info.to_owned());
            let r = to_oai(r).await.into_response();
            return Err(ClewdrError::CacheFound { res: Box::new(r) });
        }

        Ok(Self(body, info))
    }
}

#[derive(Debug, Clone)]
pub struct ClaudeCodeContext {
    /// Whether the response should be streamed
    pub stream: bool,
    /// The API format being used (Claude or OpenAI)
    pub api_format: ClaudeApiFormat,
    /// The hash of the system messages for caching purposes
    pub system_prompt_hash: Option<u64>,
    // Usage information for the request
    pub usage: Usage,
}

pub struct ClaudeCodePreprocess(pub CreateMessageParams, pub ClaudeCodeContext);

impl FromRequest<ClaudeCodeState> for ClaudeCodePreprocess {
    type Rejection = ClewdrError;

    async fn from_request(req: Request, _: &ClaudeCodeState) -> Result<Self, Self::Rejection> {
        let uri = req.uri().to_string();
        let Json(mut body) = Json::<CreateMessageParams>::from_request(req, &()).await?;

        // Handle thinking mode by modifying the model name

        if body.model.ends_with("-thinking") {
            body.model = body.model.trim_end_matches("-thinking").to_string();
            body.thinking = serde_json::from_value(json!({
                "budget_tokens": 1024,
                "type": "enabled",
            }))
            .ok();
        }
        body.model = body.model.trim_end_matches("-claude-ai").to_string();

        // Check for test messages and respond appropriately
        if !body.stream.unwrap_or_default()
            && (body.messages == vec![TEST_MESSAGE_CLAUDE.to_owned()]
                || body.messages == vec![TEST_MESSAGE_OAI.to_owned()])
        {
            // Respond with a test message
            return Err(ClewdrError::TestMessage);
        }

        // Determine streaming status and API format
        let stream = body.stream.unwrap_or_default();
        let format = if uri.contains("chat/completions") {
            body.stop_sequences = body.stop.take();
            // extract all system messages from the messages
            let (no_sys, sys) = body
                .messages
                .into_iter()
                .partition::<Vec<_>, _>(|m| m.role != Role::System);
            body.messages = no_sys;
            body.system = Some(
                sys.into_iter()
                    .flat_map(|m| match m.content {
                        MessageContent::Text { content: text } => {
                            vec![ContentBlock::Text { text }]
                        }
                        MessageContent::Blocks { content } => content,
                    })
                    .map(|b| json!(b))
                    .collect::<Vec<_>>()
                    .into(),
            );
            ClaudeApiFormat::OpenAI
        } else {
            ClaudeApiFormat::Claude
        };

        // Add a prelude text block to the system messages
        let prelude = ContentBlock::Text {
            text: CLEWDR_CONFIG
                .load()
                .custom_system
                .clone()
                .unwrap_or_else(|| {
                    "You are Claude Code, Anthropic's official CLI for Claude.".into()
                }),
        };
        let mut system = vec![json!(prelude)];
        match body.system {
            Some(Value::String(text)) => {
                let text_content = ContentBlock::Text { text };
                system.push(json!(text_content));
            }
            Some(Value::Array(a)) => system.extend(a),
            _ => {}
        }

        let cache_systems = system
            .iter_mut()
            .filter_map(|s| {
                // Claude Code does not allow TTLs in system prompts
                s["cache_control"].as_object_mut()?.remove("ttl");
                Some(&*s)
            })
            .collect::<Vec<_>>();
        let system_prompt_hash = (!cache_systems.is_empty()).then(|| {
            let mut hasher = DefaultHasher::new();
            cache_systems.hash(&mut hasher);
            hasher.finish()
        });

        body.system = Some(Value::Array(system));
        let input_tokens = body.count_tokens();

        let info = ClaudeCodeContext {
            stream,
            api_format: format,
            system_prompt_hash,
            usage: Usage {
                input_tokens,
                output_tokens: 0, // Placeholder for output token count
            },
        };

        Ok(Self(body, info))
    }
}
