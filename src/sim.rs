use core::time;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub struct Simulator {
    sim_thread: Option<thread::JoinHandle<()>>,
    cmd_channel: Sender<SimCommand>,
}

enum SimCommand {
    Reset,
    Stop,
}

impl Simulator {
    pub fn new() -> Self {
        let (tx, rx): (Sender<SimCommand>, Receiver<SimCommand>) = mpsc::channel();
        // Start the simulator thread
        let sim_thread_handler = thread::spawn(move || {
            loop {
                let cmd = rx
                    .recv()
                    .expect("Simulator: Failed to receive from channel");
                match cmd {
                    SimCommand::Reset => println!("Simulator: reset command"),
                    SimCommand::Stop => break,
                }
                println!("Simulator: exiting the simulator thread");
                //thread::sleep(time::Duration::from_secs(1));

                // TODO: receive commands from the gui main thread
            }
        });
        Simulator {
            sim_thread: Some(sim_thread_handler),
            cmd_channel: tx,
        }
    }

    pub fn stop(&mut self) {
        if self.sim_thread.is_some() {
            if let Err(err) = self.cmd_channel.send(SimCommand::Stop) {
                println!("Simulator: failed to send command: {}", err);
            }
            self.sim_thread.take().unwrap().join().unwrap();
        }
    }
}
