//! RVA mappings for ELDEN RING 2.6.2.0 (WW / EN).

use super::RvaBundle;

/// The RVAs for the 2.6.2.0 executable.
pub const RVAS: RvaBundle = RvaBundle {
    dluid_keyboard_device_should_block_input: 0x1f6d640,
    dluid_mouse_device_should_block_input: 0x1f6dd20,
    dluid_pad_device_should_block_input: 0x1f6bad0,
    equip_game_data_deserialize: 0x248520,
    equip_game_data_serialize: 0x248fd0,
};
