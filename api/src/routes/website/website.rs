use std::sync::{Arc, Mutex};

use poem::{
    handler,
    web::{Data, Json, Path},
};
use store::store::Store;

use crate::{
    request_inputs::CreateWebsiteInput,
    response_outputs::{CreateWebsiteOutput, GetWebsiteOutput},
};

#[handler]
pub fn get_website(
    Path(id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id).unwrap();
    Json(GetWebsiteOutput { url: website.url })
}

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s
        .create_website(
            String::from("b8c7a13c-e518-4888-b196-db0b159a8723"),
            data.url,
        )
        .unwrap();

    let response = CreateWebsiteOutput { id: website.id };
    Json(response)
}
