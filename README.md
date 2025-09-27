# bevy_ecs_enum_filter

Cloned [forked](https://github.com/mikkelens/bevy_enum_filter) from [bevy_enum_filter](https://github.com/MrGVSV/bevy_enum_filter) by [MrGVSV](https://github.com/MrGVSV) and edited.

Enum marker check by systems was abolished.  
Instead, ComponentHooks is used for it.

Derive macro name was changed from ```EnumFilter``` to ```EnumComponent```.  
Because bevy's Component derive macro is missing on code.

The license complies with the original crate.

## using with only bevy_ecs crate 
```toml
[dependencies]
bevy_ecs_enum_filter = { git = "https://github.com/Kyoutoza/bevy_ecs_enum_filter", branch = "0.17" }
```

```rust
use bevy_ecs_enum_filter::prelude::*;
use bevy_ecs::prelude::*;

fn main() {
    // Clone is required
    // Component is unnecessary, it will be conflict with EnumComponent
    #[derive(Clone, Debug, EnumComponent)]
    // default const STORAGE_TYPE for Component implementation is bevy_ecs(bevy::ecs)::component::StorageType::Table
    // if you need to change it, use attribute enum_component(storage_type = bevy_ecs(bevy::ecs)::component::StorageType::SparseSet)
    #[enum_component(storage_type = bevy_ecs::component::StorageType::SparseSet)]
    // default type Mutability for Component implementation is bevy_ecs(bevy::ecs)::component::Mutable
    // if you need to change it, use attribute enum_component(mutability = bevy_ecs(bevy::ecs)::component::Immutable)
    #[enum_component(mutability = bevy_ecs::component::Immutable)]
    enum TestEnum {
        A,
        B {
          v: f64,
        },
        C(i32),
    }

    let mut world = World::new();
    let entity = world.spawn(TestEnum::A).id();

    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().single(&world).is_ok());
    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::B)>>().single(&world).is_err());

    // Marker Component is removed when TestEnum is removed
    world.entity_mut(entity).remove::<TestEnum>();
    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().single(&world).is_err());
    assert!(world
        .query_filtered::<Entity, Without<Enum!(TestEnum::A)>>()
        .iter(&world)
        .any(|target| target == entity));

    // insert other TestEnum type
    world.entity_mut(entity).insert(TestEnum::B { v: 0.0 });
    assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::B)>>().single(&world).is_ok());

    // overwritten TestEnum by other variable
    world.entity_mut(entity).insert(TestEnum::C(42));
    assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::B)>>().single(&world).is_err());
    assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::C)>>().single(&world).is_ok());
}
```

## using bevy crate 
```toml
[dependencies]
bevy_ecs_enum_filter = { git = "https://github.com/Kyoutoza/bevy_ecs_enum_filter", branch = "0.17", features = [
  "bevy",
] }
```

## Bevy Compatibility

| bevy   | bevy_ecs_enum_filter |
| :----- | -------------------- |
| 0.17.x | 0.17.0-rc.2 (main branch)          |
| 0.16.x | 0.16.6 ("0.16" branch)          |
| 0.15.x | 0.1.0 ("0.15" branch)          |
