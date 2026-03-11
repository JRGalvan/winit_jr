#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ActivationToken {
    pub(crate) token: String,
}

impl ActivationToken {
    pub fn from_raw(token: String) -> Self {
        Self { token }
    }

    pub fn into_raw(self) -> String {
        self.token
    }

    pub fn as_raw(&self) -> &str {
        &self.token
    }
}
