#[allow(unused_imports)]
use log::{info, error, warn};

use chess_datagram::*;

use super::Handler;

impl Handler {
  pub fn request_match(&mut self, with: String) -> DataPacketToClient {
    let mut model = self.model.lock();
    // Ensure the current user has a username 
    if self.username.is_none() 
      || model.as_mut().unwrap()
          .user_manager.find_user_by_name(&self.username.clone().unwrap())
          .is_none() {
      return DataPacketToClient::error("You haven't register or your client lost connection before.".into());
    }

    // Ensure the opponent is a valid user
    if model.as_mut().unwrap().user_manager.find_user_by_name(&with).is_none() {
      return DataPacketToClient::error(format!("There is no user with name '{}'.", with));
    }

    // Create a match and get its id.
    let match_id = {
      let this = model.as_mut().unwrap().user_manager.find_user_by_name(&self.username.clone().unwrap()).unwrap().clone();
      let that = model.as_mut().unwrap().user_manager.find_user_by_name(&with).unwrap().clone();

      let matching = model.as_mut().unwrap().match_manager.new_matching(
        this,
        that
      );

      matching.id
    };

    // Send request data packet to opponent.
    let opponent_stream = &mut model.as_mut().unwrap()
      .user_manager.find_user_by_name(&with).unwrap().stream;

    let result = DataPacketToClient::request_match(self.username.clone().unwrap(), match_id).send(opponent_stream);

    if result.is_ok() {
      DataPacketToClient::success()
    } else {
      DataPacketToClient::error("Cannot send request to your opponent.".into())
    }
  }

  pub fn accept_match(&mut self, id: usize) -> DataPacketToClient {
    let mut model = self.model.lock();

    // Ensure the current user has a username 
    if self.username.is_none() 
      || model.as_mut().unwrap()
          .user_manager.find_user_by_name(&self.username.clone().unwrap())
          .is_none() {
      return DataPacketToClient::error("You haven't register or your client lost connection before.".into());
    }

    // Ensure the matching exists
    if model.as_mut().unwrap().match_manager.find_match_by_id(id).is_none() 
      || model.as_mut().unwrap().match_manager.find_match_by_id(id).unwrap().that != self.username.clone().unwrap() {
      return DataPacketToClient::error("Match doesn't exist.".into());
    }

    // Start the match
    model.as_mut().unwrap().match_manager.find_match_by_id(id).unwrap().start();

    DataPacketToClient::success()
  }
}
