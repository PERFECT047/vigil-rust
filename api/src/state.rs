use std::sync::{Arc, Mutex};
use store::store::Store;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<Store>>,
    pub jwt_secret: String,
    pub jwt_expiry_seconds: usize,
}

impl AppState {
    pub fn new(store: Store, jwt_secret: String, jwt_expiry_seconds: usize) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
            jwt_secret,
            jwt_expiry_seconds,
        }
    }
}
