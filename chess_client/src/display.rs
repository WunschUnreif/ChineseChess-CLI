use std::{sync::{Arc, Mutex, mpsc::channel}, thread::{self, JoinHandle}, time::Duration};

use thread::sleep;
use view_model::ViewModel;



pub mod board_widget;
pub mod match_board_widget;
pub mod view_model;

pub struct View {
  vm: Arc<Mutex<ViewModel>>,
  running: Arc<Mutex<bool>>,
  threads: Vec<JoinHandle<()>>,
}

impl View {
  pub fn new() -> Result<View, std::io::Error> {
    Ok(View {
      vm: Arc::new(Mutex::new(ViewModel::new()?)),
      running: Arc::new(Mutex::new(false)),
      threads: vec![],
    })
  }

  pub fn run(&mut self) {
    // Set the flag 
    *self.running.lock().unwrap() = true;

    // Create channels
    let (exec_tx, exec_rx) = channel();
    let (update_tx, update_rx) = channel();

    // Create command updating thread.
    let run_clone = self.running.clone();
    let vm_clone = self.vm.clone();
    let comm_update_thread = thread::spawn(move || {
      loop {
        if let Ok(x) = update_rx.recv() {
          if *run_clone.lock().unwrap() == false {
            break;
          }

          let mut vm = vm_clone.lock().unwrap();
          vm.update_command(x);
        }
      }
    });
    self.threads.push(comm_update_thread);

    // Create event handler thread.
    let run_clone = self.running.clone();
    let event_thread = thread::spawn(move || {
      view_model::handle_events(exec_tx, update_tx, run_clone);
    });
    self.threads.push(event_thread);

    // Create routine renderer thread 
    let run_clone = self.running.clone();
    let vm_clone = self.vm.clone();
    let render_thread = thread::spawn(move || {
      loop {
        if *run_clone.lock().unwrap() == false {
          break;
        }

        {
          let mut vm = vm_clone.lock().unwrap();
          vm.render();
        }

        sleep(Duration::from_millis(500));
      }
    });
    self.threads.push(render_thread);

    // Create command executing thread 
    let run_clone = self.running.clone();
    let vm_clone = self.vm.clone();
    let exec_thread = thread::spawn(move || {
      loop {
        if let Ok(x) = exec_rx.recv() {
          if *run_clone.lock().unwrap() == false {
            break;
          }
          
          let mut vm = vm_clone.lock().unwrap();
          vm.data_model.error_message = Some("Aha".into());
        }
      }
    });
    self.threads.push(exec_thread);
  }

  pub fn stop(&mut self) {
    println!("Please press a key to stop the program.");

    *(self.running.lock().unwrap()) = false;

    while !self.threads.is_empty() {
      let x = self.threads.pop().unwrap();
      let _ = x.join();
    }
  }
}
