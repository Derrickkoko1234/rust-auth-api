use actix_web::{error::HttpError, web, Error, HttpMessage, HttpRequest, HttpResponse};
use bson::{doc, oid::ObjectId, Bson, Document};
use futures_util::TryStreamExt;
use serde_json::json;
use tokio::runtime;
use validator::Validate;
use crate::{
    api::v1::{middlewares::middleware::Headers, models::user::{ BasicUserInfo, BasicUserInfoSimple, NewLoginNotif, TwoFA, UpdateUser, User, UserOutput, UsernameCheck, Verify2FA }, responses::{response::{self}, self}}, app::{mongo::AppInstance, redis::Redis}, init::AppState, library::{constant, email, schema::{EmailConfig, Recipient, TemplateBuilder}, time, two_fa}
};
static COLLECTION_NAME: &str = constant::USERS_COLLECTION;

// checks whether a user_id is admin or not
pub async fn is_admin(user_id: &str, app: &AppInstance)->Result<bool,Error>{
    
    let id = ObjectId::parse_str(&user_id).unwrap();
    let filter = doc!{
        "_id": id,
    };

    let result = app.get_one_item::<User>(&filter,COLLECTION_NAME).await?;

    // check if user is admin
    if let Some(user) = result{
        if constant::ADMIN_ROLES.contains(&user.access_role.as_str()){
            return Ok(true);
        }
    }
    
    Ok(false)
}

// checks whether a certain username is available or not
pub async fn check_username_availability(_req:HttpRequest,data: web::Data<AppState>,path: web::Path<UsernameCheck>)->Result<HttpResponse,Error>{
    let app = data.app.clone();

    // validate payload input data
    let result = path.validate();
    if let Err(e) = result {
        return Ok(HttpResponse::InternalServerError().json(response::get_validation_err_msg(e)));
    }

    let filter = doc!{
        "username": path.username.clone(),
    };

    let result = app.get_one_item::<User>(&filter,COLLECTION_NAME).await?;

    // username is not available
    if result.is_some(){
        return Ok(HttpResponse::Ok().json(response::get_success_msg(json!({"available": false}))));
    }
    
    // username is available
    Ok(HttpResponse::Ok().json(response::get_success_msg(json!({"available": true}))))
}

// refreshes user's token
pub async fn refresh_user(req:HttpRequest,data: web::Data<AppState>)->Result<HttpResponse,Error>{
    let app = data.app.clone();
    let headers = req.extensions().get::<Headers>().unwrap().clone();
    let user_id = headers.user_id;

    let id = ObjectId::parse_str(&user_id).unwrap();
    let filter = doc!{
        "_id": id,
    };

    let result = app.get_one_item::<User>(&filter,COLLECTION_NAME).await?;

    if let Some(res) = result.clone(){
        let user = res.build_output()?;
        return Ok(HttpResponse::Ok().json(response::get_success_msg(user.clone())));
    }
    
    Ok(HttpResponse::InternalServerError().json(responses::response::get_custom_err_msg(constant::NOT_FOUND)))
    
}

