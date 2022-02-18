use crate::{controller::*, Pattern, RunnablePattern};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type MessageSender = mpsc::Sender<Command>;
type MessageReceiver = mpsc::Receiver<Command>;

// type ResultSender = mpsc::Sender<LedResponse>;
// type ResultReceiver = mpsc::Receiver<LedResponse>;

#[derive(Debug)]
enum State {
    Idle,
    Pattern { pattern: Pattern },
}

#[derive(Debug)]
enum Command {
    On,
    Off,
    Power,
    Pattern { pattern: Pattern },
}

pub trait Runner {
    fn run_pattern(&mut self, pattern: Pattern);
    fn off(&mut self);
    fn on(&mut self);
    fn power(&mut self);
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
        self.send_message(Command::Pattern { pattern });
    }

    fn off(&mut self) {
        self.send_message(Command::Off);
    }

    fn on(&mut self) {
        self.send_message(Command::On)
    }

    fn power(&mut self) {
        self.send_message(Command::Power)
    }
}

impl LedRunner {
    /// Create a new Virtual `LedRunner`
    ///
    /// # Arguments
    ///
    /// * `count` - The number of virtual leds
    /// * `cell_size` - The size in pixels for each virtual led
    #[cfg(feature = "simulate")]
    pub fn new(count: usize, cell_size: usize) -> Self {
        let (sender, reciever) = mpsc::channel();

        Self {
            inner: Arc::new(Mutex::new(InnerRunner::new(reciever, count, cell_size))),
            sender,
        }
    }

    /// Create a new Hardware `LedRunner`
    ///
    /// # Arguments
    ///
    /// * `count` - The number of leds on the strip
    /// * `pin` - The raspberry pin the leds are connected too
    /// * `brightness` - The base brightness to run the lights at
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

    fn send_message(&self, command: Command) {
        self.sender.send(command).unwrap();
    }
}

struct InnerRunner {
    controller: Controller,
    state: State,
    receiver: MessageReceiver,
    tick: u64,
    past_states: Vec<State>,
}

impl InnerRunner {
    #[cfg(feature = "simulate")]
    fn new(recv: MessageReceiver, count: usize, cell_size: usize) -> Self {
        let controller = Controller::new(count, cell_size);
        Self::from_controller(controller, recv)
    }

    #[cfg(feature = "hardware")]
    fn new(recv: MessageReceiver, count: usize, pin: i32, brightness: u8) -> Self {
        let controller = Controller::new(count, pin, brightness);
        Self::from_controller(controller, recv)
    }

    fn from_controller(controller: Controller, receiver: MessageReceiver) -> Self {
        Self {
            controller: controller,
            state: State::Idle,
            receiver,
            tick: 0u64,
            past_states: vec![],
        }
    }

    pub fn main_loop(&mut self) {
        self.recv_message(false);

        match &self.state {
            State::Idle => self.recv_message(true),
            State::Pattern { .. } => self.tick_pattern(),
        }
    }

    /// Recieve a message from the reciever and change internal state
    ///
    /// # Arguments
    /// * `blocking` - Whether or not to wait for a message to set the state
    fn recv_message(&mut self, blocking: bool) {
        if blocking {
            let command = self.receiver.recv().unwrap();
            let new_state = self.map_command_to_state(command);
            self.change_state(new_state)
        } else if let Ok(command) = self.receiver.try_recv() {
            let new_state = self.map_command_to_state(command);
            self.change_state(new_state);
        }
    }

    fn map_command_to_state(&mut self, command: Command) -> State {
        match command {
            Command::On => self.past_states.pop().unwrap_or(State::Idle),
            Command::Off => State::Idle,
            Command::Power => match self.state {
                State::Idle => self.past_states.pop().unwrap_or(State::Idle),
                _ => State::Idle,
            },
            Command::Pattern { pattern } => State::Pattern { pattern },
        }
    }

    fn change_state(&mut self, new_state: State) {
        let old_state = std::mem::replace(&mut self.state, new_state);

        match old_state {
            State::Idle => (),
            _ => self.past_states.push(old_state)
        }
    }

    fn tick_pattern(&mut self) {
        if let State::Pattern { pattern } = &mut self.state {
            pattern.start_tick(self.tick, &mut self.controller).unwrap();
            self.tick += 1;
            thread::sleep(Duration::from_millis(pattern.tick_rate()))
        }
    }
}
