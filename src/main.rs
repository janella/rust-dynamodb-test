use std::collections::HashMap;

use aws_sdk_dynamodb::{model::AttributeValue, types::SdkError, Client, Error};
use impl_serde_dynamo::get_via_serde_dynamo;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::impl_raw_sdk::get_via_sdk;

mod impl_raw_sdk;
mod impl_serde_dynamo;

#[tokio::main]
async fn main() -> Result<(), ProfileError> {
    let shared_config = aws_config::load_from_env().await;
    get_via_sdk().await?;
    get_via_serde_dynamo().await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    name: String,
}

impl Profile {
    fn new(name: String) -> Self {
        Self { name }
    }
}

fn as_string(val: Option<&AttributeValue>, default: &String) -> String {
    if let Some(v) = val {
        if let Ok(s) = v.as_s() {
            return s.to_owned();
        }
    }
    default.to_owned()
}
impl From<&HashMap<String, AttributeValue>> for Profile {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        Profile::new(as_string(value.get("name"), &"".to_string()))
    }
}

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("failed to parse serde_json::Value into Profile {0}")]
    FromValue(&'static Value),

    #[error("failed to parse response into Profiles: {0}")]
    FromSerde(serde_dynamo::Error),

    #[error("aws_sdk_dynamodb error: {0}")]
    Dynamo(aws_sdk_dynamodb::Error),

    #[error("unknown DynamoDB Profiles error: {0}")]
    Unknown(String),
}

impl From<aws_sdk_dynamodb::Error> for ProfileError {
    fn from(err: aws_sdk_dynamodb::Error) -> Self {
        ProfileError::Dynamo(err)
    }
}

impl From<serde_dynamo::Error> for ProfileError {
    fn from(err: serde_dynamo::Error) -> Self {
        ProfileError::FromSerde(err)
    }
}

impl<E> From<SdkError<E>> for ProfileError
where
    E: std::fmt::Debug,
{
    fn from(err: SdkError<E>) -> Self {
        ProfileError::Unknown(format!("{err:?}"))
    }
}
