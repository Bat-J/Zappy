use crate::command::Command;
use ordered_list::OrderedList;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ScheduledCommand {
    pub command: Command,
    pub execute_at: Instant,
    pub client_id: u32,
}

pub struct TimeManager {
    queue: OrderedList<ScheduledCommand, Instant, fn(&ScheduledCommand) -> Instant>,
    frequency: u32,
}

impl TimeManager {
    /// Create a new time manager
    /// frequency scales command duration (default 100, higher = faster)
    pub fn new(frequency: u32) -> Self {
        Self {
            queue: OrderedList::new_by(|cmd| cmd.execute_at),
            frequency,
        }
    }

    /// Schedule a command for execution
    /// Returns when the command will execute
    pub fn schedule(&mut self, command: Command, client_id: u32) -> Instant {
        let duration_units = command.execution_time() as u64;
        // Convert duration to milliseconds: ms = (action * 1000) / frequency
        // Example: action=7, frequency=100 -> ms = 7*1000/100 = 70 ms
        let ms = (duration_units * 1000) / self.frequency as u64;
        let execute_at = Instant::now() + Duration::from_millis(ms);

        self.queue.push(ScheduledCommand {
            command,
            execute_at,
            client_id,
        });
        execute_at
    }

    /// Check current time and return all commands ready to execute
    pub fn tick(&mut self) -> Vec<ScheduledCommand> {
        let now = Instant::now();
        let mut ready = Vec::new();

        while let Some(next) = self.queue.get(0) {
            if next.execute_at <= now {
                ready.push(self.queue.remove(0));
            } else {
                break;
            }
        }

        ready
    }

    pub fn next_timeout_ms(&self) -> Option<i32> {
        let next = self.queue.get(0)?;
        let now = Instant::now();
        let delay = next.execute_at.saturating_duration_since(now);
        let ms = delay.as_millis().min(i32::MAX as u128);
        Some(ms as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn schedule_then_execute() {
        let mut manager = TimeManager::new(100);
        manager.schedule(Command::Inventory, 0);
        manager.schedule(Command::Forward, 1);

        // First tick: nothing ready yet
        let ready = manager.tick();
        assert!(ready.is_empty());

        // Wait for Inventory to be ready (~10ms)
        thread::sleep(Duration::from_millis(20));
        let ready = manager.tick();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].client_id, 0);

        // Wait for Forward to be ready (~70ms total)
        thread::sleep(Duration::from_millis(100));
        let ready = manager.tick();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].client_id, 1);
    }

    #[test]
    fn multiple_commands_same_execution_time() {
        let mut manager = TimeManager::new(100);
        manager.schedule(Command::Inventory, 0);
        manager.schedule(Command::Inventory, 1);
        manager.schedule(Command::Inventory, 2);

        thread::sleep(Duration::from_millis(20));
        let ready = manager.tick();
        assert_eq!(ready.len(), 3);
    }

    #[test]
    fn frequency_affects_timing() {
        let mut slow = TimeManager::new(50);
        let mut fast = TimeManager::new(200);

        let slow_time = slow.schedule(Command::Forward, 0);
        let fast_time = fast.schedule(Command::Forward, 0);

        // Slow should take longer
        assert!(slow_time > fast_time);
    }

    #[test]
    fn commands_ordered_by_execution_time() {
        let mut manager = TimeManager::new(100);
        manager.schedule(Command::Forward, 0);
        manager.schedule(Command::Inventory, 1);
        manager.schedule(Command::Incantation, 2);

        thread::sleep(Duration::from_millis(20));
        let ready = manager.tick();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].client_id, 1); // Inventory executes first

        thread::sleep(Duration::from_millis(100));
        let ready = manager.tick();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].client_id, 0); // Forward executes second
    }
}
