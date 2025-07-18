use crate::{
    claude_code_state::ClaudeCodeState,
    claude_web_state::ClaudeApiFormat,
    error::ClewdrError,
    middleware::claude::{ClaudeCodeContext, ClaudeCodePreprocess},
    utils::{enabled, print_out_json},
};
use axum::{Extension, extract::State, response::Response};
use colored::Colorize;
use tracing::info;

pub async fn api_claude_code(
    State(mut state): State<ClaudeCodeState>,
    ClaudeCodePreprocess(p, f): ClaudeCodePreprocess,
) -> (Extension<ClaudeCodeContext>, Result<Response, ClewdrError>) {
    state.system_prompt_hash = f.system_prompt_hash;
    state.stream = p.stream.unwrap_or_default();
    state.api_format = f.api_format;
    state.usage = f.usage.to_owned();
    print_out_json(&p, "client_req.json");
    let format_display = match f.api_format {
        ClaudeApiFormat::Claude => f.api_format.to_string().green(),
        ClaudeApiFormat::OpenAI => f.api_format.to_string().yellow(),
    };
    info!(
        "[REQ] stream: {}, msgs: {}, model: {}, think: {}, format: {}",
        enabled(state.stream),
        p.messages.len().to_string().green(),
        p.model.green(),
        enabled(p.thinking.is_some()),
        format_display
    );
    let stopwatch = chrono::Utc::now();
    let res = state.try_chat(p).await;

    let elapsed = chrono::Utc::now().signed_duration_since(stopwatch);
    info!(
        "[FIN] elapsed: {}s",
        format!("{}", elapsed.num_milliseconds() as f64 / 1000.0).green()
    );

    (Extension(f), res)
}
