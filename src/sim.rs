use core::time;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub struct Simulator {
    sim_thread: thread::JoinHandle<()>,
}

enum SimCommand {
    Reset,
}

impl Simulator {
    pub fn new() -> Self {
        let (tx, rx): (Sender<SimCommand>, Receiver<SimCommand>) = mpsc::channel();
        // Start the simulator thread
        let sim_thread_handler = thread::spawn(move || loop {
            //println!("Hi from thread");
            //thread::sleep(time::Duration::from_secs(1));

            // TODO: receive commands from the gui main thread
        });
        Simulator {
            sim_thread: sim_thread_handler,
        }
    }

    pub fn stop(self) {
        self.sim_thread.join().unwrap();
    }
}
