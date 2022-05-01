use ::network::{DeltaSnapshot, Message};

use std::sync::Once;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn network_init() {
    static SET_HOOK: Once = Once::new();

    SET_HOOK.call_once(|| {
        use wasm_bindgen_console_logger::DEFAULT_LOGGER;

        console_error_panic_hook::set_once();

        log::set_logger(&DEFAULT_LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Info);
    });
}

#[wasm_bindgen]
pub struct MessageWrapper(Message);

#[wasm_bindgen]
impl MessageWrapper {
    pub fn is_bios(&self) -> bool {
        match self.0 {
            Message::Bios(_) => true,
            _ => false,
        }
    }

    pub fn get_bios(self) -> js_sys::Uint8Array {
        match self.0 {
            Message::Bios(bios) => js_sys::Uint8Array::from(bios.as_ref()),
            _ => unreachable!("Call `is_bios` first."),
        }
    }

    pub fn is_rom(&self) -> bool {
        match self.0 {
            Message::Rom(_) => true,
            _ => false,
        }
    }

    pub fn get_rom(self) -> js_sys::Uint8Array {
        match self.0 {
            Message::Rom(rom) => js_sys::Uint8Array::from(rom.as_ref()),
            _ => unreachable!("Call `is_rom` first."),
        }
    }

    pub fn is_play(&self) -> bool {
        match self.0 {
            Message::Play(_) => true,
            _ => false,
        }
    }

    pub fn get_play(self) -> js_sys::Uint8Array  {
        match self.0 {
            Message::Play(snapshot) => js_sys::Uint8Array::from(snapshot.as_ref()),
            _ => unreachable!("Call `is_play` first."),
        }
    }

    pub fn is_delta_snapshot(&self) -> bool {
        match self.0 {
            Message::DeltaSnapshot(_) => true,
            _ => false,
        }
    }

    pub fn get_delta_snapshot(self, old_array: &[u8]) -> js_sys::Uint8Array  {
        match self.0 {
            Message::DeltaSnapshot(delta_snapshot) => js_sys::Uint8Array::from(delta_snapshot.apply(old_array).as_ref()),
            _ => unreachable!("Call `is_delta_snapshot` first."),
        }
    }

    pub fn is_snapshot(&self) -> bool {
        match self.0 {
            Message::Snapshot(_) => true,
            _ => false,
        }
    }

    pub fn get_snapshot(self) -> js_sys::Uint8Array  {
        match self.0 {
            Message::Snapshot(snapshot) => js_sys::Uint8Array::from(snapshot.as_ref()),
            _ => unreachable!("Call `is_snapshot` first."),
        }
    }
}

#[wasm_bindgen]
pub struct Network;

#[wasm_bindgen]
impl Network {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Network {
        Network
    }

    pub fn deserialize(&self, data: &str) -> MessageWrapper {
        MessageWrapper(Message::try_from(data).unwrap())
    }

    pub fn create_bios_message(&self, bios: js_sys::Uint8Array) -> String {
        let message = Message::Bios(bios.to_vec());
        (&message).try_into().unwrap()
    }

    pub fn create_rom_message(&self, rom: js_sys::Uint8Array) -> String {
        let message = Message::Rom(rom.to_vec());
        (&message).try_into().unwrap()
    }

    pub fn create_play_message(&self, snapshot: &[u8]) -> String {
        let message = Message::Play(Vec::from(snapshot));
        (&message).try_into().unwrap()
    }

    pub fn create_delta_snapshot_message(&self, old_array: js_sys::Uint8Array, new_array: js_sys::Uint8Array) -> String {
        let message = Message::DeltaSnapshot(DeltaSnapshot::new(old_array.to_vec().as_ref(), new_array.to_vec().as_ref()));
        (&message).try_into().unwrap()
    }

    pub fn create_snapshot_message(&self, snapshot: &[u8]) -> String {
        let message = Message::Snapshot(Vec::from(snapshot));
        (&message).try_into().unwrap()
    }
}
