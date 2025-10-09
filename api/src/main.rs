use std::sync::Arc;

use poem::{EndpointExt, Route, Server, get, listener::TcpListener, post};

use store::store::Store;

use crate::routes::{
    user::user_auth::{sign_in, sign_up},
    website::website::{create_website, get_website},
};
pub mod middlewares;
pub mod request_inputs;
pub mod response_outputs;
pub mod routes;
pub mod state;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    // load config from env
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_expiry_seconds = std::env::var("JWT_EXPIRY_SECONDS")
        .unwrap_or_else(|_| "3600".to_string())
        .parse::<usize>()
        .expect("JWT_EXPIRY_SECONDS must be a usize");

    // Initialize store synchronously (blocking) before starting the server
    let store = Store::new().expect("failed to create store");

    let app_state = Arc::new(state::AppState::new(store, jwt_secret, jwt_expiry_seconds));

    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in))
        .data(app_state.clone());

    // creates and runs the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("vigil-api")
        .run(app)
        .await
}
