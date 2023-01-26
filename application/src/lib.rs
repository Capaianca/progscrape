mod persist;
mod story;

pub use persist::{
    MemIndex, PersistError, PersistLocation, Storage, StorageSummary, StorageWriter, StoryIndex,
};
pub use story::{
    Story, StoryEvaluator, StoryIdentifier, StoryRender, StoryScoreConfig, TaggerConfig,
};

#[cfg(test)]
mod test {
    use rstest::*;
    use tracing_subscriber::EnvFilter;

    #[fixture]
    #[once]
    pub fn enable_tracing() -> bool {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        true
    }

    #[fixture]
    #[once]
    pub fn enable_slow_tests() -> bool {
        matches!(std::env::var("ENABLE_SLOW_TESTS"), Ok(_))
    }
}

macro_rules! timer_start {
    () => {
        if ::tracing::event_enabled!(tracing::Level::DEBUG) {
            Some(std::time::Instant::now())
        } else {
            None
        }
    };
}
pub(crate) use timer_start;

macro_rules! timer_end {
    ($timer:ident, $message:literal $(, $e:expr )*) => {
        if let Some(start) = $timer {
            let elapsed = start.elapsed();
            if elapsed.as_secs() >= 2 {
                tracing::info!(concat!($message, " [{:.3}s]") $(,$e)*, elapsed.as_secs_f32());
            } else {
                tracing::info!(concat!($message, " [{}ms]") $(,$e)*, elapsed.as_millis());
            }
        }
    };
}
pub(crate) use timer_end;
