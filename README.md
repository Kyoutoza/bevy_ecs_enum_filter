# bevy_ecs_enum_filter

Cloned [forked](https://github.com/mikkelens/bevy_enum_filter) from [bevy_enum_filter](https://github.com/MrGVSV/bevy_enum_filter) by [MrGVSV](https://github.com/MrGVSV) and edited.

Since bevy_ecs_enum_filter version 0.16.2,  
Enum marker check by systems was abolished.  
Instead, ComponentHooks is used for it.

The license complies with the original crate.

## using with only bevy_ecs crate 
```toml
[dependencies]
bevy_ecs_enum_filter = {git = "https://github.com/Kyoutoza/bevy_ecs_enum_filter"}
```

```rust
use bevy_ecs_enum_filter::prelude::*;
use bevy_ecs::prelude::*;

fn main() {
    // Clone is required
    // Component is unnecessary
    #[derive(Clone, Debug, EnumFilter)]
    enum TestEnum {
        A,
        B,
        C,
    }

    let mut world = World::new();
    let entity = world.spawn(TestEnum::A).id();

    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().single(&world).is_ok());
    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::B)>>().single(&world).is_err());

    // world detects Enum source is detected
    world.entity_mut(entity).remove::<TestEnum>();
    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().single(&world).is_err());
    assert!(world
        .query_filtered::<Entity, Without<Enum!(TestEnum::A)>>()
        .iter(&world)
        .any(|target| target == entity));

    // insert other TestEnum type
    world.entity_mut(entity).insert(TestEnum::B);
    assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::B)>>().single(&world).is_ok());

    // overwrite TestEnum by other type
    world.entity_mut(entity).insert(TestEnum::C);
    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::B)>>().single(&world).is_err());
    assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::C)>>().single(&world).is_ok());
}
```

## using bevy crate 
```toml
[dependencies]
bevy_ecs_enum_filter = {git = "https://github.com/Kyoutoza/bevy_ecs_enum_filter", features = ["bevy"]}
```

## Bevy Compatibility

| bevy   | bevy_ecs_enum_filter |
| :----- | -------------------- |
| 0.16.x | 0.16.2 (main)          |
| 0.15.x | 0.1.0                |
