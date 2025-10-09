use crate::{
    middlewares::auth_middleware::UserData,
    request_inputs::CreateWebsiteInput,
    response_outputs::{CreateWebsiteOutput, GetWebsiteOutput},
};
use poem::{
    handler,
    web::{Data, Json, Path},
};
use std::sync::{Arc, Mutex};
use store::store::Store;

#[handler]
pub fn get_website(
    Path(id): Path<String>,
    UserData(user_id): UserData,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id, user_id).unwrap();
    Json(GetWebsiteOutput {
        url: website.url,
        id: website.id,
        user_id: website.user_id,
    })
}

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    UserData(user_id): UserData,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.create_website(user_id, data.url).unwrap();

    let response = CreateWebsiteOutput { id: website.id };
    Json(response)
}
