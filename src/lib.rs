//! API types shared between game-engine (host) and WASM addons (guest).

/// Frame identifier.
pub type FrameId = u64;

/// Event types that addons can register for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum GameEvent {
    PlayerLogin = 0,
    UnitHealth = 1,
    UnitPowerUpdate = 2,
    PlayerEnteringWorld = 3,
    ChatMessage = 4,
}

/// Host function IDs for the ABI boundary.
/// These match the function names exported by the engine to WASM modules.
pub mod host_functions {
    pub const CREATE_FRAME: &str = "host_create_frame";
    pub const SET_TEXT: &str = "host_set_text";
    pub const SET_SIZE: &str = "host_set_size";
    pub const SET_POINT: &str = "host_set_point";
    pub const SHOW: &str = "host_show";
    pub const HIDE: &str = "host_hide";
    pub const SET_ALPHA: &str = "host_set_alpha";
    pub const REGISTER_EVENT: &str = "host_register_event";
}

/// Addon lifecycle entry points that the engine calls.
pub mod entry_points {
    pub const ON_LOAD: &str = "on_load";
    pub const ON_UNLOAD: &str = "on_unload";
    pub const ON_EVENT: &str = "on_event";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_event_repr() {
        assert_eq!(GameEvent::PlayerLogin as u32, 0);
        assert_eq!(GameEvent::UnitHealth as u32, 1);
    }

    #[test]
    fn host_function_names() {
        assert_eq!(host_functions::CREATE_FRAME, "host_create_frame");
        assert_eq!(host_functions::SET_TEXT, "host_set_text");
    }

    #[test]
    fn entry_point_names() {
        assert_eq!(entry_points::ON_LOAD, "on_load");
        assert_eq!(entry_points::ON_UNLOAD, "on_unload");
        assert_eq!(entry_points::ON_EVENT, "on_event");
    }
}
