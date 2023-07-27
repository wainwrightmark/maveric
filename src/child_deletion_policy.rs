use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildDeletionPolicy {
    DeleteImmediately,
    Linger(Duration),
}