use serde::{Serialize, Deserialize};
use structural::Structural;
use validator::Validate;

// RAW MODELS

// END RAW MODELS


// INPUT MODELS

#[derive(Debug, Clone, Serialize,Validate, Deserialize, Default,Structural)]
pub struct IdInput {
    #[validate(length(min = 24, max = 24))]
    pub id: String,
}

#[derive(Debug, Clone, Serialize,Validate, Deserialize, Default,Structural)]
pub struct PageInput {
    pub page_number: i32,
}


// END INPUT MODELS



// OUTPUT MODELS
#[derive(Debug, Clone, Serialize, Deserialize,Default,Structural)]
#[serde(default)]
pub struct PageResult<T: Send + Sync> {
    pub per_page: i32,
    pub page_number: i32, 
    pub total_pages: i32,
    pub result: Vec<T>
}


// END OUTPUT MODELS