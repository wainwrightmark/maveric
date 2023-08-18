use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeletionPolicy {
    DeleteImmediately,
    Linger(Duration),
}


impl DeletionPolicy{
    pub fn linger(secs: f32)-> Self{
        if secs <= 0.0{
            return DeletionPolicy::DeleteImmediately;
        }
        return DeletionPolicy::Linger(Duration::from_secs_f32(secs));
    }
}