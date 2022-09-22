use {
    crate::{clock::DEFAULT_TICKS_PER_SECOND, unchecked_div_by_const},
    std::time::Duration,
};

#[derive(Serialize, Deserialize, Clone, Debug, AbiExample, Eq, PartialEq)]
pub struct PohConfig {
    /// The target tick rate of the cluster.
    pub target_tick_duration: Duration,

    /// The target total tick count to be produced; used for testing only
    pub target_tick_count: Option<u64>,

    /// How many hashes to roll before emitting the next tick entry.
    /// None enables "Low power mode", which implies:
    /// * sleep for `target_tick_duration` instead of hashing
    /// * the number of hashes per tick will be variable
    hashes_per_tick: Option<u64>,
}

impl PohConfig {
    pub fn set_hashes_per_tick(&mut self, hashes: Option<u64>) {
        self.hashes_per_tick = hashes;
    }

    pub fn get_hashes_per_tick(&self) -> Option<u64> {
        self.hashes_per_tick
    }

    pub fn new(
        target_tick_duration: Duration,
        target_tick_count: Option<u64>,
        hashes_per_tick: Option<u64>
    ) -> Self {
        PohConfig {
            target_tick_duration,
            target_tick_count,
            hashes_per_tick,
        }
    }

    pub fn new_sleep(target_tick_duration: Duration) -> Self {
        Self {
            target_tick_duration,
            hashes_per_tick: None,
            target_tick_count: None,
        }
    }
}

impl Default for PohConfig {
    fn default() -> Self {
        Self::new_sleep(Duration::from_micros(unchecked_div_by_const!(
            1000 * 1000,
            DEFAULT_TICKS_PER_SECOND
        )))
    }
}
