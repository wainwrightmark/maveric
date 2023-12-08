#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NumberKey{
    Unsigned(u32),
    Signed(i32),
    Pair(u16,u16),
    Trio(u16,u16,u16)
}

impl std::fmt::Display for NumberKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsigned(u) => u.fmt(f),
            Self::Signed(i) => i.fmt(f),
            Self::Pair(x,y) => write!(f, "({x}, {y})"),
            Self::Trio(x,y,z) => write!(f, "({x}, {y}, {z})"),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChildKey {
    String(&'static str),
    Number(NumberKey),
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
        Self::Number(NumberKey::Unsigned(value))
    }
}


impl From<i32> for ChildKey {
    fn from(value: i32) -> Self {
        Self::Number(NumberKey::Signed(value))
    }
}

impl From<(u16,u16)> for ChildKey {
    fn from(value: (u16,u16)) -> Self {
        Self::Number(NumberKey::Pair(value.0, value.1))
    }
}


impl From<(u16,u16, u16)> for ChildKey {
    fn from(value: (u16,u16, u16)) -> Self {
        Self::Number(NumberKey::Trio(value.0, value.1, value.2))
    }
}

impl From<&'static str> for ChildKey {
    fn from(value: &'static str) -> Self {
        Self::String(value)
    }
}
