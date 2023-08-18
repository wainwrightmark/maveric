use bevy::{prelude::*, time::TimePlugin};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use state_hierarchy::prelude::*;


fn reverse_leaves_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("reverse_leaves");

    for size in [1u32, 2, 4, 8, 16, 32, 64, 128] {
        group.throughput(criterion::Throughput::Elements((size * size) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                run_state_transition(
                    TreeState((0..size).collect()),
                    TreeState((0..size).rev().collect()),
                    LingerState(false)
                )
            })
        });
    }
}

criterion_group!(
    benches,
    reverse_leaves_benchmark
);
criterion_main!(benches);

pub fn run_state_transition(s1: TreeState, s2: TreeState, linger_state: LingerState) {
    let mut app = App::new();

    app.add_plugins(TimePlugin::default());

    app.insert_resource(s1)
        .insert_resource(linger_state)
        .register_state_hierarchy::<Root>();
    app.update();
    update_state(&mut app, s2);
    app.update();
}

fn update_state(app: &mut App, new_state: TreeState) {
    let mut state = app.world.resource_mut::<TreeState>();
    *state = new_state;
}
#[derive(Debug, Clone, PartialEq, Resource, Default)]
pub struct TreeState(Vec<u32>);

#[derive(Debug, Clone, PartialEq, Resource, Default)]
pub struct LingerState(bool);

#[derive(Debug, Clone, PartialEq, Default)]
struct Root;

impl HasContext for Root {
    type Context = NC2<TreeState, LingerState>;
}

impl ChildrenAspect for Root {
    fn set_children(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    ) {
        commands.add_child("branch", Branch, context);
    }
}

impl_hierarchy_root!(Root);

#[derive(Debug, Clone, PartialEq, Default)]
struct Branch;

impl HasContext for Branch {
    type Context = NC2<TreeState, LingerState>;
}

impl ChildrenAspect for Branch {
    fn set_children<'r>(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        for &number in context.0 .0.iter() {
            let linger = context.1 .0;
            commands.add_child(number, Leaf { number, linger }, &());
        }
    }
}

impl StaticComponentsAspect for Branch {
    type B = ();

    fn get_bundle() -> Self::B {}
}

#[derive(Debug, Clone, PartialEq)]
struct Leaf {
    number: u32,
    linger: bool,
}

impl HasNoContext for Leaf {}

impl HasNoChildren for Leaf {}

impl ComponentsAspect for Leaf {
    fn set_components<'r>(
        &self,
        _previous: Option<&Self>,
        _context: &<Self::Context as NodeContext>::Wrapper<'r>,
        _commands: &mut impl ComponentCommands,
        _event: SetComponentsEvent,
    ) {
    }

    fn on_deleted<'r>(&self, _commands: &mut impl ComponentCommands) -> DeletionPolicy {
        if self.linger {
            DeletionPolicy::linger(1.0)
        } else {
            DeletionPolicy::DeleteImmediately
        }
    }
}
