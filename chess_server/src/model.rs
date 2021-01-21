pub mod user;

use std::sync::{Mutex, Arc};
pub use user::*;

lazy_static! {
  pub static ref SERVER_MODEL: Arc<Mutex<Model>> = Arc::new(Mutex::new(Model::new()));
}

#[derive(Debug)]
pub struct Model {
  pub user_manager: UserManager
}

impl Model {
  pub fn new() -> Model {
    Model {
      user_manager: UserManager::new()
    }
  }
}
