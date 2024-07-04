use std::sync::atomic::AtomicUsize;

use bevy::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct TracingPlugin;

impl Plugin for TracingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, reset_tracing);
    }
}

pub(crate) static GRAPH_UPDATES: AtomicUsize = AtomicUsize::new(0);
pub(crate) static SCHEDULED_DELETIONS: AtomicUsize = AtomicUsize::new(0);
pub(crate) static SCHEDULED_CHANGES: AtomicUsize = AtomicUsize::new(0);
pub(crate) static TRANSITIONS: AtomicUsize = AtomicUsize::new(0);

/// Counts maveric graph updates
pub fn count_graph_updates() -> usize {
    GRAPH_UPDATES.load(std::sync::atomic::Ordering::SeqCst)
}

/// Counts the nodes that have been deleted this frame
pub fn count_scheduled_deletions() -> usize {
    SCHEDULED_DELETIONS.load(std::sync::atomic::Ordering::SeqCst)
}

/// Counts the nodes that have been subject of scheduled changes this frame
pub fn count_scheduled_changes() -> usize {
    SCHEDULED_CHANGES.load(std::sync::atomic::Ordering::SeqCst)
}
/// Counts the nodes that have been affected by transitions this frame
pub fn count_transitions() -> usize {
    TRANSITIONS.load(std::sync::atomic::Ordering::SeqCst)
}

fn reset_tracing() {
    SCHEDULED_DELETIONS.store(0, std::sync::atomic::Ordering::SeqCst);
    SCHEDULED_CHANGES.store(0, std::sync::atomic::Ordering::SeqCst);
    GRAPH_UPDATES.store(0, std::sync::atomic::Ordering::SeqCst);
    TRANSITIONS.store(0, std::sync::atomic::Ordering::SeqCst);
}
