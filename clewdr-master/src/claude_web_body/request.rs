use base64::{Engine, prelude::BASE64_STANDARD};
use futures::{StreamExt, stream};
use itertools::Itertools;
use rand::{Rng, rng};
use serde_json::Value;
use std::{fmt::Write, mem};
use tracing::warn;
use wreq::{
    Method,
    multipart::{Form, Part},
};

use crate::{
    claude_web_body::{Attachment, RequestBody, Tool},
    claude_web_state::{ClaudeApiFormat, ClaudeWebState},
    config::CLEWDR_CONFIG,
    types::claude_message::{
        ContentBlock, CreateMessageParams, ImageSource, Message, MessageContent, Role,
    },
    utils::{TIME_ZONE, print_out_text},
};

/// Merged messages and images
#[derive(Default, Debug)]
struct Merged {
    pub paste: String,
    pub prompt: String,
    pub images: Vec<ImageSource>,
}

impl ClaudeWebState {
    pub fn transform_request(&self, mut value: CreateMessageParams) -> Option<RequestBody> {
        let (value, merged) = match self.api_format {
            ClaudeApiFormat::Claude => {
                let system = value.system.take();
                let msgs = mem::take(&mut value.messages);
                let system = merge_system(system.unwrap_or_default());
                let merged = merge_messages(msgs, system)?;
                (value, merged)
            }
            ClaudeApiFormat::OpenAI => {
                let mut msgs = mem::take(&mut value.messages);
                let mut role = msgs.first().map(|m| m.role)?;
                for msg in msgs.iter_mut() {
                    if msg.role != Role::System {
                        role = msg.role;
                    } else {
                        msg.role = role;
                    }
                }
                let merged = merge_messages(msgs, String::new())?;
                (value, merged)
            }
        };
        let mut tools = vec![];
        if CLEWDR_CONFIG.load().web_search {
            tools.push(Tool::web_search());
        }
        Some(RequestBody {
            max_tokens_to_sample: value.max_tokens,
            attachments: vec![Attachment::new(merged.paste)],
            files: vec![],
            model: if self.is_pro() {
                Some(value.model)
            } else {
                None
            },
            rendering_mode: if value.stream.unwrap_or_default() {
                "messages".to_string()
            } else {
                "raw".to_string()
            },
            prompt: merged.prompt,
            timezone: TIME_ZONE.to_string(),
            images: merged.images,
            tools,
        })
    }

    /// Upload images to the Claude.ai
    pub async fn upload_images(&self, imgs: Vec<ImageSource>) -> Vec<String> {
        // upload images
        stream::iter(imgs)
            .filter_map(async |img| {
                // check if the image is base64
                if img.type_ != "base64" {
                    warn!("Image type is not base64");
                    return None;
                }
                // decode the image
                let bytes = BASE64_STANDARD
                    .decode(img.data)
                    .inspect_err(|e| {
                        warn!("Failed to decode image: {}", e);
                    })
                    .ok()?;
                // choose the file name based on the media type
                let file_name = match img.media_type.to_lowercase().as_str() {
                    "image/png" => "image.png",
                    "image/jpeg" => "image.jpg",
                    "image/jpg" => "image.jpg",
                    "image/gif" => "image.gif",
                    "image/webp" => "image.webp",
                    "application/pdf" => "document.pdf",
                    _ => "file",
                };
                // create the part and form
                let part = Part::bytes(bytes).file_name(file_name);
                let form = Form::new().part("file", part);
                let endpoint = format!("{}/api/{}/upload", self.endpoint, self.org_uuid.as_ref()?);
                // send the request into future
                let res = self
                    .build_request(Method::POST, endpoint)
                    .multipart(form)
                    .send()
                    .await
                    .inspect_err(|e| {
                        warn!("Failed to upload image: {}", e);
                    })
                    .ok()?;
                #[derive(serde::Deserialize)]
                struct UploadResponse {
                    file_uuid: String,
                }
                // get the response json
                let json = res
                    .json::<UploadResponse>()
                    .await
                    .inspect_err(|e| {
                        warn!("Failed to parse image response: {}", e);
                    })
                    .ok()?;
                // extract the file_uuid
                Some(json.file_uuid)
            })
            .collect::<Vec<_>>()
            .await
    }
}

