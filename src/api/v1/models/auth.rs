use serde::{Deserialize,Serialize};
use structural::Structural;
use validator::Validate; // https://crates.io/crates/validator


// INPUT MODELS

#[derive(Debug, Clone, Serialize,Validate, Deserialize, Default,Structural)]
pub struct SignUp {
    #[validate(length(min = 6, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 20))]
    pub referrer_code: Option<String>,
    #[validate(length(min = 6, max = 50))]
    pub password: String,

    #[validate(length(min = 6, max = 100))]
    pub country: String,

    #[validate(length(min = 6, max = 200))]
    pub device_token: Option<String>,

    #[validate(length(min = 3, max = 500))]
    pub device_info: String,
}

#[derive(Debug,Validate, Clone, Serialize, Deserialize,Default,Structural)]
pub struct SignIn{
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 50))]
    pub password: String,

    #[validate(length(min = 3, max = 500))]
    pub device_info: String,
}

#[derive(Debug,Validate, Clone, Serialize, Deserialize,Default,Structural)]
pub struct VerifyAccount{
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 20))]
    pub code: String,
}


#[derive(Debug,Validate, Clone, Serialize, Deserialize,Default,Structural)]
pub struct InitRecovery{
    #[validate(email)]
    pub email: String,
}


#[derive(Debug,Validate, Clone, Serialize, Deserialize,Default,Structural)]
pub struct ChangePassword{
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 6))]
    pub code: String,
    #[validate(length(min = 6, max = 50))]
    pub password: String,
}

// END INPUT MODELS




// OUTPUT MODELS


// END OUTPUT MODELS