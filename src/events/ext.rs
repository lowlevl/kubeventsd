use chrono::{DateTime, Utc};

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::super::Event {}
}

pub trait EventExt: sealed::Sealed {
    fn event_time(&self) -> DateTime<Utc>;
}

impl EventExt for super::Event {
    fn event_time(&self) -> DateTime<Utc> {
        self.event_time
            .as_ref()
            .map(|time| time.0)
            .or(self.last_timestamp.as_ref().map(|time| time.0))
            .or(self.first_timestamp.as_ref().map(|time| time.0))
            .unwrap_or_default()
    }
}
