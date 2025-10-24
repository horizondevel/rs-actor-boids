use std::collections::HashMap;

use derive_more::Display;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    NUM_BOIDS,
    actor::Actor,
    boid::{BoidHandle, BoidId, BoidState},
    world::{WorldHandle, WorldTime},
};

#[derive(Debug, Clone)]
pub struct BoidManagerHandle {
    sender: Sender<BoidManagerMessage>,
}

impl BoidManagerHandle {
    pub fn new(sender: Sender<BoidManagerMessage>) -> Self {
        Self { sender }
    }

    pub async fn update(&self, time: WorldTime) -> crate::Result<()> {
        self.sender
            .send(BoidManagerMessage::WorldUpdate(time))
            .await?;
        Ok(())
    }
    pub async fn boid_update(&self, boid_state: BoidState) -> crate::Result<()> {
        self.sender
            .send(BoidManagerMessage::BoidUpdate(boid_state))
            .await?;
        Ok(())
    }
}
#[derive(Debug, Clone, Display)]
pub enum BoidManagerMessage {
    WorldUpdate(u64),
    BoidUpdate(BoidState),
    Stop,
}

pub struct BoidManager {
    receiver: Receiver<BoidManagerMessage>,
    world_handle: WorldHandle,
    boids: HashMap<BoidId, BoidHandle>,
    update_cycle_time: WorldTime,
    update_cycle_completed: Vec<BoidId>,
}

impl BoidManager {
    pub fn new(
        receiver: Receiver<BoidManagerMessage>,
        world_handle: &WorldHandle,
        manager_handle: &BoidManagerHandle,
    ) -> Self {
        let mut boids = HashMap::new();
        for i in 0..NUM_BOIDS {
            boids.insert(i, BoidHandle::new(i, manager_handle));
        }
        Self {
            receiver,
            world_handle: world_handle.clone(),
            boids,
            update_cycle_time: 0,
            update_cycle_completed: Vec::new(),
        }
    }
}
impl Actor<BoidManagerMessage> for BoidManager {
    async fn handle_message(&mut self, msg: BoidManagerMessage) -> crate::Result<()> {
        match msg {
            BoidManagerMessage::WorldUpdate(time) => {
                self.update_cycle_time = time;
                for boid_id in self.boids.keys() {
                    if let Some(handle) = self.boids.get(boid_id) {
                        handle.update(time).await?;
                    }
                }
            }

            BoidManagerMessage::BoidUpdate(state) => {
                self.update_cycle_completed.push(state.id);
                if let Some(handle) = self.boids.get(&state.id) {
                    handle.confirm(state).await?;
                }

                if self.update_cycle_completed.len() >= self.boids.len() {
                    self.update_cycle_completed.clear();
                    self.world_handle
                        .update_complete(self.update_cycle_time)
                        .await?;
                }
            }
            BoidManagerMessage::Stop => self.receiver.close(),
        }
        Ok(())
    }
    async fn recv(&mut self) -> Option<BoidManagerMessage> {
        self.receiver.recv().await
    }
}

pub struct BoidsState {}
