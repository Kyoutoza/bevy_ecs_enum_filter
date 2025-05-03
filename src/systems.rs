use crate::filter_trait::EnumFilter;
use bevy_ecs::{
    prelude::{Changed, Commands, Entity, Query},
    removal_detection::RemovedComponents,
};

pub struct EnumFilterSystems;
impl EnumFilterSystems {
    /// A system that watches for changes to the given enum component.
    pub fn watch_for_enum<T: EnumFilter>(mut cmd: Commands, query: Query<(Entity, &T), Changed<T>>) {
        for (entity, value) in &query {
            T::set_marker(&mut cmd.entity(entity), value);
        }
    }

    /// A system that queries all Entities with a given enum component.
    ///
    /// Useful when you need to call `query.single()` or `query.single_mut()` since `watch_for_enum` will return 0 Entities for the first frame.
    pub fn create_marker_for_enum<T: EnumFilter>(mut cmd: Commands, query: Query<(Entity, &T)>) {
        for (entity, value) in &query {
            T::set_marker(&mut cmd.entity(entity), value);
        }
    }

    /// A system that queries all Entities with a remove enum component.
    ///
    /// Shohuld be run before watch_for_enum
    pub fn remove_marker_for_enum<T: EnumFilter>(mut cmd: Commands, mut removed: RemovedComponents<T>) {
        removed.read().for_each(|entity| {
            let Ok(mut entity_cmd) = cmd.get_entity(entity) else { return };
            T::remove_marker(&mut entity_cmd);
        });
    }
}