/// Merges multiple messages into a single text prompt, handling system instructions
/// and extracting any images from the messages
///
/// # Arguments
/// * `msgs` - Vector of messages to merge
/// * `system` - System instructions to prepend
///
/// # Returns
/// * `Option<Merged>` - Merged prompt text, images, and additional metadata, or None if merging fails
fn merge_messages(msgs: Vec<Message>, system: String) -> Option<Merged> {
    if msgs.is_empty() {
        return None;
    }
    let h = CLEWDR_CONFIG
        .load()
        .custom_h
        .to_owned()
        .unwrap_or("Human".to_string());
    let a = CLEWDR_CONFIG
        .load()
        .custom_a
        .to_owned()
        .unwrap_or("Assistant".to_string());

    let user_real_roles = CLEWDR_CONFIG.load().use_real_roles;
    let line_breaks = if user_real_roles { "\n\n\x08" } else { "\n\n" };
    let system = system.trim().to_string();
    let size = size_of_val(&msgs);
    // preallocate string to avoid reallocations
    let mut w = String::with_capacity(size);
    // generate padding text
    if !CLEWDR_CONFIG.load().pad_tokens.is_empty() {
        let len = CLEWDR_CONFIG.load().padtxt_len;
        let padding = generate_padding(len);
        w.push_str(padding.as_str());
    }

    let mut imgs: Vec<ImageSource> = vec![];

    let chunks = msgs
        .into_iter()
        .filter_map(|m| match m.content {
            MessageContent::Blocks { content } => {
                // collect all text blocks, join them with new line
                let blocks = content
                    .into_iter()
                    .filter_map(|b| match b {
                        ContentBlock::Text { text } => Some(text.trim().to_string()),
                        ContentBlock::Image { source } => {
                            // push image to the list
                            imgs.push(source);
                            None
                        }
                        ContentBlock::ImageUrl { image_url } => {
                            // oai image
                            if let Some(source) = extract_image_from_url(&image_url.url) {
                                imgs.push(source);
                            }
                            None
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                if blocks.is_empty() {
                    None
                } else {
                    Some((m.role, blocks))
                }
            }
            MessageContent::Text { content } => {
                // plain text
                let content = content.trim().to_string();
                if content.is_empty() {
                    None
                } else {
                    Some((m.role, content))
                }
            }
        })
        // chunk by role
        .chunk_by(|m| m.0);
    // join same role with new line
    let mut msgs = chunks.into_iter().map(|(role, grp)| {
        let txt = grp.into_iter().map(|m| m.1).collect::<Vec<_>>().join("\n");
        (role, txt)
    });
    // first message does not need prefix
    if !system.is_empty() {
        w += system.as_str();
    } else {
        let first = msgs.next()?;
        w += first.1.as_str();
    }
    for (role, text) in msgs {
        let prefix = match role {
            Role::System => {
                warn!("System message should be merged into the first message");
                continue;
            }
            Role::User => format!("{h}: "),
            Role::Assistant => format!("{a}: "),
        };
        write!(w, "{line_breaks}{prefix}{text}").ok()?;
    }
    print_out_text(w.to_owned(), "paste.txt");

    // prompt polyfill
    let p = CLEWDR_CONFIG.load().custom_prompt.to_owned();

    Some(Merged {
        paste: w,
        prompt: p,
        images: imgs,
    })
}

/// Generates random padding text of specified length
/// Used to pad prompts with tokens to meet minimum length requirements
///
/// # Arguments
/// * `length` - The target length of padding in tokens
///
/// # Returns
/// A string containing the padding text
fn generate_padding(length: usize) -> String {
    if length == 0 {
        return String::new();
    }
    let conf = CLEWDR_CONFIG.load();
    let tokens = conf
        .pad_tokens
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>();
    assert!(tokens.len() >= length, "Padding tokens too short");

    let mut result = String::with_capacity(length * 8);
    let mut rng = rng();
    let mut pushed = 0;
    loop {
        let slice_len = rng.random_range(16..64);
        let slice_start = rng.random_range(0..tokens.len() - slice_len);
        let slice = &tokens[slice_start..slice_start + slice_len];
        result.push_str(&slice.concat());
        pushed += slice_len;
        result.push('\n');
        if rng.random_range(0..100) < 5 {
            result.push('\n');
        }
        if pushed > length {
            break;
        }
    }
    print_out_text(result.to_owned(), "padding.txt");
    result.push_str("\n\n\n\n------------------------------------------------------------\n");
    result
}

/// Merges system message content into a single string
/// Handles both string and array formats for system messages
///
/// # Arguments
/// * `sys` - System message content as a JSON Value
///
/// # Returns
/// Merged system message as a string
fn merge_system(sys: Value) -> String {
    match sys {
        Value::String(s) => s,
        Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v["text"].as_str())
            .map(|v| v.trim())
            .collect::<Vec<_>>()
            .join("\n"),
        _ => String::new(),
    }
}

fn extract_image_from_url(url: &str) -> Option<ImageSource> {
    if !url.starts_with("data:") {
        return None; // only support data URI
    }
    let (metadata, base64_data) = url.split_once(',')?;

    let (media_type, type_) = metadata.strip_prefix("data:")?.split_once(';')?;

    Some(ImageSource {
        type_: type_.to_string(),
        media_type: media_type.to_string(),
        data: base64_data.to_owned(),
    })
}
