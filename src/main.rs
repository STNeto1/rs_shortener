use anyhow::{anyhow, Error, Result};
use aws_sdk_dynamodb::Client;

mod url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let resp = client
        .put_item()
        .table_name("Url_Shortener")
        .item(
            "short_url",
            aws_sdk_dynamodb::model::AttributeValue::S(String::from("some-short-url")),
        )
        .item(
            "url",
            aws_sdk_dynamodb::model::AttributeValue::S(String::from("https://github.com/stneto1")),
        )
        .send()
        .await
        .map_err(|e| anyhow!("{}", e))?;

    println!("{:?}", resp);

    return Ok(());
}
