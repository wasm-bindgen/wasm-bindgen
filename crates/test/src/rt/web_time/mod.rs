mod instant;
mod js;
mod system_time;

pub use instant::Instant;
pub use system_time::SystemTime;
pub const UNIX_EPOCH: SystemTime = SystemTime::UNIX_EPOCH;
