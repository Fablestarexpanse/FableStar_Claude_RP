use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::interval;
use anyhow::Result;

use super::world::GameWorld;

/// Manages the simulation tick loop for real-time and fast-forward execution
pub struct TickManager {
    world: Arc<Mutex<GameWorld>>,
    tick_rate: Duration,
    running: Arc<AtomicBool>,
}

impl TickManager {
    /// Create a new tick manager with the given world and tick rate
    pub fn new(world: Arc<Mutex<GameWorld>>, tick_rate: Duration) -> Self {
        Self {
            world,
            tick_rate,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Create a tick manager with default 1-second tick rate
    pub fn with_default_rate(world: Arc<Mutex<GameWorld>>) -> Self {
        Self::new(world, Duration::from_secs(1))
    }

    /// Start the real-time simulation loop
    /// Runs continuously at the configured tick rate
    pub async fn start_realtime_loop(&self) {
        self.running.store(true, Ordering::SeqCst);
        let mut ticker = interval(self.tick_rate);
        
        println!("⏰ Tick manager starting real-time loop (tick rate: {:?})", self.tick_rate);
        
        while self.running.load(Ordering::SeqCst) {
            ticker.tick().await;
            
            // Execute one simulation tick
            if let Err(e) = self.execute_tick().await {
                eprintln!("❌ Error during tick execution: {}", e);
            }
        }
        
        println!("⏰ Tick manager stopped");
    }

    /// Stop the real-time simulation loop
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Fast-forward the simulation by a specified number of ticks
    /// Used when player logs in after being offline
    pub async fn fast_forward(&self, num_ticks: u64) -> Result<()> {
        println!("⏩ Fast-forwarding {} ticks...", num_ticks);
        
        let start_time = std::time::Instant::now();
        
        for i in 0..num_ticks {
            self.execute_tick().await?;
            
            // Progress update every 100 ticks
            if (i + 1) % 100 == 0 {
                println!("⏩ Progress: {}/{} ticks", i + 1, num_ticks);
            }
        }
        
        let elapsed = start_time.elapsed();
        println!("✅ Fast-forward complete in {:?} ({} ticks/sec)", 
                 elapsed, 
                 num_ticks as f64 / elapsed.as_secs_f64());
        
        Ok(())
    }

    /// Fast-forward by a duration of in-game time
    pub async fn fast_forward_duration(&self, duration: Duration) -> Result<()> {
        // Calculate number of ticks based on tick rate
        let num_ticks = (duration.as_secs_f64() / self.tick_rate.as_secs_f64()) as u64;
        self.fast_forward(num_ticks).await
    }

    /// Execute a single simulation tick
    async fn execute_tick(&self) -> Result<()> {
        let mut world = self.world.lock().await;
        
        // Run Bevy ECS systems
        world.tick();
        
        Ok(())
    }

    /// Get the current tick count
    pub async fn get_tick_count(&self) -> u64 {
        let world = self.world.lock().await;
        world.tick_count
    }

    /// Check if the tick loop is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Pause the simulation (stop ticking but don't destroy the manager)
    pub fn pause(&self) {
        self.running.store(false, Ordering::SeqCst);
        println!("⏸️  Simulation paused");
    }

    /// Resume the simulation after pausing
    pub fn resume(&self) {
        self.running.store(true, Ordering::SeqCst);
        println!("▶️  Simulation resumed");
    }
}

/// Builder for TickManager with configurable options
pub struct TickManagerBuilder {
    tick_rate: Duration,
}

impl TickManagerBuilder {
    pub fn new() -> Self {
        Self {
            tick_rate: Duration::from_secs(1),
        }
    }

    pub fn tick_rate(mut self, rate: Duration) -> Self {
        self.tick_rate = rate;
        self
    }

    pub fn ticks_per_second(mut self, tps: u64) -> Self {
        self.tick_rate = Duration::from_millis(1000 / tps);
        self
    }

    pub fn build(self, world: Arc<Mutex<GameWorld>>) -> TickManager {
        TickManager::new(world, self.tick_rate)
    }
}

impl Default for TickManagerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tick_manager_creation() {
        let world = Arc::new(Mutex::new(GameWorld::new()));
        let manager = TickManager::with_default_rate(world);
        
        assert!(!manager.is_running());
        assert_eq!(manager.get_tick_count().await, 0);
    }

    #[tokio::test]
    async fn test_fast_forward() {
        let world = Arc::new(Mutex::new(GameWorld::new()));
        let manager = TickManager::with_default_rate(world.clone());
        
        manager.fast_forward(10).await.unwrap();
        
        let tick_count = manager.get_tick_count().await;
        assert_eq!(tick_count, 10);
    }
}
