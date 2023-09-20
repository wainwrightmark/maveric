#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChildKey {
    Number(u32),
    String(&'static str),
}

impl std::fmt::Display for ChildKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(u) => u.fmt(f),
            Self::String(s) => s.fmt(f),
        }
    }
}

impl From<u32> for ChildKey {
    fn from(value: u32) -> Self {
        Self::Number(value)
    }
}

impl From<&'static str> for ChildKey {
    fn from(value: &'static str) -> Self {
        Self::String(value)
    }
}
