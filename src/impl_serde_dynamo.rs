use aws_sdk_dynamodb::{model::AttributeValue, Client};
use serde_dynamo::from_items;

use crate::{Profile, ProfileError};

pub async fn get_via_serde_dynamo() -> Result<(), ProfileError> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let profiles = get_profiles(&client, "janellatable", "abc").await?;
    println!("Current DynamoDB values: {:?}", profiles);
    Ok(())
}
async fn get_profiles(
    client: &Client,
    table_name: &str,
    account_id: &str,
) -> Result<Vec<Profile>, ProfileError> {
    let results = client
        .query()
        .table_name(table_name)
        .key_condition_expression("pk = :acct_id")
        .expression_attribute_values(
            ":acct_id",
            AttributeValue::S(format!("acct#{}#v1", account_id)),
        )
        .expression_attribute_names("#name", "name")
        .projection_expression("#name")
        .send()
        .await?;

    if let Some(items) = results.items {
        let items: Result<Vec<Profile>, serde_dynamo::Error> = from_items(items);
        match items {
            Ok(items) => return Ok(items),
            Err(err) => return Err(ProfileError::FromSerde(err)),
        }
    }
    Ok(vec![])
}
