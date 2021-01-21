use std::{net::TcpStream, time::{Duration, SystemTime}};

use chess_datagram::{DataPacketToClient, DataPacket};


#[derive(Debug, PartialEq)]
pub enum UserState {
  Alive,
  NoResponse,
  OffLine
}

#[derive(Debug)]
pub struct User {
  pub username: String,
  pub state: UserState,
  pub state_update_time: SystemTime,
  pub stream: TcpStream
}

impl User {
  pub fn new(username: &String, stream: TcpStream) -> User {
    User {
      username: username.clone(),
      state: UserState::Alive,
      state_update_time: SystemTime::now(),
      stream
    }
  }

  pub fn touch(&mut self) {
    self.state = UserState::Alive;
    self.state_update_time = SystemTime::now();
  }

  pub fn test(&mut self) {
    let duration = SystemTime::now().duration_since(self.state_update_time);
    if let Ok(duration) = duration {
      if duration > Duration::from_secs(300) {
        match self.state {
            UserState::Alive => {
              self.state = UserState::NoResponse;
              self.state_update_time = SystemTime::now();
              self.try_contact();
            }
            UserState::NoResponse => {
              self.make_offline();
            }
            UserState::OffLine => {}
        }
      }
    } else {
      self.make_offline();
    }
  }

  fn try_contact(&mut self) {
    let result = DataPacketToClient::aloha().send(&mut self.stream);

    if result.is_err() {
      self.make_offline();
    }
  }

  fn make_offline(&mut self) {
    self.state = UserState::NoResponse;
    self.state_update_time = SystemTime::now();
  }
}


#[derive(Debug)]
pub struct UserManager {
  users: Vec<User>
}

impl UserManager {
  pub fn new() -> UserManager {
    UserManager {
      users: vec![]
    }
  }

  pub fn add_user(&mut self, username: &String, stream: TcpStream) -> Result<(), ()> {
    self.clean();
    
    if self.find_user_by_name(username).is_some() {
      return Err(());
    }

    let new_user = User::new(username, stream);
    self.users.push(new_user);

    Ok(())
  }

  pub fn find_user_by_name(&mut self, username: &String) -> Option<&mut User> {
    for user in self.users.iter_mut() {
      if user.username == *username {
        return Some(user);
      }
    }

    None
  }

  pub fn clean(&mut self) {
    for user in self.users.iter_mut() {
      user.test();
    }

    let mut offline_indices = vec![];

    for (i, user) in self.users.iter().enumerate() {
      if user.state == UserState::OffLine {
        offline_indices.push(i);
      }
    }

    let mut accum = 0;
    for index in offline_indices {
      self.users.remove(index - accum);
      accum += 1;
    }
  }
}
