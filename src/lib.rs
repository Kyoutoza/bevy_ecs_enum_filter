#![doc = include_str!("../README.md")]

use bevy_ecs::prelude::Component;
pub use bevy_ecs_enum_filter_derive::{Enum, EnumFilter};

pub mod prelude {
    pub use crate::EnumFilter;
    pub use bevy_ecs_enum_filter_derive::Enum;
}

/// A trait used to denote an enum as "filterable".
pub trait EnumFilter: Clone + Component {}

#[allow(unused)]
#[derive(Clone, EnumFilter)]
enum Sample {
    A,
    B,
    C,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;

    #[derive(Clone, Debug, EnumFilter)]
    enum TestEnum {
        A,
        B,
        C,
    }

    #[test]
    fn test_observer() {
        use test_enum_filters::*;

        #[derive(Event)]
        struct TriTest;

        fn on_test(tri: Trigger<TriTest>, q: Query<Entity, With<Enum!(TestEnum::B)>>) {
            assert!(q.get(tri.target()).is_ok());
        }

        fn sys_trigger(mut cmd: Commands, q: Query<Entity>) {
            let entity = q.iter().last().unwrap();
            cmd.entity(entity).insert(TestEnum::B).trigger(TriTest);
        }

        let mut world = World::new();
        world.add_observer(on_test);

        let update_systems = [world.register_system(sys_trigger)];

        world.spawn_empty();

        update_systems.into_iter().for_each(|id| world.run_system(id).unwrap());

        assert!(world.query_filtered::<Entity, With<B>>().single(&world).is_ok());
    }

    #[test]
    fn test_abbr() {
        use test_enum_filters::*;

        let mut world = World::new();
        let entity = world.spawn(TestEnum::A).id();

        assert!(world.query_filtered::<Entity, With<A>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Added<A>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Added<B>>().single(&world).is_err());

        world.entity_mut(entity).remove::<TestEnum>();

        assert!(world.query_filtered::<Entity, With<A>>().single(&world).is_err());
        assert!(world.query_filtered::<Entity, Without<A>>().iter(&world).any(|target| target == entity));
        assert!(world.query_filtered::<Entity, Added<A>>().single(&world).is_err());
    }

    #[test]
    fn test_filter() {
        let mut world = World::new();
        let entity = world.spawn(TestEnum::A).id();

        assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::A)>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::B)>>().single(&world).is_err());

        world.entity_mut(entity).remove::<TestEnum>();

        assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::A)>>().single(&world).is_err());
        assert!(
            world
                .query_filtered::<Entity, Without<Enum!(TestEnum::A)>>()
                .iter(&world)
                .any(|target| target == entity)
        );
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::A)>>().single(&world).is_err());

        world.entity_mut(entity).insert(TestEnum::B);

        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::A)>>().single(&world).is_err());
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::B)>>().single(&world).is_ok());

        world.entity_mut(entity).insert(TestEnum::C);

        assert!(world.query_filtered::<Entity, With<Enum!(TestEnum::B)>>().iter(&world).len() == 0);
        assert!(world.query_filtered::<Entity, Changed<Enum!(TestEnum::A)>>().single(&world).is_err());
        assert!(world.query_filtered::<Entity, Added<Enum!(TestEnum::C)>>().single(&world).is_ok());
        assert!(world.query_filtered::<Entity, Changed<Enum!(TestEnum::C)>>().single(&world).is_ok());
    }
}
