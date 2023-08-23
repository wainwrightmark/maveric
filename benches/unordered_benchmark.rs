use bevy::{prelude::*, time::TimePlugin};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use maveric::prelude::*;

criterion_group!(
    benches,
    delete_leaves_benchmark,
    create_leaves_benchmark,
    morph_leaves_benchmark
);
criterion_main!(benches);

const SIZES: [u32; 8] = [1u32, 2, 4, 8, 16, 32, 64, 128];
//const SIZES: [u32;0] = [];

fn delete_leaves_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("delete_leaves");

    for size in SIZES {
        group.throughput(criterion::Throughput::Elements((size * size) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                run_state_transition(
                    TreeState {
                        branch_count: size,
                        blue_leaf_count: size,
                        red_leaf_count: 0,
                    },
                    TreeState {
                        branch_count: size,
                        blue_leaf_count: 0,
                        red_leaf_count: 0,
                    },
                )
            })
        });
    }
}

fn create_leaves_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("create_leaves");

    for size in SIZES {
        group.throughput(criterion::Throughput::Elements((size * size) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                run_state_transition(
                    TreeState {
                        branch_count: 0,
                        blue_leaf_count: size,
                        red_leaf_count: 0,
                    },
                    TreeState {
                        branch_count: size,
                        blue_leaf_count: size,
                        red_leaf_count: 0,
                    },
                )
            })
        });
    }
}

fn morph_leaves_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("morph_leaves");

    for size in SIZES {
        group.throughput(criterion::Throughput::Elements((size * size) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                run_state_transition(
                    TreeState {
                        branch_count: size,
                        blue_leaf_count: size,
                        red_leaf_count: 0,
                    },
                    TreeState {
                        branch_count: size,
                        blue_leaf_count: 0,
                        red_leaf_count: size,
                    },
                )
            })
        });
    }
}

pub fn run_state_transition(s1: TreeState, s2: TreeState) {
    let mut app = App::new();

    app.add_plugins(TimePlugin::default());

    app.insert_resource(s1).register_maveric::<Root>();
    app.update();
    update_state(&mut app, s2);
    app.update();
}

fn update_state(app: &mut App, new_state: TreeState) {
    let mut state = app.world.resource_mut::<TreeState>();
    *state = new_state;
}

#[derive(Debug, Clone, PartialEq, Resource, Default)]
pub struct TreeState {
    branch_count: u32,
    blue_leaf_count: u32,
    red_leaf_count: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Root;

impl RootChildren for Root {
    type Context = TreeState;

    fn set_children(
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    ) {
        for x in 0..(context.branch_count) {
            commands.add_child(x, Branch, context);
        }
    }
}

impl_maveric_root!(Root);

#[derive(Debug, Clone, PartialEq, Default)]
struct Branch;

impl MavericNode for Branch {
    type Context = TreeState;


    fn set<R: MavericRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        data.ignore_args()
            .unordered_children_with_context(commands, |context, commands| {
                for x in 0..(context.blue_leaf_count) {
                    commands.add_child(x, Leaf::Blue, &());
                }

                for x in
                    (context.blue_leaf_count)..(context.blue_leaf_count + context.red_leaf_count)
                {
                    commands.add_child(x, Leaf::Red, &());
                }
            })
    }
}

#[derive(Debug, Clone, PartialEq, Component)]
enum Leaf {
    Blue,
    Red,
}

