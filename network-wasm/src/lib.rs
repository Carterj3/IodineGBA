use wasm_bindgen::prelude::*;

use ::network::{ArrayDelta, Message};

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

    pub fn get_bios(self) -> Vec<u8> {
        match self.0 {
            Message::Bios(bios) => bios,
            _ => unreachable!("Call `is_bios` first."),
        }
    }

    pub fn is_rom(&self) -> bool {
        match self.0 {
            Message::Rom(_) => true,
            _ => false,
        }
    }

    pub fn get_rom(self) -> Vec<u8> {
        match self.0 {
            Message::Rom(rom) => rom,
            _ => unreachable!("Call `is_rom` first."),
        }
    }

    pub fn is_delta_snapshot(&self) -> bool {
        match self.0 {
            Message::DeltaSnapshot(_) => true,
            _ => false,
        }
    }

    pub fn get_delta_snapshot(self, old_array: &[u8]) -> Vec<u8> {
        match self.0 {
            Message::DeltaSnapshot(delta_array) => {
                let mut new_array = Vec::from(old_array);
                for delta in delta_array.iter() {
                    new_array[delta.index() as usize] = delta.value();
                }

                new_array
            }
            _ => unreachable!("Call `is_delta_snapshot` first."),
        }
    }

    pub fn is_snapshot(&self) -> bool {
        match self.0 {
            Message::Snapshot(_) => true,
            _ => false,
        }
    }

    pub fn get_snapshot(self) -> Vec<u8> {
        match self.0 {
            Message::Snapshot(snapshot) => snapshot,
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

    pub fn create_bios_message(&self, bios: &[u8]) -> String {
        let message = Message::Bios(Vec::from(bios));
        (&message).try_into().unwrap()
    }

    pub fn create_rom_message(&self, rom: &[u8]) -> String {
        let message = Message::Rom(Vec::from(rom));
        (&message).try_into().unwrap()
    }

    pub fn create_delta_snapshot_message(&self, old_array: &[u8], new_array: &[u8]) -> String {
        let array_deltas = old_array
            .iter()
            .zip(new_array.iter())
            .enumerate()
            .filter(|(_, (old_value, new_value))| old_value != new_value)
            .map(|(index, (_, new_value))| ArrayDelta::new(index as u32, new_value.clone()))
            .collect();

        let message = Message::DeltaSnapshot(array_deltas);
        (&message).try_into().unwrap()
    }

    pub fn create_snapshot_message(&self, snapshot: &[u8]) -> String {
        let message = Message::Snapshot(Vec::from(snapshot));
        (&message).try_into().unwrap()
    }
}
