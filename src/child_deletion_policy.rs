use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeletionPolicy {
    DeleteImmediately,
    Linger(Duration),
}
