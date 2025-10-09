use crate::routes::user::user_auth::Claims;
use jsonwebtoken::{DecodingKey, Validation, decode};
use poem::{Error, FromRequest, Request, RequestBody, http::StatusCode};
use std::env;

pub struct UserData(pub String);

impl<'a> FromRequest<'a> for UserData {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self, Error> {
        let token = req
            .header("Authorization")
            .map(|value| value.to_string())
            .ok_or_else(|| Error::from_string("missing token", StatusCode::UNAUTHORIZED))?;

        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(
                env::var("JWT_SECRET")
                    .unwrap_or_else(|_| panic!("JWT_SECRET must be set"))
                    .as_ref(),
            ),
            &Validation::default(),
        )
        .map_err(|_| Error::from_string("invalid token", StatusCode::UNAUTHORIZED))?;

        Ok(UserData(token_data.claims.sub))
    }
}
