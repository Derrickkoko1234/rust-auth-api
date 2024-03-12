use actix_web::http::header::HeaderMap;
use actix_web::http::StatusCode;
use actix_web::HttpMessage;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponseBuilder,
};
use futures_util::future::LocalBoxFuture;
use regex::Regex;
use std::future::{ready, Ready};
use std::sync::Arc;

use crate::library::{constant, token};

fn remove_version_from_link(link: &str) -> String {
    let re = Regex::new(r"/v\d+/").unwrap();
    let cleaned_link = re.replace(link, "/").to_string();
    cleaned_link
}

#[derive(Debug, Clone)]
pub struct Headers {
    pub origin: String,
    pub user_id: String,
    pub ip_address: String,
    pub x_access_token: String, // jwt token
}

impl Headers {
    pub fn default() -> Self {
        Self {
            origin: String::new(),
            user_id: String::new(),
            x_access_token: String::new(),
            ip_address: String::new(),
        }
    }

    pub fn get_req_headers(&mut self, headers: &HeaderMap, ip_addr: &str) {
        let _ = headers
            .get(constant::ORIGIN)
            .and_then(|value| value.to_str().map(|s| self.origin = s.to_owned()).ok());

        let _ = headers.get(constant::X_ACCESS_TOKEN).and_then(|value| {
            value
                .to_str()
                .map(|s| self.x_access_token = s.to_owned())
                .ok()
        });

        self.ip_address = ip_addr.to_owned();
    }
}

pub struct V1Mdw;

impl<S, B> Transform<S, ServiceRequest> for V1Mdw
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = APIMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(APIMiddleware {
            service: Arc::new(service),
        }))
    }
}

// #[derive(Clone)]
pub struct APIMiddleware<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for APIMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        let fut = async move {
            // if the platform is undergoing a maintenance, then inform the client
            if constant::UNDERGOING_MAINTENANCE {
                let response = HttpResponseBuilder::new(StatusCode::SERVICE_UNAVAILABLE)
                    .json(serde_json::json!({
                        "success": false,
                        "message": constant::MAINTENANCE_MODE_ACTIVATED,
                        "data": null
                    }))
                    .map_into_right_body();

                let rq = request.request().clone();
                return Ok(ServiceResponse::new(rq, response));
            }

            // if no maintenance, then proceed

            let conn_info = request.connection_info().clone();
            let ip_addr = match conn_info.realip_remote_addr() {
                Some(v) => v,
                None => constant::EMPTY_STR,
            };

            let mut hdrs = Headers::default();
            hdrs.get_req_headers(request.headers(), &ip_addr);

            // let app_state = request.app_data::<web::Data<AppState>>().clone().unwrap();

            // if user tries to access a protected route
            let mut rq_path = request.path();

            // use remove_version_from_link to remove the version from the link
            let binding = remove_version_from_link(rq_path);
            rq_path = &binding;

            if !constant::AUTHORIZED_ROUTES.contains(&rq_path) {
                // check if token is valid
                let (valid, user_id) = token::token_valid(&hdrs.x_access_token);

                // token is invalid
                if !valid {
                    let req = request.request().clone();
                    let response = HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
                        .json(serde_json::json!({
                            "success": false,
                            "message": constant::UNAUTHORIZED,
                            "data": null
                        }))
                        .map_into_right_body();

                    let rq = req.clone();
                    return Ok(ServiceResponse::new(rq, response));
                }

                // inject user_id to the request
                hdrs.user_id = user_id;
            }

            request.extensions_mut().insert(hdrs);

            // proceed with actual request
            let res = srv
                .call(request)
                .await
                .map(ServiceResponse::map_into_left_body)?;
            Ok(res)
        };

        Box::pin(fut)
    }
}
