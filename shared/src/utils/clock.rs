use std::time::Duration;

use chrono::{TimeZone, Utc};
use slot_clock::{SlotClock, SystemTimeSlotClock};
use types::{ChainSpec, Slot};

pub struct Clock {
    clock: SystemTimeSlotClock,
}

impl Clock {
    pub fn new(spec: ChainSpec) -> Self {
        Clock {
            clock: SystemTimeSlotClock::new(
                spec.genesis_slot,
                Duration::from_secs(1606824023),
                Duration::from_secs(spec.seconds_per_slot),
            ),
        }
    }

    pub fn start_of(&self, slot: Slot) -> Option<Duration> {
        self.clock.start_of(slot)
    }

    pub fn timestamp(&self, slot: Slot) -> u64 {
        self.clock
            .start_of(slot)
            .unwrap_or(Duration::new(0, 0))
            .as_secs()
    }

    pub fn format(&self, slot: Slot) -> String {
        let timestamp = self.timestamp(slot);
        let date = Utc.timestamp(timestamp as i64, 0);

        date.format("%a, %e %b %Y %r %Z").to_string()
    }
}
