#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum UserAttentionType {
    Critical,
    #[default]
    Informational,
}
