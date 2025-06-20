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

    #[derive(Clone, Debug, EnumFilter)]
    enum TestEnum {
        A,
        B,
        C,
    }
    impl bevy_ecs::component::Component for TestEnum {
        const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
        type Mutability = bevy_ecs::component::Mutable;
        fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
            hooks
                .on_add(|mut world, ctx| {
                    println!("on_add");
                    let enum_comp = world.get::<TestEnum>(ctx.entity).unwrap().clone();
                    let mut cmd = world.commands();
                    cmd.queue(move |world: &mut World| {
                        let mut entity_mut = world.entity_mut(ctx.entity);
                        match enum_comp {
                            TestEnum::A => {
                                if !entity_mut.contains::<test_enum_filters::A>() {
                                    entity_mut.insert(test_enum_filters::A);
                                }
                            }
                            TestEnum::B => {
                                if !entity_mut.contains::<test_enum_filters::B>() {
                                    entity_mut.insert(test_enum_filters::B);
                                }
                            }
                            TestEnum::C => {
                                if !entity_mut.contains::<test_enum_filters::C>() {
                                    entity_mut.insert(test_enum_filters::C);
                                }
                            }
                        }
                    });
                })
                .on_insert(|mut world, ctx| {
                    println!("on_insert");
                    let enum_comp = world.get::<TestEnum>(ctx.entity).unwrap().clone();
                    let mut cmd = world.commands();
                    cmd.queue(move |world: &mut World| {
                        let mut entity_mut = world.entity_mut(ctx.entity);
                        match enum_comp {
                            TestEnum::A => {
                                if !entity_mut.contains::<test_enum_filters::A>() {
                                    entity_mut.insert(test_enum_filters::A);
                                }
                            }
                            TestEnum::B => {
                                if !entity_mut.contains::<test_enum_filters::B>() {
                                    entity_mut.insert(test_enum_filters::B);
                                }
                            }
                            TestEnum::C => {
                                if !entity_mut.contains::<test_enum_filters::C>() {
                                    entity_mut.insert(test_enum_filters::C);
                                }
                            }
                        }
                    })
                })
                .on_replace(|mut world, ctx| {
                    println!("on_replace");
                    let enum_comp = world.get::<TestEnum>(ctx.entity).unwrap().clone();
                    let mut cmd = world.commands();
                    let mut cmd = cmd.entity(ctx.entity);
                    match enum_comp {
                        TestEnum::A => cmd.remove::<test_enum_filters::A>(),
                        TestEnum::B => cmd.remove::<test_enum_filters::B>(),
                        TestEnum::C => cmd.remove::<test_enum_filters::C>(),
                    };
                })
                .on_remove(|mut world, ctx| {
                    println!("on_remove");
                    let enum_comp = world.get::<TestEnum>(ctx.entity).unwrap().clone();
                    let mut cmd = world.commands();
                    let mut cmd = cmd.entity(ctx.entity);
                    match enum_comp {
                        TestEnum::A => cmd.remove::<test_enum_filters::A>(),
                        TestEnum::B => cmd.remove::<test_enum_filters::B>(),
                        TestEnum::C => cmd.remove::<test_enum_filters::C>(),
                    };
                });
        }
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
        assert!(world
            .query_filtered::<Entity, Without<Enum!(TestEnum::A)>>()
            .iter(&world)
            .any(|target| target == entity));
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
