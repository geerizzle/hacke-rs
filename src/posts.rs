use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: Option<u128>,
    pub deleted: Option<bool>,
    #[serde(rename = "type")]
    pub post_type: Option<String>,
    pub by: Option<String>,
    pub time: Option<u128>,
    pub text: Option<String>,
    pub dead: Option<bool>,
    pub parent: Option<u128>,
    pub poll: Option<u128>,
    pub kids: Option<Vec<u128>>,
    pub url: Option<String>,
    pub score: Option<u128>,
    pub title: Option<String>,
    pub parts: Option<Vec<u128>>,
    pub descendants: Option<u128>,
}
