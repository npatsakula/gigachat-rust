use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCheckDiagnostics {
    description: String,
    schema_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionExample<I> {
    pub request: String,
    pub params: I,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserFunction {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) parameters: serde_json::Value,
    pub(crate) few_shot_examples: Vec<serde_json::Value>,
    pub(crate) return_parameters: serde_json::Value,
}

impl UserFunction {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn function_name(&self) -> FunctionName {
        FunctionName::new(self.name.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionName {
    name: String,
}

impl FunctionName {
    fn new<N: Into<String>>(name: N) -> Self {
        Self { name: name.into() }
    }

    pub fn text2image() -> Self {
        Self::new("text2image")
    }

    pub fn get_file_content() -> Self {
        Self::new("get_file_content")
    }

    pub fn text2model3d() -> Self {
        Self::new("text2model3d")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionCheckResult {
    Error {
        errors: Vec<FunctionCheckDiagnostics>,
    },
    Ok {
        #[serde(default)]
        warnings: Vec<FunctionCheckDiagnostics>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCheckResponse {
    pub(crate) status: u16,
    pub(crate) message: String,
    pub(crate) json_ai_rules_version: String,
    #[serde(flatten)]
    pub(crate) result: FunctionCheckResult,
}