pub async fn init_2fa(req:HttpRequest,data: web::Data<AppState>)->Result<HttpResponse,Error>{
    let app = data.app.clone();
    let headers = req.extensions().get::<Headers>().unwrap().clone();
    let user_id = headers.user_id;

    let id = ObjectId::parse_str(&user_id).unwrap();
    let filter = doc!{
        "_id": id,
    };

    let operation_failed = HttpResponse::InternalServerError().json(responses::response::get_custom_err_msg(constant::OPERATION_FAILED));

    // get user info
    let result = app.get_one_item::<User>(&filter,COLLECTION_NAME).await?;
    if let Some(user) = result{
        let mut user = user;

        let (secret_key,auth_url_img) = two_fa::enroll_2fa(&user.email.clone());
    
        let obj_id = ObjectId::parse_str(&user_id).unwrap();
        let filter = doc! {"_id": obj_id};
    
        user.settings.security.two_fa = Some(TwoFA{
            is_enabled: true,
            secret_key
        });
    
        let user_doc = bson::to_document(&user);
        if let Err(_) = user_doc{
            return  Ok(operation_failed);
        }

        let user_doc = user_doc.unwrap();

        let p_doc = doc! {"updated_at": time::now()};
        let user_doc: Document = user_doc.into_iter().chain(p_doc).collect();
    
        let update = doc! {"$set": user_doc};
    
        let res = app.update_one_item::<User>(&filter,&update,COLLECTION_NAME).await?;

        if res.modified_count > 0{
            // notify the user of the enabled 2fa
            let closure = move || {
                let async_block = async {
                    let mut email_config = EmailConfig::default();
                    email_config.low_mail_input.recipients = vec![Recipient{ name: user.name.clone(), email: user.email }];
                    email_config.low_mail_input.subject = constant::TWO_FA_ENABLED.to_owned();
                    let msg = format!("2 Factor Authentication security layer has just been enabled on your account");
    
                    let template_builder = TemplateBuilder{
                        template: constant::MESSAGE.to_owned(),
                        replacers: vec![
                            ("title".to_owned(),email_config.low_mail_input.subject.clone()),
                            ("name".to_owned(),user.name.clone()),
                            (constant::MESSAGE.to_owned(), msg),
                            ("current_year".to_owned(),time::get_current_date_year_and_month().0),
                        ]
                    };
                    let template = email::build_template(&template_builder);
                    if let Ok(tempt) = template{
                        email_config.low_mail_input.message = tempt;
                        let _ = email::send_email(&email_config).await;
                    }
                };
            
                let rt = runtime::Runtime::new().unwrap();
                rt.block_on(async_block);
            };
            app.pool.run_job(closure);
    
            Ok(HttpResponse::Ok().json(response::get_success_msg(json!({"auth_url_img": auth_url_img}))))
    
        }else{
            Ok(operation_failed)
        }
    }else{
        Ok(operation_failed)
    }
}

pub async fn verify_2fa(_req:HttpRequest,data: web::Data<AppState>,payload: web::Json<Verify2FA>)->Result<HttpResponse,Error>{
    let app = data.app.clone();
    
    // validate payload input data
    let result = payload.validate();
    if let Err(e) = result {
        return Ok(HttpResponse::InternalServerError().json(response::get_validation_err_msg(e)));
    }

    // one of phone number or email must be provided
    if payload.phone.is_none() && payload.email.is_none(){
        return Ok(HttpResponse::InternalServerError().json(response::get_custom_err_msg(constant::PROVIDE_PHONE_OR_EMAIL))); 
    }

    let method_key_value;
    let result;
    if let Some(email) = payload.email.clone(){
        method_key_value = email.clone();

        let filter = doc! {"email": email.to_owned()};
        result = app.get_one_item::<User>(&filter,COLLECTION_NAME).await?;
    }else{
        method_key_value = payload.phone.clone().unwrap();

        let filter = doc! {"phone": method_key_value.clone()};
        result = app.get_one_item::<User>(&filter,COLLECTION_NAME).await?;
    }

    let key = format!("{:?}{:?}",constant::USER_VERIFICATION_DATA,method_key_value);
    let mut rc = app.redis.client;
    let user_cached_data = Redis::get_json_item::<UserOutput>(&mut rc, &key);

    let operation_failed = HttpResponse::InternalServerError().json(responses::response::get_custom_err_msg(constant::OPERATION_FAILED));

    if user_cached_data.is_none()  || result.is_none(){
        return Ok(operation_failed);
    }

    let user = user_cached_data.unwrap();
    let user_ = result.unwrap();

    let mut code_valid = false;

    if let Some(two_fa) = user_.settings.security.two_fa{
        if two_fa.is_enabled{
            let secret_key = two_fa.secret_key;
            let result = two_fa::verify_2fa(&secret_key,&payload.code);
            code_valid = result;

            // remove cached user output info
            if result{
                let _ = Redis::delete_item(&mut rc, &key);
            }
        }
    }

    if !code_valid{
        return Ok(operation_failed)
    }

    Ok(HttpResponse::Ok().json(responses::response::get_success_msg(user)))
}

