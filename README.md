# bevy_ecs_enum_filter

cloned [forked](https://github.com/mikkelens/bevy_enum_filter) from [bevy_enum_filter](https://github.com/MrGVSV/bevy_enum_filter) by [MrGVSV](https://github.com/MrGVSV) and edited for bevy_ecs crate without bevy_app crate (mainly for me).

remove_marker_for_enum is added, which retrieves the removed enum component and removes the marker.  
However, needs to take care in the system calling order.

The license complies with the original crate.

```rust
use bevy_ecs_enum_filter::prelude::*;
use bevy_ecs::prelude::*;

fn main() {
    #[derive(Component, Debug, EnumFilter)]
    enum TestEnum {
        A,
        B,
        C,
    }

    let mut world = World::new();
    let update_systems = [
        // remove_marker_for_enum should be run before watch_for_enum
        world.register_system(EnumFilterSystems::remove_marker_for_enum::<TestEnum>),
        world.register_system(EnumFilterSystems::watch_for_enum::<TestEnum>),
    ];

    let entity = world.spawn(TestEnum::A).id();

    // update world
    update_systems
        .into_iter()
        .for_each(|id| world.run_system(id).unwrap());

    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().get_single(&world).is_ok());
    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::B)>>().get_single(&world).is_err());

    // world detects Enum source is detected
    world.entity_mut(entity).remove::<TestEnum>();
    update_systems
        .into_iter()
        .for_each(|id| world.run_system(id).unwrap());

    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().get_single(&world).is_err());
    assert!(world
        .query_filtered::<Entity, Without<Enum!(TestEnum::A)>>()
        .iter(&world)
        .any(|target| target == entity));

    // insert other TestEnum type
    world.entity_mut(entity).insert(TestEnum::B);
    update_systems
        .into_iter()
        .for_each(|id| world.run_system(id).unwrap());

    assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::B)>>().get_single(&world).is_ok());

    // overwrite TestEnum by other type
    world.entity_mut(entity).insert(TestEnum::C);
    update_systems
        .into_iter()
        .for_each(|id| world.run_system(id).unwrap());

    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::B)>>().get_single(&world).is_err());
    assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::C)>>().get_single(&world).is_ok());

    // use markers directly
    {
        // EnumFilter proc macro generate a mod
        use test_enum_filters::C;

        assert!(world.query_filtered::<Entity, With<test_enum_filters::B>>().get_single(&world).is_err());
        assert!(world.query_filtered::<Entity, Added<C>>().get_single(&world).is_ok());
    }
}
```

## Bevy Compatibility

| bevy   | bevy_ecs_enum_filter |
| :----- | -------------------- |
| 0.15.x | 0.1.0 (main)         |
