use std::time::Duration;

use tokio::sync::mpsc::{self, Sender, channel};
use tracing::{Level, debug, event, span};
pub type WorldTime = u64;
use crate::{
    END_TIME,
    boid_manager::{BoidManager, BoidManagerHandle, run_boid_manager},
};
pub async fn run_world(mut world: World) -> crate::Result<()> {
    while let Some(msg) = world.receiver.recv().await {
        {
            let span = span!(Level::DEBUG, "world_msg");
            let _enter = span.enter();
            world.handle_message(msg).await?;
        }
    }
    Ok(())
}

pub struct World {
    receiver: mpsc::Receiver<WorldMessage>,
    manager_handle: BoidManagerHandle,
    world_state: WorldState,
}
#[derive(Clone)]
pub struct WorldState {
    time: WorldTime,
}
#[derive(Clone)]
pub struct WorldHandle {
    sender: Sender<WorldMessage>,
}

impl WorldHandle {
    pub fn new(sender: Sender<WorldMessage>) -> Self {
        Self { sender }
    }

    pub async fn start(&self) -> crate::Result<()> {
        self.sender.send(WorldMessage::Start).await?;
        Ok(())
    }
    pub async fn stop(&self) -> crate::Result<()> {
        self.sender.send(WorldMessage::Stop).await?;
        Ok(())
    }

    pub async fn update_complete(&self, time: WorldTime) -> crate::Result<()> {
        self.sender.send(WorldMessage::UpdateComplete(time)).await?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum WorldMessage {
    Start,
    UpdateComplete(WorldTime),
    Stop,
}

impl World {
    pub fn new(
        receiver: mpsc::Receiver<WorldMessage>,
        world_handle: &WorldHandle,
        time: WorldTime,
    ) -> Self {
        let (send, recv) = channel(32);
        let manager_handle = BoidManagerHandle::new(send);
        let manager = BoidManager::new(recv, world_handle, &manager_handle);
        tokio::spawn(run_boid_manager(manager));
        Self {
            receiver,
            manager_handle,
            world_state: WorldState { time },
        }
    }

    async fn handle_message(&mut self, msg: WorldMessage) -> crate::Result<()> {
        tracing::debug!("Recieved message {msg:?}");
        match msg {
            WorldMessage::Stop => self.receiver.close(),
            WorldMessage::Start => {
                tracing::debug!("Starting the boids world");

                self.manager_handle.update(self.world_state.time).await?;
            }
            WorldMessage::UpdateComplete(time) => {
                debug!("Update cycle complete {time}");
                if time > END_TIME {
                    self.receiver.close()
                } else {
                    self.world_state.time = time;
                    self.manager_handle
                        .update(self.world_state.time + 1)
                        .await?;
                }
            }
        }
        Ok(())
    }
}
