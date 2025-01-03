use bevy_ecs::prelude::*;
use bevy_ecs_enum_filter::prelude::*;
use std::process::exit;

#[derive(bevy_ecs::schedule::ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MainSchedule;

#[derive(Clone, Debug, Hash, PartialEq, Eq, SystemSet)]
pub(crate) enum SystemOrder {
    First,
    Second,
    End,
}

#[derive(Resource, Clone, Debug)]
struct Input(pub String);

/// example for instant registration
macro_rules! register_enum_filter {
    ($ty:ty) => {
        (EnumFilterSystems::remove_marker_for_enum::<$ty>, EnumFilterSystems::watch_for_enum::<$ty>).chain()
    };
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Press any of the following keys to spawn an entity with that value: [`A`, `B`, `C` or `Q`]");

    let mut buffer = String::new();

    let mut world = World::new();
    world.insert_resource::<Input>(Input(String::new()));

    let mut schedule = Schedule::new(MainSchedule);
    schedule.add_systems((
        spawn.run_if(resource_changed::<Input>).in_set(SystemOrder::First),
        (on_spawn_a, on_spawn_b, on_spawn_c, on_spawn_q).in_set(SystemOrder::Second),
        register_enum_filter!(Choice).in_set(SystemOrder::End),
    ));

    world.add_schedule(schedule);

    loop {
        std::io::stdin().read_line(&mut buffer)?;
        world.resource_mut::<Input>().0 = std::mem::take(&mut buffer);
        for _ in 0..10 {
            world.run_schedule(MainSchedule);
        }
    }
}

#[derive(Component, Debug, EnumFilter)]
enum Choice {
    A,
    B,
    C,
    Q,
}

fn spawn(mut commands: Commands, mut input: ResMut<Input>) {
    let input = std::mem::take(&mut input.0).to_lowercase();

    if input.contains('a') {
        commands.spawn((Choice::A,));
    }
    if input.contains('b') {
        commands.spawn((Choice::B,));
    }
    if input.contains('c') {
        commands.spawn((Choice::C,));
    }
    if input.contains('q') {
        commands.spawn((Choice::Q,));
    }
}

fn on_spawn_a(query: Query<Entity, Added<Enum!(Choice::A)>>) {
    for _ in &query {
        println!("Spawned entity with `Choice::A`!");
    }
}

fn on_spawn_b(query: Query<Entity, Added<Enum!(Choice::B)>>) {
    for _ in &query {
        println!("Spawned entity with `Choice::B`!");
    }
}

fn on_spawn_c(query: Query<Entity, Added<Enum!(Choice::C)>>) {
    for _ in &query {
        println!("Spawned entity with `Choice::C`!");
    }
}

fn on_spawn_q(query: Query<Entity, Added<Enum!(Choice::Q)>>) {
    for _ in &query {
        println!("Spawned entity with `Choice::Q`! So, bye bye!!");
        exit(0);
    }
}
