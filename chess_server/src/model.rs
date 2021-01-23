pub mod user;
pub mod matching;

use std::sync::{Mutex, Arc};
pub use user::*;

use self::matching::MatchingManager;

lazy_static! {
  pub static ref SERVER_MODEL: Arc<Mutex<Model>> = Arc::new(Mutex::new(Model::new()));
}

#[derive(Debug)]
pub struct Model {
  pub user_manager: UserManager,
  pub match_manager: MatchingManager,
}

impl Model {
  pub fn new() -> Model {
    Model {
      user_manager: UserManager::new(),
      match_manager: MatchingManager::new(),
    }
  }
}
