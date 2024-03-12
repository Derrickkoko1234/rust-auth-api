// work on FCM here and add it to appinstance

use std::{sync::Arc, env};
use actix_web::error::{self,Error};
use fcm_rust::{FirebaseCloudMessaging, Message, SendOptions, AndroidOptions, WebPushOptions};
use log::{info,error};
use crate::library::{constant, file};

use serde::{Deserialize,Serialize};

#[derive(Default,Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FCMPayload{
    pub take_to: String, // where in the app to take the user to. e.g "event_details", "my_profile"
    pub action_id: Option<String>, // id of the action to open in the app from the notification
    pub user_id: Option<String> // user id of the user to open their profile
}

#[derive(Default,Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FCMMessage{
    pub device_tokens: Vec<String>,
    pub image: Option<String>,
    pub title: String,
    pub body: String,
    pub payload: FCMPayload
}

#[derive(Clone)]
pub struct FCM{
    pub fcm: Arc<FirebaseCloudMessaging>
}

impl FCM {
    pub async fn init()-> Result<Self,Error>{
        // download the fcm data and store in specified path
        // if files does not exist, proceed to downloading them
        if !file::file_exists(constant::CRYPTO_FCM){
            // download fcm file
            let ks_file_url = env::var("FCM_FILE").expect("FCM_FILE not found");
            let ks_download = file::download_file(&ks_file_url,constant::CRYPTO_FCM);
        
            if ks_download.is_err() || !ks_download.as_ref().unwrap(){
                return Err(error::ErrorInternalServerError(ks_download.err().unwrap()));
            }
        }

        let fcm = FirebaseCloudMessaging::from_credential_path(constant::CRYPTO_FCM);
        Ok(Self{
            fcm: Arc::new(fcm)
        })
    }

    pub async fn fcm_send(&self,fcm_message: &FCMMessage) -> Result<bool,Error>{
        let tokens = fcm_message.device_tokens.clone();
        let mut img = constant::EMPTY_STR.to_owned();
        if let Some(image) = fcm_message.image.clone(){
            img = image;
        }
        let message = Message::new(img,fcm_message.title.clone(), fcm_message.body.clone());
        let data = serde_json::json!(fcm_message.payload);
        let apns = SendOptions{
            content_available: None,
            mutable_content: Some(true),
            priority: None,
            image: fcm_message.image.clone(),
        };
        let android = AndroidOptions { image: fcm_message.image.clone() };
        let webpush = WebPushOptions { image: fcm_message.image.clone() };
        let res = self.fcm.send_to_devices(tokens.clone(), message, apns, android,webpush, Some(data)).await;
        println!("hre..6..{:?}...token{:?}",res,tokens);
        match res {
            Ok(r)=>{
                info!("FCM.....success...............{:?}",r);
            },
            Err(u)=>{
                error!("FCM.....failure...............{:?}",u);
            }
        }

        Ok(true)
    }

    pub async fn init_fcm_send(&self,device_tokens: Vec<String>,image: Option<String>,title: String,body: String,payload: FCMPayload)->Result<bool,Error>{
        let fcm_message = FCMMessage{
            device_tokens,
            image,
            title,
            body,
            payload
        };
        let res = self.fcm_send(&fcm_message).await?;
        Ok(res)
    }
}