pub async fn remove_2fa(req:HttpRequest,data: web::Data<AppState>)->Result<HttpResponse,Error>{
    let app = data.app.clone();
    let headers = req.extensions().get::<Headers>().unwrap().clone();
    let user_id = headers.user_id;

    let app2 = app.clone();
    let closure = move || {
        let app2 = app2;

        let async_block = async {

            let id = ObjectId::parse_str(&user_id).unwrap();
            let filter = doc!{
                "_id": id,
            };

            // get user info
            let result = app2.get_one_item::<User>(&filter,COLLECTION_NAME).await;
            if let Ok(user_opt) = result{
                if let Some(user) = user_opt{
                    let mut user = user;
                
                    let obj_id = ObjectId::parse_str(&user_id).unwrap();
                    let filter = doc! {"_id": obj_id};
                
                    user.settings.security.two_fa = None;
                
                    let user_doc = bson::to_document(&user);
                    if let Err(_) = user_doc{
                        return;
                    }
            
                    let user_doc = user_doc.unwrap();
            
                    let p_doc = doc! {"updated_at": time::now()};
                    let user_doc: Document = user_doc.into_iter().chain(p_doc).collect();
                
                    let update = doc! {"$set": user_doc};
                
                    let res = app2.update_one_item::<User>(&filter,&update,COLLECTION_NAME).await;
            
                    if res.is_ok() && res.unwrap().modified_count > 0{
                        // notify the user of the disabled 2fa
                        let mut email_config = EmailConfig::default();
                        email_config.low_mail_input.recipients = vec![Recipient{ name: user.name.clone(), email: user.email }];
                        email_config.low_mail_input.subject = constant::TWO_FA_DISABLED.to_owned();
                        let msg = format!("2 Factor Authentication security layer has just been disabled on your account");
        
                        let template_builder = TemplateBuilder{
                            template: constant::MESSAGE.to_owned(),
                            replacers: vec![
                                ("title".to_owned(),email_config.low_mail_input.subject.clone()),
                                ("name".to_owned(),user.name.clone()),
                                (constant::MESSAGE.to_owned(), msg),
                                ("current_year".to_owned(),time::get_current_date_year_and_month().0),
                            ]
                        };
                        let template = email::build_template(&template_builder);
                        if let Ok(tempt) = template{
                            email_config.low_mail_input.message = tempt;
                            let _ = email::send_email(&email_config).await;
                        }
                    }
                }
            }
        };
    
        let rt = runtime::Runtime::new().unwrap();
        rt.block_on(async_block);
    };
    app.pool.run_job(closure);


    Ok(HttpResponse::Ok().json(response::get_custom_success_msg(constant::SUCCESS)))
}

pub async fn update_user(req:HttpRequest,data: web::Data<AppState>,payload: web::Json<UpdateUser>)->Result<HttpResponse,HttpError>{
    let app = data.app.clone();
    let headers = req.extensions().get::<Headers>().unwrap().clone();
    let user_id = headers.user_id;

    let app2 = app.clone();
    let closure = move || {
        let app2 = app2;

        let async_block = async {

            let id = ObjectId::parse_str(&user_id).unwrap();
            let filter = doc!{
                "_id": id,
            };

            let result = app2.get_one_item::<User>(&filter,COLLECTION_NAME).await;

            if let Ok(res) = result{
                if let Some(user) = res{
                    // update user info here
                    let mut user = user;
                    if let Some(name) = payload.name.clone(){
                        user.name = name;
                    }

                    user.profile_image = payload.profile_image.clone();

                    // if changing their username, and it hasn't been taken, then proceed to changing it
                    if let Some(username) = payload.username.clone(){
                        let filter = doc!{
                            "username": payload.username.clone(),
                        };
                        let result = app2.get_one_item::<User>(&filter,COLLECTION_NAME).await;
                        if result.is_err() || result.unwrap().is_none(){
                            user.username = username;
                        }
                    }

                    if let Some(country) = payload.country.clone(){
                        user.country = country;
                    }
                    user.phone = payload.phone.clone();
                    user.dob = payload.dob.clone();
                    user.device_token = payload.device_token.clone();

                    // handle settings changes
                    if let Some(settings) = payload.settings.clone(){
                        user.settings.notifications.notify_me_on_transactions = settings.notifications.notify_me_on_transactions;

                        if let Some(login_notif) = settings.notifications.notify_me_on_new_login{
                            // if there's an existing new_login notification
                            if let Some(old_login_notif) = user.settings.notifications.notify_me_on_new_login{
                                user.settings.notifications.notify_me_on_new_login = Some(NewLoginNotif{
                                    is_enabled: login_notif.is_enabled,
                                    last_login_device: old_login_notif.last_login_device,
                                });

                            }else{
                                // if no new_login notification has been set
                                user.settings.notifications.notify_me_on_new_login = Some(NewLoginNotif{
                                    is_enabled: login_notif.is_enabled,
                                    last_login_device: None,
                                });
                            }
                        }
                    }

                    let user_doc = bson::to_document(&user);
                    if let Err(_) = user_doc{
                        return;
                    }
            
                    let user_doc = user_doc.unwrap();
            
                    let p_doc = doc! {"updated_at": time::now()};
                    let user_doc: Document = user_doc.into_iter().chain(p_doc).collect();
                
                    let update = doc! {"$set": user_doc};
                
                    let _ = app2.update_one_item::<User>(&filter,&update,COLLECTION_NAME).await;
                }
            }
        };
        let rt = runtime::Runtime::new().unwrap();
        rt.block_on(async_block);
    };
    app.pool.run_job(closure);
    
    Ok(HttpResponse::Ok().json(responses::response::get_custom_success_msg(constant::SUCCESS)))
}




