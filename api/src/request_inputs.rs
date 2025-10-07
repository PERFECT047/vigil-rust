use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSignupInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSigninInput {
    pub username: String,
    pub password: String,
}
