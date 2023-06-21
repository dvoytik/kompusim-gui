use std::{
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    thread,
};

use kompusim::{bus, device::Device, ram, rv64i_cpu::RV64ICpu, uart::Uart};

pub struct Simulator {
    sim_thread: Option<thread::JoinHandle<()>>,
    cmd_channel: Sender<SimCommand>,
    /// UART TX receive queue
    uart_tx_recv: Receiver<u8>,
}

enum SimCommand {
    //Reset,
    //Init,
    LoadImage((u64, &'static [u8])),
    Continue,
    Stop,
}

impl Simulator {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx): (Sender<SimCommand>, Receiver<SimCommand>) = mpsc::channel();
        let (uart_tx_send, uart_tx_recv): (Sender<u8>, Receiver<u8>) = mpsc::channel();

        // Start the simulator thread
        let sim_thread_handler = thread::spawn(move || {
            let addr = 0x0000000080000000; // TODO: remove
            let ram_sz = 4 * 1024; // TODO: remove
            let ram = ram::Ram::new(addr, ram_sz);
            let mut bus = bus::Bus::new();
            bus.attach_ram(ram);

            let mut uart0 = Box::new(Uart::new("0".to_string()));
            uart0.register_out_callback(Box::new(move |b: u8| {
                if let Err(err) = uart_tx_send.send(b) {
                    println!("Simulator: failed to send command: {}", err);
                }
            }));
            bus.attach_device(Device::new(uart0, 0x1001_0000, 0x20));

            let mut cpu0 = RV64ICpu::new(bus);
            cpu0.regs.pc = addr;

            loop {
                let cmd = cmd_rx
                    .recv()
                    .expect("Simulator: Failed to receive from channel");
                match cmd {
                    // SimCommand::Reset => {
                    //     println!("Simulator: reset command")
                    // }
                    //SimCommand::Init => {}
                    SimCommand::LoadImage((load_addr, image)) => {
                        cpu0.bus.load_image(load_addr, image).unwrap();
                        println!("Simulator: image loaded at 0x{:x}", load_addr);
                    }
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
            cmd_channel: cmd_tx,
            uart_tx_recv,
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

    pub fn load_image(&mut self, addr: u64, image: &'static [u8]) {
        self.cmd_channel
            .send(SimCommand::LoadImage((addr, image)))
            .unwrap();
    }

    // continue is a Rust keyword, so use carry_on()
    pub fn carry_on(&self) {
        self.cmd_channel.send(SimCommand::Continue).unwrap();
    }

    pub fn console_recv(&self) -> Option<u8> {
        match self.uart_tx_recv.try_recv() {
            Ok(byte) => Some(byte),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                println!("Simulator: FATAL ERROR: got Disconnected on UART TX receive attemp");
                None
            }
        }
    }
}