// INTERNAL FUNCTIONS
pub async fn get_users_in_country_group(broadcast_type: &str, country_group: Option<Vec<String>>,app: &AppInstance)->Result<Vec<BasicUserInfo>,Error>{
    let filter;

    if let Some(cg) = country_group{
        // Convert the Vec<String> to Vec<Bson> for the query
        let countries_bson: Vec<Bson> = cg.into_iter().map(Bson::from).collect();

        if broadcast_type.eq(constant::BROADCAST_SMS){
            filter = doc!{
                "$and": [
                    {
                        "country": {
                            "$in": countries_bson
                        }
                    },
                    {
                        "phone": {
                            "$exists": true,
                            "$ne": "",
                            "$ne": null
                        }
                    },
                ]
            };
        }else if broadcast_type.eq(constant::BROADCAST_EMAIL){
            filter = doc!{
                "$and": [
                    {
                        "country": {
                            "$in": countries_bson
                        }
                    },
                    {
                        "email": {
                            "$exists": true,
                            "$ne": "",
                            "$ne": null
                        }
                    },
                ]
            };
        }else{
            filter = doc!{
                "$and": [
                    {
                        "country": {
                            "$in": countries_bson
                        }
                    },
                    {
                        "device_token": {
                            "$exists": true,
                            "$ne": "",
                            "$ne": null
                        }
                    },
                ]
            };
        }
    }else{
        if broadcast_type.eq(constant::BROADCAST_SMS){
            filter = doc!{
                "phone": {
                    "$exists": true,
                    "$ne": "",
                    "$ne": null
                }
            };
        }else if broadcast_type.eq(constant::BROADCAST_EMAIL){
            filter = doc!{
                "email": {
                    "$exists": true,
                    "$ne": "",
                    "$ne": null
                }
            };
        }else{
            filter = doc!{
                "device_token": {
                    "$exists": true,
                    "$ne": "",
                    "$ne": null
                }
            };
        }
    }

    let mut result = app.get_many_items_by_aggregate_randomized::<User>(&filter,COLLECTION_NAME,constant::BROADCAST_LIMIT).await?;

    let mut users = vec![];
    while let Some(user_doc) = result
    .try_next()
    .await
    .expect("Error mapping through cursor")
    {
        let res = bson::from_document(user_doc);
        if let Ok(user) = res{
            let user: BasicUserInfo = user;
            users.push(user);
        }
    }
    

    Ok(users)
}

pub async fn get_basic_user_info_simple(user_id: &str, app: &AppInstance)->Result<BasicUserInfoSimple,Error>{
    let id = ObjectId::parse_str(&user_id).unwrap();
    let filter = doc!{
        "_id": id,
    };

    let result = app.get_one_item::<User>(&filter,COLLECTION_NAME).await?;

    // username is not available
    if result.is_none(){
        return Err(actix_web::error::ErrorInternalServerError(constant::NOT_FOUND));
    }

    let user_v = serde_json::to_value(&result.unwrap())?;
    let user: BasicUserInfoSimple = serde_json::from_value(user_v)?;

    Ok(user)
}