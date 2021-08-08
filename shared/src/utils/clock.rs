use std::time::Duration;

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
}
