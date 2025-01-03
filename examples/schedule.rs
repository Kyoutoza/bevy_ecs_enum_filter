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
    world.spawn(Player);

    let mut schedule = Schedule::new(MainSchedule);
    schedule
        .configure_sets((SystemOrder::First, SystemOrder::Second, SystemOrder::End).chain())
        .add_systems((
            spawn.run_if(resource_changed::<Input>).in_set(SystemOrder::First),
            (on_insert_a, on_insert_b, on_insert_c, on_insert_q).in_set(SystemOrder::Second),
            register_enum_filter!(Choice).in_set(SystemOrder::End),
        ));

    world.add_schedule(schedule);

    loop {
        std::io::stdin().read_line(&mut buffer)?;
        world.resource_mut::<Input>().0 = std::mem::take(&mut buffer);

        for _ in 0..10 {
            world.run_schedule(MainSchedule);
        }

        print!("\x1b[2J\x1b[1;1H");
    }
}

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug, EnumFilter)]
enum Choice {
    A,
    B,
    C,
    Q,
}

fn spawn(mut cmd: Commands, q_choice: Query<Entity, With<Player>>, mut input: ResMut<Input>) {
    let mut entity_cmd = cmd.entity(q_choice.single());
    let binding = std::mem::take(&mut input.0).to_lowercase();
    let input = binding.trim();

    if input.is_empty() {
        entity_cmd.remove::<Choice>();
        return;
    }

    match input {
        "a" => entity_cmd.insert(Choice::A),
        "b" => entity_cmd.insert(Choice::B),
        "c" => entity_cmd.insert(Choice::C),
        "q" => entity_cmd.insert(Choice::Q),
        _ => {
            println!("Bad Choice!");
            entity_cmd.remove::<Choice>()
        }
    };
}

fn on_insert_a(query: Query<Entity, Changed<Enum!(Choice::A)>>) {
    if !query.is_empty() {
        println!("Inserted `Choice::A`!");
    }
}

fn on_insert_b(query: Query<Entity, Changed<Enum!(Choice::B)>>) {
    if !query.is_empty() {
        println!("Inserted `Choice::B`!");
    }
}

fn on_insert_c(query: Query<Entity, Changed<Enum!(Choice::C)>>) {
    if !query.is_empty() {
        println!("Inserted `Choice::C`!");
    }
}

fn on_insert_q(query: Query<Entity, Changed<Enum!(Choice::Q)>>) {
    if !query.is_empty() {
        println!("Ultra Bad Choice!!! Bye Bye!!");
        exit(0);
    }
}
