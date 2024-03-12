use actix_web::{web, Error, HttpRequest, HttpResponse};
use bson::oid::ObjectId;
use bson::Document;
use mongodb::bson::doc;
use redis::Client;
use tokio::runtime;
use validator::Validate;

use crate::{
    api::v1::{
        models::{
            auth::{SignIn, SignUp},
            user::{
                NewLoginNotif, User, UserNotifications, UserOutput, UserSecurity, UserSettings,
            },
        },
        responses::response,
    },
    app::redis::Redis,
    init::AppState,
    library::{
        code, constant, email, password,
        schema::{EmailConfig, Recipient, SMSConfig, TemplateBuilder},
        sms, time, token,
    },
};

static COLLECTION_NAME: &str = constant::USERS_COLLECTION;

// TODO: add endpoint to register user
pub async fn sign_up(
    _req: HttpRequest,
    data: web::Data<AppState>,
    payload: web::Json<SignUp>,
) -> Result<HttpResponse, Error> {
    /// Handles the POST /api/users request
    let app = data.app.clone();

    // validate payload input data
    let result = payload.validate();
    if let Err(e) = result {
        return Ok(HttpResponse::InternalServerError().json(response::get_validation_err_msg(e)));
    }

    // Check if user exists
    let filter = doc! {"email": payload.email.to_owned()};
    let result = app.get_one_item::<User>(&filter, COLLECTION_NAME).await?;
    if let Some(_) = result {
        let message = format!("Email address already exist! Please login in or try another email");
        return Ok(HttpResponse::InternalServerError().json(response::get_custom_err_msg(&message)));
    }

    // Create new user
    let mut new_user = User::default();

    new_user._id = Some(ObjectId::new());
    new_user.name = payload.name.clone();
    new_user.username = code::generate_username(&new_user.name);
    new_user.email = payload.email.clone();
    new_user.password = password::hash_password(&payload.password);
    new_user.country = payload.country.clone();
    new_user.is_verified = true;
    new_user.device_token = payload.device_token.clone();
    new_user.referral_code = code::generate_random_code(constant::CUSTOM_CODE_LEN);
    new_user.referred_by = payload.referrer_code.clone();
    new_user.access_role = constant::ACCESS_ROLES[0].to_owned();
    new_user.is_enabled = true;
    new_user.settings = UserSettings {
        notifications: UserNotifications {
            notify_me_on_transactions: true,
            notify_me_on_new_login: Some(NewLoginNotif {
                is_enabled: true,
                last_login_device: Some(payload.device_info.clone()),
            }),
        },
        security: UserSecurity { two_fa: None },
    };

    // Save user to db
    let data = app
        .insert_one_item::<User>(&new_user, COLLECTION_NAME)
        .await?;

    let message = format!("Please login to continue");
    return Ok(HttpResponse::Ok().json(response::get_custom_success_msg(&message)));
}

// TODO: add endpoint to login user
pub async fn sign_in(
    _req: HttpRequest,
    data: web::Data<AppState>,
    payload: web::Json<SignIn>,
) -> Result<HttpResponse, Error> {
    /// Handles the POST /api/users/login request
    let app = data.app.clone();

    // validate payload input data
    let result = payload.validate();
    if let Err(e) = result {
        return Ok(HttpResponse::InternalServerError().json(response::get_validation_err_msg(e)));
    }

    // Check if user exists
    let filter = doc! {"email": payload.email.to_owned()};
    let user = app.get_one_item::<User>(&filter, COLLECTION_NAME).await?;
    if let Some(user) = user {
        // Check if password is correct
        if password::verify_password(&payload.password, &user.password) {
            
            // Create token
            let token = token::create_token(&user._id.unwrap().to_string());

            // Return user object and token
            let user = user.build_output()?;
            let data = Data {
                user,
                token,
            };
            let res = response::get_success_msg(data);
            return Ok(HttpResponse::Ok().json(res));

            // return Ok(HttpResponse::Ok().json(response::get_success_msg(&user, &token)));
        } else {
            let message = format!("Invalid email or password");
            return Ok(
                HttpResponse::InternalServerError().json(response::get_custom_err_msg(&message))
            );
        }
    }

    let message = format!("Invalid email or password");
    return Ok(HttpResponse::InternalServerError().json(response::get_custom_err_msg(&message)));
}
