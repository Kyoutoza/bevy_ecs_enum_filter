use bevy_ecs::prelude::*;
use bevy_ecs_enum_filter::prelude::*;
use std::process::exit;

#[derive(bevy_ecs::schedule::ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MainSchedule;

macro_rules! register_enum_filter {
    ($ty:ty) => {
        (EnumFilterSystems::remove_marker_for_enum::<$ty>, EnumFilterSystems::watch_for_enum::<$ty>).chain()
    };
}

fn main() {
    println!("Press any of the following keys to spawn an entity with that value: [`A`, `B`, or `C`]");

    let mut world = World::new();
    let mut schedule = Schedule::new(MainSchedule);
    schedule.add_systems(register_enum_filter!(Choice));

    loop {
        world.flush();
    }
}

#[derive(Component, Debug, EnumFilter)]
enum Choice {
    A,
    B,
    C,
}

// fn spawn(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
//     if input.just_pressed(KeyCode::KeyA) {
//         commands.spawn((Choice::A,));
//     }
//     if input.just_pressed(KeyCode::KeyB) {
//         commands.spawn((Choice::B,));
//     }
//     if input.just_pressed(KeyCode::KeyC) {
//         commands.spawn((Choice::C,));
//     }
//     if input.just_pressed(KeyCode::KeyESC) {
//         exit(0);
//     }
// }

// fn on_spawn_a(query: Query<Entity, Added<Enum!(Choice::A)>>) {
//     for _ in &query {
//         println!("Spawned entity with `Choice::A`!");
//     }
// }

// fn on_spawn_b(query: Query<Entity, Added<Enum!(Choice::B)>>) {
//     for _ in &query {
//         println!("Spawned entity with `Choice::B`!");
//     }
// }

// fn on_spawn_c(query: Query<Entity, Added<Enum!(Choice::C)>>) {
//     for _ in &query {
//         println!("Spawned entity with `Choice::C`!");
//     }
// }
