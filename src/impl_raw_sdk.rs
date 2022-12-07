use crate::{Profile, ProfileError};
use aws_sdk_dynamodb::{model::AttributeValue, Client};

pub async fn get_via_sdk() -> Result<(), ProfileError> {
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
        .send()
        .await?;

    if let Some(items) = results.items {
        let profiles = items.iter().map(|v| v.into()).collect();
        Ok(profiles)
    } else {
        Ok(vec![])
    }
}
