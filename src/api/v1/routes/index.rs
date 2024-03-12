pub mod v1api {
    use crate::api::v1::routes::auth::auth_routes;
    use crate::api::v1::routes::user::user_routes;
    use actix_web::web;

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/auth").configure(auth_routes::configure));
        cfg.service(web::scope("/user").configure(user_routes::configure));
    }
}
