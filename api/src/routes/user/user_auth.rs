use crate::state::AppState;
use crate::{
    request_inputs::{UserSigninInput, UserSignupInput},
    response_outputs::{SignUpOutput, SigninOutput},
};
use jsonwebtoken::{EncodingKey, Header, encode};
use poem::{
    Error, handler,
    http::StatusCode,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

#[handler]
pub async fn sign_up(
    Json(data): Json<UserSignupInput>,
    Data(state): Data<&Arc<AppState>>,
) -> Result<Json<SignUpOutput>, Error> {
    // Offload blocking DB work to a dedicated blocking thread
    let store_arc = state.store.clone();
    let username = data.username.clone();
    let password = data.password.clone();

    let res = task::spawn_blocking(move || {
        let mut locked = store_arc.lock().unwrap();
        locked
            .sign_up_user(username, password)
            .map_err(|_| Error::from_status(StatusCode::CONFLICT))
    })
    .await
    .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let id = res?;
    let response = SignUpOutput { id };
    Ok(Json(response))
}

#[handler]
pub async fn sign_in(
    Json(data): Json<UserSigninInput>,
    Data(state): Data<&Arc<AppState>>,
) -> Result<Json<SigninOutput>, Error> {
    let store_arc = state.store.clone();
    let username = data.username.clone();
    let password = data.password.clone();

    let res = task::spawn_blocking(move || {
        let mut locked = store_arc.lock().unwrap();
        locked
            .sign_in_user(username, password)
            .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))
    })
    .await
    .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let user_id = res?;

    let jwt_expiry = state.jwt_expiry_seconds;
    let jwt_secret = state.jwt_secret.clone();

    let my_claims = Claims {
        sub: user_id,
        exp: jwt_expiry,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| Error::from_status(StatusCode::FORBIDDEN))?;

    let response = SigninOutput { jwt: token };
    Ok(Json(response))
}
