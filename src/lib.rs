#![doc = include_str!("../README.md")]

#[cfg(feature = "app")]
mod extensions;
#[cfg(feature = "bevy")]
mod extensions;
mod filter_trait;
mod systems;

pub use bevy_ecs_enum_filter_derive::{Enum, EnumFilter};
pub use filter_trait::EnumFilter;

pub mod prelude {
    pub use super::filter_trait::EnumFilter;
    #[cfg(feature = "bevy")]
    pub use crate::extensions::AddEnumFilter;
    #[cfg(feature = "app")]
    pub use crate::extensions::AddEnumFilter;
    #[cfg(feature = "macro")]
    pub use crate::register_enum_filter_systems;
    pub use crate::systems::EnumFilterSystems;
    pub use bevy_ecs_enum_filter_derive::{Enum, EnumFilter};
}

#[cfg(feature = "macro")]
/// instant registration
#[macro_export]
macro_rules! register_enum_filter_systems {
    ($ty:ty) => {
        (EnumFilterSystems::remove_marker_for_enum::<$ty>, EnumFilterSystems::watch_for_enum::<$ty>).chain()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use bevy_ecs::prelude::*;

    #[derive(Component, Debug, EnumFilter)]
    enum TestEnum {
        A,
        B,
        C,
    }

    #[test]
    fn test_abbr() {
        use test_enum_filters::*;

        let mut world = World::new();
        let update_systems = [
            world.register_system(EnumFilterSystems::remove_marker_for_enum::<TestEnum>),
            world.register_system(EnumFilterSystems::watch_for_enum::<TestEnum>),
        ];

        let entity = world.spawn(TestEnum::A).id();

        update_systems.into_iter().for_each(|id| world.run_system(id).unwrap());

        assert!(world.query_filtered::<Entity, With<A>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Added<A>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Added<B>>().single(&world).is_err());

        world.entity_mut(entity).remove::<TestEnum>();
        update_systems
            .into_iter()
            .for_each(|id| world.run_system(id).unwrap_or_else(|e| panic!("{e}")));

        assert!(world.query_filtered::<Entity, With<A>>().single(&world).is_err());
        assert!(world.query_filtered::<Entity, Without<A>>().iter(&world).any(|target| target == entity));
        assert!(world.query_filtered::<Entity, Added<A>>().single(&world).is_err());
    }

    #[test]
    fn test_filter() {
        let mut world = World::new();
        let update_systems = [
            // remove_marker_for_enum should be run before watch_for_enum
            world.register_system(EnumFilterSystems::remove_marker_for_enum::<TestEnum>),
            world.register_system(EnumFilterSystems::watch_for_enum::<TestEnum>),
        ];

        let entity = world.spawn(TestEnum::A).id();

        update_systems.into_iter().for_each(|id| world.run_system(id).unwrap());

        assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::A)>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::B)>>().single(&world).is_err());

        world.entity_mut(entity).remove::<TestEnum>();
        update_systems
            .into_iter()
            .for_each(|id| world.run_system(id).unwrap_or_else(|e| panic!("{e}")));

        assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().single(&world).is_err());
        assert!(world
            .query_filtered::<Entity, Without<Enum!(TestEnum::A)>>()
            .iter(&world)
            .any(|target| target == entity));
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::A)>>().single(&world).is_err());

        world.entity_mut(entity).insert(TestEnum::B);
        update_systems
            .into_iter()
            .for_each(|id| world.run_system(id).unwrap_or_else(|e| panic!("{e}")));

        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::A)>>().single(&world).is_err());
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::B)>>().single(&world).is_ok());

        world.entity_mut(entity).insert(TestEnum::C);
        update_systems
            .into_iter()
            .for_each(|id| world.run_system(id).unwrap_or_else(|e| panic!("{e}")));

        assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::B)>>().iter(&world).len() == 0);
        assert!(world.query_filtered::<Entity, Changed<Enum!(TestEnum::A)>>().single(&world).is_err());
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::C)>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Changed<Enum!(TestEnum::C)>>().single(&world).is_ok());
    }

    #[test]
    fn test_order() {
        let mut world = World::new();
        let reserved_update_systems = [
            // switched watch_for_enum and remove_marker_for_enum order
            world.register_system(EnumFilterSystems::watch_for_enum::<TestEnum>),
            world.register_system(EnumFilterSystems::remove_marker_for_enum::<TestEnum>),
        ];

        let entity = world.spawn(TestEnum::A).id();

        [world.register_system(EnumFilterSystems::create_marker_for_enum::<TestEnum>)]
            .into_iter()
            .for_each(|id| world.run_system(id).unwrap_or_else(|e| panic!("{e}")));

        world.entity_mut(entity).remove::<TestEnum>();
        world.entity_mut(entity).insert(TestEnum::C);
        reserved_update_systems
            .into_iter()
            .for_each(|id| world.run_system(id).unwrap_or_else(|e| panic!("{e}")));

        assert!(world.query_filtered::<Entity, Changed<Enum!(TestEnum::B)>>().single(&world).is_err());
        assert!(world.query_filtered::<Entity, Changed<Enum!(TestEnum::A)>>().single(&world).is_err());

        // failed!
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::C)>>().single(&world).is_err());
        // failed!
        assert!(world.query_filtered::<Entity, Changed<Enum!(TestEnum::C)>>().single(&world).is_err());
    }
}
