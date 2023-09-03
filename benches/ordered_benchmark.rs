use bevy::{prelude::*, time::TimePlugin};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use maveric::prelude::*;

fn reverse_leaves_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("reverse_leaves");

    for size in [1u32, 2, 4, 8, 16, 32, 64, 128] {
        group.throughput(criterion::Throughput::Elements((size * size) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                run_state_transition(
                    TreeState((0..size).collect()),
                    TreeState((0..size).rev().collect()),
                    LingerState(false),
                )
            })
        });
    }
}

criterion_group!(benches, reverse_leaves_benchmark);
criterion_main!(benches);

pub fn run_state_transition(s1: TreeState, s2: TreeState, linger_state: LingerState) {
    let mut app = App::new();

    app.add_plugins(TimePlugin);

    app.insert_resource(s1)
        .insert_resource(linger_state)
        .register_maveric::<Root>();
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

impl MavericRootChildren for Root {
    type Context = NC2<TreeState, LingerState>;

    fn set_children(
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    ) {
        commands.add_child("branch", Branch, context);
    }
}

impl_maveric_root!(Root);

#[derive(Debug, Clone, PartialEq, Default)]
struct Branch;

impl MavericNode for Branch {
    type Context = NC2<TreeState, LingerState>;

    fn set_components(_commands: SetComponentCommands<Self, Self::Context>) {}

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context, R>) {
        commands
            .ignore_node()
            .ordered_children_with_context(|context, commands| {
                for &number in context.0 .0.iter() {
                    let linger = context.1 .0;
                    commands.add_child(number, Leaf { number, linger }, &());
                }
            });
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Leaf {
    number: u32,
    linger: bool,
}

impl MavericNode for Leaf {
    type Context = NoContext;

    fn set_components(_commands: SetComponentCommands<Self, Self::Context>) {}

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}

    fn on_deleted<'r>(&self, _commands: &mut ComponentCommands) -> DeletionPolicy {
        if self.linger {
            DeletionPolicy::linger(1.0)
        } else {
            DeletionPolicy::DeleteImmediately
        }
    }
}
