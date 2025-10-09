use std::sync::{Arc, Mutex};

use crate::{
    request_inputs::{UserSigninInput, UserSignupInput},
    response_outputs::{SignUpOutput, SigninOutput},
};
use dotenvy::dotenv;
use jsonwebtoken::{EncodingKey, Header, encode};
use poem::{
    Error, handler,
    http::StatusCode,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};
use std::env;
use store::store::Store;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

#[handler]
pub fn sign_up(
    Json(data): Json<UserSignupInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<SignUpOutput>, Error> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s
        .sign_up_user(data.username, data.password)
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;

    let response = SignUpOutput { id };
    Ok(Json(response))
}

#[handler]
pub fn sign_in(
    Json(data): Json<UserSigninInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<SigninOutput>, Error> {
    let mut locked_s = s.lock().unwrap();
    let user = locked_s.sign_in_user(data.username, data.password);

    match user {
        Ok(user) => {
            dotenv().ok();
            let my_claims = Claims {
                sub: user,
                exp: env::var("JWT_EXPIRY_SECONDS")
                    .unwrap_or_else(|_| panic!("JWT_EXPIRY_SECONDS must be set"))
                    .parse::<usize>()
                    .expect("JWT_EXPIRY_SECONDS must be a valid usize"),
            };

            let token = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret(
                    env::var("JWT_SECRET")
                        .unwrap_or_else(|_| panic!("JWT_SECRET must be set"))
                        .as_ref(),
                ),
            )
            .map_err(|_| Error::from_status(StatusCode::FORBIDDEN))?;
            let response = SigninOutput { jwt: token };
            Ok(Json(response))
        }

        Err(_e) => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
    }
}
