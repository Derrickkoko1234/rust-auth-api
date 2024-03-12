
pub mod user_routes{
    use actix_web::{web, guard};

    use crate::api::v1::handlers::user::{
        check_username_availability,
        refresh_user,
        init_2fa,
        verify_2fa,
        remove_2fa,
        update_user,
    };

    pub fn configure(cfg: &mut web::ServiceConfig){
        cfg.service(
            web::resource("/check-username-availability/{username}")
            .name("check_username_availability") 
            .guard(guard::Get())
            .route(web::get().to(check_username_availability))
        );

        cfg.service(
            web::resource("/refresh")
            .name("refresh_user")
            .guard(guard::Get())
            .route(web::get().to(refresh_user))
        );
        cfg.service(
            web::resource("/init-2fa")
            .name("init_2fa")
            .guard(guard::Get())
            .route(web::get().to(init_2fa))
        );

        cfg.service(
            web::resource("/verify-2fa")
            .name("verify_2fa")
            .guard(guard::Post())
            .route(web::post().to(verify_2fa))
        );

        cfg.service(
            web::resource("/remove-2fa")
            .name("remove_2fa")
            .guard(guard::Get())
            .route(web::get().to(remove_2fa))
        );

        cfg.service(
            web::resource("")
            .name("update_user")
            .guard(guard::Patch())
            .route(web::patch().to(update_user))
        );
    }
}