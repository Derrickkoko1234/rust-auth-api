// schemas for library files

use serde::{Serialize,Deserialize};
use structural::Structural;

use super::constant;

#[derive(Debug, Clone, Serialize, Deserialize,Default,Structural)]
pub struct Attachment {
    pub file_type: String,
    pub file_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,Default,Structural)]
pub struct Recipient {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,Default,Structural)]
pub struct LowMailInput {
    pub integration: String,
    pub subject: String,
    pub message: String,
    pub recipients: Vec<Recipient>,
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize,Default,Structural)]
pub struct EmailConfig {
    pub service: String,
    pub low_mail_input: LowMailInput
}

impl EmailConfig {
    pub fn default()->Self{
        EmailConfig{
            service: constant::LOW_MAIL.to_owned(),
            low_mail_input: LowMailInput{
                integration: constant::ANY.to_owned(),
                subject: String::new(),
                message: String::new(),
                recipients: vec![],
                attachments: None,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize,Default,Structural)]
pub struct TemplateBuilder{
    pub template: String,
    pub replacers: Vec<(String,String)>
}



#[derive(Debug, Clone, Serialize, Deserialize,Default,Structural)]
pub struct LowSMSlInput {
    pub integration: String,
    pub message: String,
    pub recipients: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize,Default,Structural)]
pub struct SMSConfig {
    pub service: String,
    pub low_sms_input: LowSMSlInput
}

impl SMSConfig {
    pub fn new(message: String,recipients: Vec<String>)->Self{
        SMSConfig{
            service: constant::LOW_SMS.to_owned(),
            low_sms_input: LowSMSlInput{
                integration: constant::ANY.to_owned(),
                message,
                recipients,
            },
        }
    }
}