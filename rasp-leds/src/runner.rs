use crate::Color;
use crate::{error::Result, Controller, LedController, LedError, Pattern, RunnablePattern};
use async_trait::async_trait;
use log::{error, info};
use serde::Serialize;
use std::{collections::VecDeque, thread, time::Duration};
use tokio::runtime::{Builder, Runtime};
use tokio::sync::{mpsc, oneshot};

type Sender = mpsc::Sender<Command>;
type Receiver = mpsc::Receiver<Command>;
type Responder<T> = oneshot::Sender<Result<T>>;

pub type HistoryList = VecDeque<History>;

/// An input to the internal state machine
#[derive(Debug)]
enum Command {
    On(Responder<()>),
    Off(Responder<()>),
    Power(Responder<()>),
    History(Responder<HistoryList>),
    Pattern(Responder<()>, Pattern),
    Info(Responder<Info>)
}

impl Command {
    fn into_history(&self) -> History {
        match self {
            Command::On(_) => History::On,
            Command::Off(_) => History::Off,
            Command::Power(_) => History::Power,
            Command::History(_) => History::History,
            Command::Pattern(_, pattern) => History::Pattern(pattern.clone()),
            Command::Info(_) => History::Info,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum History {
    On,
    Off,
    Power,
    History,
    Info,
    Pattern(Pattern),
}

#[derive(Debug, Clone, Serialize)]
pub struct Info {
    led_count: usize,
    current_state: State,
    state: Vec<Color>
}

impl Info {
    fn new(runner: &mut InnerRunner) -> Self {
       Self {
           led_count: runner.controller.get_count(),
           current_state: runner.state.clone(),
           state: runner.controller.get_data().clone()
       }
    }
}



pub struct LedRunner {
    sender: Sender,
}

impl LedRunner {
    #[cfg(feature = "simulate")]
    pub fn new(count: usize, cell_size: usize) -> Self {
        let (sender, receiver) = mpsc::channel(100);

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        thread::spawn(move || {
            let controller = Controller::new(count, cell_size);
            let mut inner = InnerRunner::new(runtime, receiver, controller);

            loop {
                inner.main_loop().map_err(|e| error!("Led Runner Main Loop Error: {:?}", e));
            }
        });

        Self { sender }
    }

    #[cfg(feature = "hardware")]
    pub fn new(count: usize, pin: i32, brightness: u8) -> Self {
        use log::warn;

        let (sender, receiver) = mpsc::channel(100);

        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        thread::spawn(move || {
            let controller = Controller::new(count, pin, brightness);
            let mut inner = InnerRunner::new(runtime, receiver, controller);

            loop {
                inner.main_loop().map_err(|e| error!("Led Runner Main Loop Error: {:?}", e));
            }
        });

        Self { sender }
    }
}

#[async_trait]
pub trait Runner {
    async fn on(&self) -> Result<()>;
    async fn off(&self) -> Result<()>;
    async fn power(&self) -> Result<()>;
    async fn history(&self) -> Result<HistoryList>;
    async fn pattern(&self, pattern: Pattern) -> Result<()>;
    async fn info(&self) -> Result<Info>;
}

macro_rules! send_message {
    {
        $(
            async fn $name:ident(&self $(,)? $($param:ident : $typ:ty),* $(,)?)
            $( -> $ret:ty )? | $en:expr
        )*
    } => {
        #[async_trait]
        impl Runner for LedRunner {
            $(
                async fn $name(&self, $($param: $typ),*) $( -> $ret)? {
                    let (resp, recv) = oneshot::channel();

                    let command = $en(resp, $($param),*);

                    self.sender.send(command).await.map_err(|_| LedError::SendError)?;

                    recv.await.unwrap()
                }
            )*
        }
    }
}

send_message! {
    async fn on(&self) -> Result<()>                        | Command::On
    async fn off(&self) -> Result<()>                       | Command::Off
    async fn power(&self) -> Result<()>                     | Command::Power
    async fn history(&self) -> Result<HistoryList>          | Command::History
    async fn pattern(&self, pattern: Pattern) -> Result<()> | Command::Pattern
    async fn info(&self) -> Result<Info>                    | Command::Info
}

#[derive(Debug, Serialize, Clone)]
enum State {
    Idle,
    Pattern { pattern: Pattern },
}

struct InnerRunner {
    runtime: Runtime,
    receiver: Receiver,
    controller: Controller,
    state: State,
    tick: u64,
    history: HistoryList,
}

impl InnerRunner {
    pub fn new(runtime: Runtime, receiver: Receiver, controller: Controller) -> Self {
        Self {
            runtime,
            receiver,
            controller,
            state: State::Idle,
            tick: 0,
            history: HistoryList::default(),
        }
    }
    pub fn main_loop(&mut self) -> Result<()> {
        self.receive_message(false)?;

        match &self.state {
            State::Idle => self.receive_message(true)?,
            State::Pattern { .. } => self.tick_pattern()?,
        }

        Ok(())
    }

    fn tick_pattern(&mut self) -> Result<()> {
        if let State::Pattern { pattern } = &mut self.state {
            pattern.start_tick(self.tick, &mut self.controller).unwrap();
            self.tick += 1;
            thread::sleep(Duration::from_millis(pattern.tick_rate()))
        }

        Ok(())
    }

    fn receive_message(&mut self, blocking: bool) -> Result<()> {
        if blocking {
            let command = self.runtime.block_on(self.receiver.recv()).unwrap();
            self.command_to_state(command)?;
        } else if let Ok(command) = self.receiver.try_recv() {
            self.command_to_state(command)?;
        }

        Ok(())
    }

    fn command_to_state(&mut self, command: Command) -> Result<()> {
        if let Command::Pattern(..) = command {
            self.history.push_front(command.into_history());
        }

        info!("Recieved Commands: {:?}", command);

        match command {
            Command::On(resp) => {
                let result: Result<()> = if let Some(pattern) = self.last_pattern() {
                    self.state = State::Pattern { pattern };
                    Ok(())
                } else {
                    Err(LedError::NoHistory)
                };

                let _ = resp.send(result);
            }
            Command::Off(resp) => {
                self.state = State::Idle;
                self.controller.clear(false)?;

                let _ = resp.send(Ok(()));
            }
            Command::Power(resp) => {
                let result = if let State::Idle = self.state {
                    if let Some(pattern) = self.last_pattern() {
                        self.state = State::Pattern { pattern };
                        Ok(())
                    } else {
                        Err(LedError::NoHistory)
                    }
                } else {
                    self.state = State::Idle;
                    self.controller.clear(false)?;
                    Ok(())
                };

                let _ = resp.send(result);
            }
            Command::History(resp) => {
                let _ = resp.send(Ok(self.history.clone()));
            }
            Command::Pattern(resp, mut pattern) => {
                let result = pattern.init(&mut self.controller);

                if result.is_ok() {
                    self.controller.clear(true)?;
                    self.state = State::Pattern { pattern }
                }

                let _ = resp.send(result);
            }
            Command::Info(resp) => {
                let _ = resp.send(Ok(Info::new(self)));
            },
        }

        Ok(())
    }

    fn last_pattern(&self) -> Option<Pattern> {
        for history in &self.history {
            if let History::Pattern(pattern) = history {
                return Some(pattern.clone());
            }
        }

        None
    }
}
