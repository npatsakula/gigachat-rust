pub mod error;
pub mod structures;

use std::marker::PhantomData;

use schemars::{JsonSchema, generate::SchemaSettings};
use serde::Serialize;
use snafu::ResultExt;

use crate::{
    client::GigaChatClient,
    function::{
        error::BadFunctionSnafu,
        structures::{FunctionCheckDiagnostics, FunctionCheckResult},
    },
};
pub use structures::{FunctionCheckResponse, FunctionExample, FunctionName, UserFunction};

pub trait SchemaGenerator {
    fn generate() -> schemars::SchemaGenerator;
}

pub struct SberSchema;

impl SchemaGenerator for SberSchema {
    fn generate() -> schemars::SchemaGenerator {
        let mut settings = SchemaSettings::openapi3();
        settings.inline_subschemas = true;
        settings.meta_schema = None;
        schemars::SchemaGenerator::from(settings)
    }
}

pub struct FunctionBuilder<I, O, S = SberSchema> {
    name: String,
    description: Option<String>,
    arguments: PhantomData<I>,
    output: PhantomData<O>,
    examples: Vec<FunctionExample<I>>,
    schema_generator: PhantomData<S>,
}

pub trait FunctionExt {
    type Arguments;
    type Output;
}

impl<I, O, S> FunctionExt for FunctionBuilder<I, O, S> {
    type Arguments = I;
    type Output = O;
}

impl<I, O, S> FunctionBuilder<I, O, S> {
    pub fn new<N: Into<String>>(name: N) -> Self {
        FunctionBuilder {
            name: name.into(),
            description: None,
            arguments: PhantomData,
            output: PhantomData,
            examples: Vec::new(),
            schema_generator: PhantomData,
        }
    }

    pub fn with_description<D: Into<String>>(mut self, description: D) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_example(mut self, example: FunctionExample<I>) -> Self {
        self.examples.push(example);
        self
    }
}

impl<I: JsonSchema + Serialize, O: JsonSchema, S: SchemaGenerator> FunctionBuilder<I, O, S> {
    pub fn build(self) -> UserFunction {
        let schema = S::generate();
        let parameters = schema.clone().into_root_schema_for::<I>().to_value();
        let return_parameters = schema.into_root_schema_for::<O>().to_value();
        UserFunction {
            name: self.name,
            description: self.description,
            parameters,
            few_shot_examples: self
                .examples
                .into_iter()
                .map(|e| serde_json::to_value(e).unwrap())
                .collect(),
            return_parameters,
        }
    }
}

impl GigaChatClient {
    pub async fn check_function(
        &self,
        function: &UserFunction,
    ) -> Result<Vec<FunctionCheckDiagnostics>, error::Error> {
        let url = self.build_url("functions/validate", None).unwrap();
        let response = self
            .perform_request(
                |c| c.post(url).json(function),
                async |r| r.json::<FunctionCheckResponse>().await,
            )
            .await
            .context(error::BadRequestSnafu)?;

        match response.result {
            FunctionCheckResult::Error { errors } => BadFunctionSnafu { errors }.fail(),
            FunctionCheckResult::Ok { warnings } => Ok(warnings),
        }
    }
}
