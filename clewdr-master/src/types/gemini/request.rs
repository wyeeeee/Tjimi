use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Default)]
#[allow(non_camel_case_types)]
pub enum Role {
    #[default]
    user,
    model,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct InlineData {
    mime_type: String,
    data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum Language {
    ///Unspecified language. This value should not be used.
    LANGUAGE_UNSPECIFIED,
    ///Python >= 3.10, with numpy and simpy available.
    PYTHON,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct ExecutableCode {
    language: Language,
    code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct FunctionCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct FunctionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    response: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
#[allow(non_snake_case)]
pub struct FileData {
    #[serde(skip_serializing_if = "Option::is_none")]
    mimeType: Option<String>,
    fileUrl: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum Outcome {
    /// Unspecified status. This value should not be used.
    OUTCOME_UNSPECIFIED,
    /// Code execution completed successfully.
    OUTCOME_OK,
    /// Code execution finished but with a failure. `stderr` should contain the reason.
    OUTCOME_FAILED,
    /// Code execution ran for too long, and was cancelled.
    /// There may or may not be a partial output present.
    OUTCOME_DEADLINE_EXCEEDED,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct CodeExecuteResult {
    outcome: Outcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum Part {
    #[serde(alias = "inlineData")]
    inline_data(InlineData),
    #[serde(alias = "executableCode")]
    executable_code(ExecutableCode),
    #[serde(alias = "codeExecutionResult")]
    code_execution_result(CodeExecuteResult),
    functionCall(FunctionCall),
    functionResponse(FunctionResponse),
    fileData(FileData),
    #[serde(untagged)]
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thought: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Chat {
    #[serde(default)]
    role: Role,
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct SystemInstruction {
    parts: Vec<Part>,
}
impl SystemInstruction {
    pub fn from_string(prompt: impl Into<String>) -> Self {
        Self {
            parts: vec![Part::Text {
                text: prompt.into(),
                thought: None,
            }],
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeminiRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<SystemInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    pub contents: Vec<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Value>,
}

impl GeminiRequestBody {
    pub fn safety_off(&mut self) {
        self.safety_settings = Some(json!([
          { "category": "HARM_CATEGORY_HARASSMENT", "threshold": "OFF" },
          { "category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "OFF" },
          { "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", "threshold": "OFF" },
          { "category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "OFF" },
          {
            "category": "HARM_CATEGORY_CIVIC_INTEGRITY",
            "threshold": "BLOCK_NONE"
          }
        ]));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
#[allow(non_camel_case_types)]
pub enum Tool {
    /// Generally it can be `Tool::google_search(json!({}))`
    google_search(Value),
    /// It is of form `Tool::function_calling(`[functionDeclaration](https://ai.google.dev/gemini-api/docs/function-calling?example=meeting)`)`
    functionDeclarations(Vec<Value>),
    /// Generally it can be `Tool::code_execution(json!({}))`,
    code_execution(Value),
}
