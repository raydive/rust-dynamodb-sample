use std::collections::HashMap;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_config::config::Credentials;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, new, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Model {
    yourname: String,
    age: i32,
}

impl From<HashMap<String, aws_sdk_dynamodb::types::AttributeValue>> for Model {
    fn from(value: HashMap<String, aws_sdk_dynamodb::types::AttributeValue>) -> Self {
        let yourname = value.get("yourname").unwrap().as_s().unwrap().to_string();

        let age = match value.get("age") {
            Some(age) => age.as_n().unwrap().parse().unwrap(),
            None => 0,
        };


        Model::new(yourname, age)
    }
}

const DYNAMODB_ENDPOINT: &str = "http://localhost:8000";
const REGION: Region = Region::from_static("ap-northeast-1");

#[tokio::main]
async fn main() {
    let config = aws_config::from_env()
        .endpoint_url(DYNAMODB_ENDPOINT)
        .region(REGION)
        .credentials_provider(Credentials::new(
            "dummy",
            "dummy",
            None,
            None,
            "provider_name",
        ))
        .behavior_version(BehaviorVersion::latest())
        .load()
        .await;

    let client = aws_sdk_dynamodb::Client::new(&config);

    let response = client
        .get_item()
        .table_name("test")
        .projection_expression("yourname, age")
        .key(
            "yourname".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::S("bbbbb".to_string()),
        )
        .send()
        .await
        .unwrap();

    let model = response.item.unwrap();
    let model = Model::from(model);

    println!("{:?}", model);

    // こっちのデータはageがない想定
    let response = client
        .get_item()
        .table_name("test")
        .projection_expression("yourname, age")
        .key(
            "yourname".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::S("aaaaa".to_string()),
        )
        .send()
        .await
        .unwrap();

    let model = response.item.unwrap();
    let model = Model::from(model);

    println!("{:?}", model);
}
