use actix_web::Error;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize,Serialize};
use serde_json::{json, Value};
use structural::Structural;
use validator::Validate;
use bson::doc;

use crate::library::{constant, token};
// use bson::serde_helpers::{chrono_datetime_as_bson_datetime, bson_datetime_as_chrono_datetime};

// RAW MODELS
#[derive(Debug, Clone, Serialize, Deserialize, Default,Structural)]
pub struct BasicUserInfoSimple{
    pub id: String,
    pub profile_image: Option<String>,
    pub name: String,
    pub username: String,
    pub country: String,
    pub is_verified: bool,
    pub is_enabled: bool
}

#[derive(Debug, Clone, Serialize, Deserialize, Default,Structural)]
pub struct BasicUserInfo{
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub device_token: Option<String>, 
}

#[derive(Debug, Clone, Serialize, Deserialize, Default,Structural)]
pub struct NewLoginNotif {
    pub is_enabled: bool,
    pub last_login_device: Option<String>, // (should also be set on signup)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default,Structural)]
pub struct UserNotifications {
    pub notify_me_on_transactions: bool,
    pub notify_me_on_new_login: Option<NewLoginNotif>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default,Structural)]
pub struct TwoFA {
    pub is_enabled: bool,
    pub secret_key:  String, // (used for validating the supplied code from the user's 2FA app like google authenticator)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default,Structural)]
pub struct UserSecurity {
    pub two_fa: Option<TwoFA>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default,Structural)]
pub struct UserSettings {
    pub notifications: UserNotifications,
    pub security: UserSecurity,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default,Structural)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub profile_image: Option<String>,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub country: String,
    pub phone: Option<String>,
    pub dob: Option<String>,
    pub is_verified: bool,
    pub device_token: Option<String>,
    pub referral_code: String,
    pub referred_by: Option<String>,
    pub access_role: String, // (one of "user", "agent", "merchant", "expert", "admin" or "super_admin")
    pub is_enabled: bool, // (i.e can be used by the admin to temporarily disable this user, until kyc or other due diligence is completed)
    pub settings: UserSettings,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,
}

impl User{
    pub fn build_output(&self)->Result<UserOutput,Error>{
        let output_value = serde_json::to_value(&self)?;

        let mut user: UserOutput = serde_json::from_value(output_value)?;

        let id = ObjectId::to_string(&self._id.unwrap());
        user.id = id.clone();

        user.token = token::create_token(&id);

        // removing sensitive data in the user settings
        let mut notifications = self.clone().settings.notifications;
        if let Some(login_notif) = notifications.notify_me_on_new_login{
            notifications.notify_me_on_new_login = Some(NewLoginNotif{
                is_enabled: login_notif.is_enabled,
                last_login_device: None,
            });
        }

        let mut security = self.clone().settings.security;
        if let Some(two_fa) = security.two_fa.clone(){
            security.two_fa = Some(TwoFA{
                is_enabled: two_fa.is_enabled,
                secret_key: constant::EMPTY_STR.to_owned()
            });
        }

        user.settings = json!({
            "notifications": notifications.clone(),
            "security": security.clone(),
        });

        Ok(user)
    }
}

// END RAW MODELS





// INPUT MODELS

#[derive(Debug,Validate, Clone, Serialize, Deserialize,Default,Structural)]
pub struct UpdateUser{
    #[validate(length(min = 6, max = 100))]
    pub name: Option<String>,

    #[validate(length(min = 6, max = 200))]
    pub profile_image: Option<String>,

    #[validate(length(min = 6, max = 100))]
    pub username: Option<String>,

    #[validate(length(min = 6, max = 100))]
    pub country: Option<String>,

    pub phone: Option<String>,
    pub dob: Option<String>,

    pub device_token: Option<String>,

    pub settings: Option<UserSettings>,
}

#[derive(Debug,Validate, Clone, Serialize, Deserialize,Default,Structural)]
pub struct Verify2FA{
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 6, max = 20))]
    pub phone: Option<String>,
    #[validate(length(min = 6, max = 6))]
    pub code: String,
}

#[derive(Debug,Validate, Clone, Serialize, Deserialize,Default,Structural)]
pub struct UsernameCheck{
    #[validate(length(min = 3, max = 50))]
    pub username: String,
}

// END INPUT MODELS





// OUTPUT MODELS
#[derive(Debug, Clone, Serialize, Deserialize, Default,Structural)]
pub struct UserOutput {
    #[serde(default)]
    pub id: String,
    pub profile_image: Option<String>,
    pub name: String,
    pub username: String,
    pub email: Option<String>,
    pub country: String,
    pub phone: Option<String>,
    pub dob: Option<String>,
    pub referral_code: String,
    pub access_role: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub settings: Value
}


// END OUTPUT MODELS