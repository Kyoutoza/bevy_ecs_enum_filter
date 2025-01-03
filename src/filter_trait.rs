use bevy_ecs::{prelude::Component, system::EntityCommands};

/// A trait used to denote an enum as "filterable".
pub trait EnumFilter: Component {
    /// Set the marker on the given entity to the given enum value.
    fn set_marker(commands: &mut EntityCommands, value: &Self);
    fn remove_marker(commands: &mut EntityCommands);
}
