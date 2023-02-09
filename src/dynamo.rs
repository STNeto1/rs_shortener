use anyhow::anyhow;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::url::Url;
use crate::AppError;

const TABLE_NAME: &str = "Url_Shortener";

pub async fn query_item(short_url: &String, client: &Client) -> Result<Url, AppError> {
    if let Ok(item) = client
        .get_item()
        .table_name(TABLE_NAME.to_owned())
        .key("short_url", AttributeValue::S(short_url.to_owned()))
        .send()
        .await
    {
        if let Some(raw_url) = item.item() {
            if let Some(url) = Url::from_raw_dynamo(&raw_url) {
                return Ok(url);
            }

            return Err(AppError(anyhow!("some error parsing")));
        } else {
            return Err(AppError(anyhow!("some error")));
        }
    }

    return Err(AppError(anyhow!("failed to get item")));
}

pub async fn store_item(payload: &Url, client: &Client) -> Result<(), AppError> {
    client
        .put_item()
        .table_name(TABLE_NAME.to_owned())
        .item(
            "short_url",
            aws_sdk_dynamodb::model::AttributeValue::S(String::from(payload.short_url.as_str())),
        )
        .item(
            "url",
            aws_sdk_dynamodb::model::AttributeValue::S(String::from(payload.url.as_str())),
        )
        .send()
        .await
        .map_err(|e| anyhow!("{}", e))?;

    return Ok(());
}
