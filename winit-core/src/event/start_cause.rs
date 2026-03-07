use std::time;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartCause {
    ResumeTimeReached {
        start: time::Instant,
        requested_resume: time::Instant,
    },
    WaitCancelled {
        start: time::Instant,
        requested_resume: Option<time::Instant>,
    },
    Poll,
    Init,
}
