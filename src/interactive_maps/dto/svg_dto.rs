use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SvgInfo {
    pub prefix: String,
    pub name: String,
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SvgRequest {
    pub name: String,
    pub content: String,
}