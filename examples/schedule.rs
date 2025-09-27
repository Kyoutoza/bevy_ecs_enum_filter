use bevy_ecs::prelude::*;
use bevy_ecs_enum_filter::{EnumComponent, prelude::*};

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

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Press any of the following keys to spawn an entity with that value: [`A`, `B`, `C` or `Q`]");

    let mut world = World::new();
    world.insert_resource::<Input>(Input(String::new()));
    world.spawn(Player);

    let mut schedule = Schedule::new(MainSchedule);
    schedule
        .configure_sets((SystemOrder::First, SystemOrder::Second, SystemOrder::End).chain())
        .add_systems((
            spawn.run_if(resource_changed::<Input>).in_set(SystemOrder::First),
            (on_insert_a, on_insert_b, on_insert_c, on_insert_q, remove_announce).in_set(SystemOrder::Second),
        ));

    world.add_schedule(schedule);

    let mut buffer = String::new();

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

#[derive(Clone, Debug, EnumComponent)]
enum Choice {
    A,
    B,
    C,
    Q,
}

fn spawn(mut cmd: Commands, q_choice: Query<Entity, With<Player>>, mut input: ResMut<Input>) -> Result<()> {
    let mut entity_cmd = cmd.entity(q_choice.single()?);
    let binding = std::mem::take(&mut input.0).to_lowercase();
    let input = binding.trim();

    if input.is_empty() {
        return Ok(());
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

    Ok(())
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
        println!("Bye Bye!!");
        std::process::exit(0);
    }
}

fn remove_announce(mut removed: RemovedComponents<Choice>) {
    removed.read().for_each(|e| {
        println!("Removed a Choice component due to your bad choice from entity: {e:?}");
    });
}
