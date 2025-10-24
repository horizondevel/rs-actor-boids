use derive_more::Display;
use tokio::sync::mpsc;

use crate::{
    actor::{Actor, run_actor},
    boid_manager::BoidManagerHandle,
    world::WorldTime,
};
pub type BoidId = u64;

#[derive(Debug)]
pub struct Boid {
    receiver: mpsc::Receiver<BoidMessage>,
    boid_state: BoidState,
    manager_handle: BoidManagerHandle,
}
#[derive(Debug, Clone, Display)]
#[display("id")]
pub struct BoidState {
    pub id: BoidId,
    pub last_update_time: WorldTime,
    pub pos: (f64, f64),
    pub vel: (f64, f64),
}

impl BoidState {
    pub fn new(id: BoidId) -> Self {
        Self {
            id,
            last_update_time: 0,
            pos: (0.0, 0.0),
            vel: (1.0, 0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoidHandle {
    sender: mpsc::Sender<BoidMessage>,
}

impl BoidHandle {
    pub fn new(id: BoidId, manager_handle: &BoidManagerHandle) -> Self {
        let (send, recv) = mpsc::channel(32);

        let boid = Boid {
            receiver: recv,
            manager_handle: manager_handle.clone(),
            boid_state: BoidState::new(id),
        };
        tokio::spawn(run_actor(boid));
        Self { sender: send }
    }
    pub async fn update(&self, time: WorldTime) -> crate::Result<()> {
        self.sender.send(BoidMessage::Update(time)).await?;
        Ok(())
    }
    pub async fn confirm(&self, boid_state: BoidState) -> crate::Result<()> {
        self.sender.send(BoidMessage::Confirm(boid_state)).await?;
        Ok(())
    }
}
#[derive(Debug, Display)]
pub enum BoidMessage {
    Update(u64),
    Confirm(BoidState),
}

impl Actor for Boid {
    type Message = BoidMessage;
    async fn handle_message(&mut self, msg: BoidMessage) -> crate::Result<()> {
        match msg {
            BoidMessage::Update(time) => {
                let mut new_state = self.boid_state.clone();
                new_state.pos = (
                    new_state.pos.0 + new_state.vel.0,
                    new_state.pos.1 + new_state.vel.1,
                );
                new_state.last_update_time = time;
                self.manager_handle.boid_update(new_state).await?;
            }
            BoidMessage::Confirm(boid_state) => {
                self.boid_state = boid_state;
            }
        }
        Ok(())
    }

    async fn recv(&mut self) -> Option<BoidMessage> {
        self.receiver.recv().await
    }
}
