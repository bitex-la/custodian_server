use jsonapi::model::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub id: Option<String>,
    pub height: u64,
    pub timestamp: u32,
}

jsonapi_model!(Block; "block");
