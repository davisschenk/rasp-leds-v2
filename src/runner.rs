use crate::{Pattern, RunnablePattern, controller::*};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type TSController = Arc<Mutex<Controller>>;
type TSState = Arc<Mutex<State>>;
type MessageSender = mpsc::Sender<State>;
type MessageReceiver = mpsc::Receiver<State>;

// type ResultSender = mpsc::Sender<LedResponse>;
// type ResultReceiver = mpsc::Receiver<LedResponse>;

enum State {
    Idle,
    Pattern { pattern: Pattern },
}

pub trait Runner {
    fn run_pattern(&mut self, pattern: Pattern);
    fn set_idle(&mut self);
    fn start(&mut self);
}

pub struct LedRunner {
    inner: Arc<Mutex<InnerRunner>>,
    sender: MessageSender,
}

impl Runner for LedRunner {
    fn start(&mut self) {
        let inner = self.inner.clone();

        thread::spawn(move || loop {
            if let Ok(mut inr) = inner.lock() {
                inr.main_loop()
            }
        });
    }

    fn run_pattern(&mut self, pattern: Pattern) {
        self.sender.send(State::Pattern{pattern}).unwrap();
    }

    fn set_idle(&mut self) {
        self.sender.send(State::Idle).unwrap();
    }
}

impl LedRunner {
    #[cfg(feature = "simulate")]
    pub fn new(count: usize, cell_size: usize) -> Self {
        let (sender, reciever) = mpsc::channel();

        Self {
            inner: Arc::new(Mutex::new(InnerRunner::new(reciever, count, cell_size))),
            sender,
        }
    }

    #[cfg(feature = "hardware")]
    pub fn new(count: usize, pin: i32, brightness: u8) -> Self {
        let (sender, reciever) = mpsc::channel();

        Self {
            inner: Arc::new(Mutex::new(InnerRunner::new(
                reciever, count, pin, brightness,
            ))),
            sender,
        }
    }
}

struct InnerRunner {
    controller: TSController,
    state: TSState,
    receiver: MessageReceiver,
    tick: Arc<Mutex<u64>>
}

impl InnerRunner {
    #[cfg(feature = "simulate")]
    pub fn new(recv: MessageReceiver, count: usize, cell_size: usize) -> Self {
        let controller = Controller::new(count, cell_size);
        Self::from_controller(controller, recv)
    }

    #[cfg(feature = "hardware")]
    pub fn new(recv: MessageReceiver, count: usize, pin: i32, brightness: u8) -> Self {
        let controller = Controller::new(count, pin, brightness);
        Self::from_controller(controller, recv)
    }

    fn from_controller(controller: Controller, receiver: MessageReceiver) -> Self {
        Self {
            controller: Arc::new(Mutex::new(controller)),
            state: Arc::new(Mutex::new(State::Idle)),
            receiver,
            tick: Arc::new(Mutex::new(0u64))
        }
    }

    pub fn main_loop(&mut self) {
        self.recv_message(false);

        if let Ok(mut state) = self.state.lock() {
            match &mut *state {
                State::Idle => self.recv_message(true),
                State::Pattern { ref mut pattern } => self.tick_pattern(pattern),
            }
        }
    }

    fn recv_message(&self, blocking: bool) {
        if let Ok(mut state) = self.state.lock() {
            if blocking {
                *state = self.receiver.recv().unwrap();
            } else if let Ok(new_state) = self.receiver.try_recv() {
                *state = new_state;
            }
        }
    }

    fn tick_pattern(&self, pattern: &mut Pattern) {
        if let (Ok(mut controller), Ok(mut tick)) = (self.controller.lock(), self.tick.lock()) {
            pattern.start_tick(*tick, &mut controller).unwrap();
            *tick += 1;
        }

        thread::sleep(Duration::from_millis(pattern.tick_rate()))
    }
}
