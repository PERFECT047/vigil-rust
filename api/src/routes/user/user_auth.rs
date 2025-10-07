use std::sync::{Arc, Mutex};

use poem::{
 handler, web::{Data, Json}
};
use store::store::Store;

use crate::{request_inputs::{UserSigninInput, UserSignupInput}, response_outputs::{SignUpOutput, SigninOutput}};


#[handler]
pub fn sign_up(
    Json(data): Json<UserSignupInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<SignUpOutput> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up_user(data.username, data.password).unwrap();

    let response = SignUpOutput { id };

    Json(response)
}

#[handler]
pub fn sign_in(
    Json(data): Json<UserSigninInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<SigninOutput> {
    let mut locked_s = s.lock().unwrap();
    let _exists = locked_s.sign_in_user(data.username, data.password).unwrap();

    let response = SigninOutput {
        jwt: String::from("siddhartha"),
    };

    Json(response)
}