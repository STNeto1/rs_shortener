use anyhow::{anyhow, bail, Error, Result};
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::QueryOutput;

use crate::url::Url;

mod url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let basic_url = Url {
        url: String::from("https://github.com/stneto1"),
        short_url: String::from("another_short_url"),
    };

    //store_item(&basic_url, &client).await?;
    let item = query_item(&basic_url.short_url, &client).await?;
    println!("{:?}", item);

    return Ok(());
}

async fn query_item(short_url: &String, client: &Client) -> Result<Url, Error> {
    if let Ok(item) = client
        .get_item()
        .table_name("Url_Shortener")
        .key("short_url", AttributeValue::S(short_url.to_owned()))
        .send()
        .await {
        if let Some(raw_url) = item.item() {
            if let Some(url) = Url::from_raw_dynamo(&raw_url) {
                return Ok(url);
            }

            bail!("some error parsing");
        } else {
            bail!("some error");
        }
    }
    bail!("failed to get item");
}

async fn store_item(payload: &Url, client: &Client) -> Result<(), Error> {
    client
        .put_item()
        .table_name("Url_Shortener")
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
