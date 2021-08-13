use std::{ops::Sub, time::Duration};

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

    pub fn format(&self, slot: Slot) -> String {
        let timestamp = self
            .clock
            .start_of(slot)
            .unwrap_or(Duration::new(0, 0))
            .as_secs();
        let date = Utc.timestamp(timestamp as i64, 0);

        date.format("%a, %e %b %Y %r %Z").to_string()
    }

    pub fn ago(&self, slot: Slot) -> String {
        let f = timeago::Formatter::new();
        let now = self.clock.now_duration();
        let duration = self.clock.start_of(slot);

        match now {
            Some(now) => match duration {
                Some(duration) => f.convert(now.sub(duration)),
                None => String::new(),
            },
            None => String::new(),
        }
    }
}
