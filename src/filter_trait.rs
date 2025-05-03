#[cfg(feature = "bevy")]
/// A trait used to denote an enum as "filterable".
pub trait EnumFilter: bevy::ecs::component::Component {
    /// Set the marker on the given entity to the given enum value.
    fn set_marker(cmd: &mut bevy::prelude::EntityCommands, value: &Self);
    fn remove_marker(cmd: &mut bevy::prelude::EntityCommands);
}

#[cfg(feature = "ecs")]
/// A trait used to denote an enum as "filterable".
pub trait EnumFilter: bevy_ecs::prelude::Component {
    /// Set the marker on the given entity to the given enum value.
    fn set_marker(cmd: &mut bevy_ecs::system::EntityCommands, value: &Self);
    fn remove_marker(cmd: &mut bevy_ecs::system::EntityCommands);
}
