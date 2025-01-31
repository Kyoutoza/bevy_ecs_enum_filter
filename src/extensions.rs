use crate::filter_trait::EnumFilter;
use bevy_app::{App, PostStartup, Update};
use bevy_ecs::schedule::IntoSystemConfigs;

/// Extension trait for [`App`] that enables adding an [enum filter].
///
/// [enum filter]: crate::EnumFilter
pub trait AddEnumFilter {
    /// Register an enum filter.
    ///
    /// This will internally add a system to the [`PostUpdate`] stage that finds all entities with
    /// a component of `T` and automatically manage their respective markers.
    ///
    /// [`PostUpdate`]: CoreStage::PostUpdate
    fn add_enum_filter<T: EnumFilter>(&mut self) -> &mut Self;
}
impl AddEnumFilter for App {
    fn add_enum_filter<T: EnumFilter>(&mut self) -> &mut Self {
        use crate::systems::EnumFilterSystems;
        self.add_systems(PostStartup, EnumFilterSystems::create_marker_for_enum::<T>).add_systems(
            Update,
            (EnumFilterSystems::remove_marker_for_enum::<T>, EnumFilterSystems::watch_for_enum::<T>).chain(),
        )
    }
}
