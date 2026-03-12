#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum ImePurpose {
    #[default]
    Normal,
    Password,
    Terminal,
    Number,
    Phone,
    Url,
    Email,
    Pin,
    Date,
    Time,
    DateTime,
}
