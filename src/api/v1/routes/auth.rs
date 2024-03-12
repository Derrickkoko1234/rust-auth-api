pub mod auth_routes {
    use actix_web::{guard, web};

    use crate::api::v1::handlers::auth::sign_up;

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/sign-up")
                .name("sign_up")
                .guard(guard::Post())
                .route(web::post().to(sign_up)),
        );

        // cfg.service(
        //     web::resource("/sign-in")
        //         .name("sign_in")
        //         .guard(guard::Post())
        //         .route(web::post().to(sign_in)),
        // );
    }
}
