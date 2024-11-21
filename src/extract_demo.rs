use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::untils::create_client_from_env;

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct MovieReview {
    sentiment: String,
    rating: f32,
}

#[tokio::test]
async fn test_extract() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let client = create_client_from_env();
    let extractor = client.extractor::<MovieReview>("gpt-3.5-turbo").build();
    let review = extractor.extract("I loved this movie! It's a solid 9/10.").await?;
    println!("Extracted: {:?}", review);
    Ok(())
}


#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct PLan {
    action: Action,
    obj: Vec<Obj>
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
enum Action {
    #[serde(rename = "pick up")]
    PickUp,
    #[serde(rename = "unstack")]
    Unstack,
    #[serde(rename = "put down")]
    PutDown,
    #[serde(rename = "stack")]
    Stack
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
enum Obj {
    #[serde(rename = "yellow block")]
    YellowBlock,
    #[serde(rename = "orange block")]
    OrangeBlock,
    #[serde(rename = "red block")]
    RedBlock
}
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct Plans{
    plans: Vec<PLan>,
}

#[tokio::test]
async fn test_extract_plan() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let client = create_client_from_env();
    let extractor = client.extractor::<Plans>("gpt-4").build();
    let res = "pick up the yellow block\n
                    unstack the yellow block from on top of the orange block\n
                    put down the yellow block\n
                    pick up the orange block\n
                    stack the orange block on top of the red block\n
                    [PLAN END]";
    let review = extractor.extract(res).await?;
    println!("Extracted: {:?}", review);
    Ok(())
}