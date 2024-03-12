use crate::{
    api::v1::middlewares::middleware::V1Mdw,
    app::mongo::AppInstance,
    library::{constant, cron},
};
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    http,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use sentry;
use std::{env, time::Duration};

use crate::api::v1::routes::index::v1api;
use actix_cors::Cors;

// Define a struct to hold your shared data
#[derive(Clone)]
pub struct AppState {
    pub version: f64,
    pub app: AppInstance,
}

// initializes the app & server
#[actix_web::main]
pub async fn init_app() -> std::io::Result<()> {
    // init env vars
    dotenv().ok();

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    // start sentry monitoring
    let sentry_key = env::var("SENTRY_KEY").expect("SENTRY_KEY not found");
    let _guard = sentry::init((
        sentry_key,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let app = AppInstance::init().await;
    let app1 = app.clone();
    let app2 = app.clone();

    // init cron jobs
    cron::init_cron_job(app2);

    // general rate-limiting rules
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(30) // max. req per 2 seconds per ip address. i.e 30/2 = 15 req./sec on average
        .finish()
        .unwrap();

    let app4 = app1.clone();
    let app_state = AppState {
        version: constant::APP_VERSION,
        app: app4,
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
            .allow_any_header()
            .expose_headers(&[http::header::CONTENT_DISPOSITION])
            .max_age(constant::ONE_HOUR);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_conf)) // Enable Governor middleware
            .app_data(Data::new(app_state.clone()))
            // initialize api v1 routes
            .service(
                web::scope("/v1")
                    // add api key checker middleware here
                    .wrap(V1Mdw)
                    .configure(v1api::configure),
            )
            // the current version (helps us know what version has been published, since versions are increased for each deployment)
            .service(web::resource("/current-version").route(web::get().to(
                |data: web::Data<AppState>| async move {
                    HttpResponse::Ok().body(format!("Version: {}", data.version))
                },
            )))
    })
    // Set the request timeout
    .client_request_timeout(Duration::from_secs(constant::THREE_MILI_MINUTES as u64))
    .workers(constant::SERVER_N_WORKERS) // sets number of workers for concurrency. by default, the number of CPUs is used.
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
