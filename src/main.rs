use std::time::Instant;

use boids::{
    END_TIME,
    world::{World, WorldHandle, run_world},
};
use tokio::sync::mpsc;
use tracing::Level;

#[tokio::main]
async fn main() -> boids::Result<()> {
    let start = Instant::now();
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::INFO)
        .init();
    tracing::info!("Starting the boids system");
    let (send, recv) = mpsc::channel(32);
    let world_handle = WorldHandle::new(send);

    let world = World::new(recv, &world_handle, 0);
    world_handle.start().await?;
    let world_join = tokio::spawn(run_world(world));

    world_join.await??;
    let end_time: u32 = u32::try_from(END_TIME).ok().unwrap_or(100u32);
    tracing::info!(
        "Completed running... {:?}",
        (Instant::now().duration_since(start) / (end_time))
    );
    Ok(())
}
