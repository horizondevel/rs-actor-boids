use derive_more::From;
use tokio::{sync::mpsc::error::SendError, task::JoinError};

use crate::world::WorldMessage;
use crate::{boid::BoidMessage, boid_manager::BoidManagerMessage};
pub mod actor;
pub mod boid;
pub mod boid_manager;
pub mod world;
pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, From)]
pub enum Error {
    #[from]
    Error(String),
    #[from]
    SendErrorWorld(SendError<WorldMessage>),
    #[from]
    SendErrorBoidManager(SendError<BoidManagerMessage>),
    #[from]
    SendErrorBoid(SendError<BoidMessage>),
    #[from]
    JoinError(JoinError),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

pub const END_TIME: u64 = 60 * 60;
pub const NUM_BOIDS: u64 = 100;
