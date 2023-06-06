use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use kompusim::{bus, device::Device, ram, rv64i_cpu::RV64ICpu, uart::Uart};

pub struct Simulator {
    sim_thread: Option<thread::JoinHandle<()>>,
    cmd_channel: Sender<SimCommand>,
}

enum SimCommand {
    //Reset,
    //Init,
    LoadBin(&'static [u8]),
    Continue,
    Stop,
}

impl Simulator {
    pub fn new() -> Self {
        let (tx, rx): (Sender<SimCommand>, Receiver<SimCommand>) = mpsc::channel();
        // Start the simulator thread
        let sim_thread_handler = thread::spawn(move || {
            let addr = 0x0000000080000000; // TODO: remove
            let ram_sz = 4 * 1024; // TODO: remove
            let ram = ram::Ram::new(addr, ram_sz);
            let mut bus = bus::Bus::new();
            bus.attach_ram(ram);
            bus.attach_device(Device::new(
                Box::new(Uart::new("0".to_string())),
                0x1001_0000,
                0x20,
            ));
            let mut cpu0 = RV64ICpu::new(bus);
            cpu0.regs.pc = addr;

            loop {
                let cmd = rx
                    .recv()
                    .expect("Simulator: Failed to receive from channel");
                match cmd {
                    // SimCommand::Reset => {
                    //     println!("Simulator: reset command")
                    // }
                    //SimCommand::Init => {}
                    SimCommand::LoadBin(bin) => {}
                    SimCommand::Continue => {
                        let _ = cpu0.exec_continue(u64::MAX);
                    }
                    SimCommand::Stop => break,
                }
                //thread::sleep(time::Duration::from_secs(1));
                // TODO: receive commands from the gui main thread
            }
            println!("Simulator: exiting the simulator thread");
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

    // continue is a Rust keyword
    pub fn carry_on(&self) {
        self.cmd_channel.send(SimCommand::Continue).unwrap();
    }

    pub fn attach_serial() {}
}
