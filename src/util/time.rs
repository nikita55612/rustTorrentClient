use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn unix_now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::default())
}
