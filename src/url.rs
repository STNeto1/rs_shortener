use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub short_url: String,
    pub url: String,
}

impl Url {
    pub fn from_raw_dynamo(data: &HashMap<String, AttributeValue>) -> Option<Self> {
        let (url, valid_url) = match data.get("url") {
            Some(url) => (parse_type_cuz_dynamo_sucks(url), true),
            None => (String::from(""), false)
        };
        let (short_url, valid_short_url) = match data.get("short_url") {
            Some(short_url) => (parse_type_cuz_dynamo_sucks(short_url), true),
            None => (String::from(""), false)
        };

        if !valid_url || !valid_short_url {
            return None;
        }

        return Some(Self {
            short_url,
            url,
        });
    }
}

fn parse_type_cuz_dynamo_sucks(data: &AttributeValue) -> String {
    return match data {
        AttributeValue::S(s) => s.to_owned(),
        _ => String::from("")
    };
}