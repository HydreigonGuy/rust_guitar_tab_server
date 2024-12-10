
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Tab {
    pub title: String,
    pub tab: Vec<Vec<u32>>,
}
