use ::b64::{FromBase64, ToBase64};

use serde::{Deserialize, Serialize};

use std::sync::Once;

use wasm_bindgen::prelude::*;

mod encoder;
use encoder::JsValueEncoder;

#[wasm_bindgen]
extern "C" {
    pub type Object;

    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get(this: &Object, prop: &str) -> JsValue;

    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get_object(this: &Object, prop: &str) -> Object;

    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get_array(this: &Object, prop: &str) -> Array;

    #[wasm_bindgen(method, structural, indexing_setter)]
    fn set(this: &Object, prop: &str, val: JsValue);

    #[wasm_bindgen(method, structural, indexing_setter)]
    fn set_object(this: &Object, prop: &str, val: Object);

    #[wasm_bindgen(method, structural, indexing_setter)]
    fn set_array(this: &Object, prop: &str, val: Array);

    pub type Array;

    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get(this: &Array, prop: &str) -> JsValue;

    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get_object(this: &Array, index: usize) -> Object;

    #[wasm_bindgen(method, structural, indexing_setter)]
    fn set_object(this: &Array, index: usize, val: Object);

    pub type Factory;

    #[wasm_bindgen(method)]
    fn object(this: &Factory) -> Object;

    #[wasm_bindgen(method)]
    fn array(this: &Factory) -> Array;
}

#[wasm_bindgen]
pub fn state_init() {
    static SET_HOOK: Once = Once::new();

    SET_HOOK.call_once(|| {
        use wasm_bindgen_console_logger::DEFAULT_LOGGER;

        console_error_panic_hook::set_once();

        log::set_logger(&DEFAULT_LOGGER).expect("Failed to set logger.");
        log::set_max_level(log::LevelFilter::Info);
    });
}

#[wasm_bindgen]
pub struct Snapshotter {
    factory: Factory,
}

#[wasm_bindgen]
impl Snapshotter {
    #[wasm_bindgen(constructor)]
    pub fn new(factory: Factory) -> Snapshotter {
        Snapshotter { factory }
    }

    pub fn serialize_to_uint8array(&self, object: Object) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(bincode::serialize(&SaveState::decode(object)).expect("Failed to serialize").as_ref())
    }

    pub fn serialize_to_b64(&self, object: Object) -> String {
        bincode::serialize(&SaveState::decode(object))
            .expect("Failed to serialize_to_b64")
            .to_base64(b64::STANDARD)
    }

    pub fn deserialize_from_uint8array(&self, data: js_sys::Uint8Array) -> Object {
        let save_state: SaveState = bincode::deserialize(data.to_vec().as_ref()).expect("Failed to deserialize");

        save_state.encode(&self.factory)
    }

    pub fn deserialize_from_b64(&self, data: &str) -> Object {
        let save_state: SaveState =
            bincode::deserialize(&data.from_base64().expect("Failed to de-base64"))
                .expect("Failed to deserialize_from_b64");

        save_state.encode(&self.factory)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayDelta {
    index: u32,
    value: u8,
}

impl ArrayDelta {
    pub fn new(index: u32, value: u8) -> ArrayDelta {
        ArrayDelta { index, value }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

fn vu8_to_vu16(vu8: &[u8]) -> Vec<u16> {
    vu8.chunks_exact(2)
        .into_iter()
        .map(|b| u16::from_ne_bytes([b[0], b[1]]))
        .collect()
}

fn vu8_to_vi32(vu8: &[u8]) -> Vec<i32> {
    vu8.chunks_exact(4)
        .into_iter()
        .map(|b| i32::from_ne_bytes([b[0], b[1], b[2], b[3]]))
        .collect()
}

#[derive(Serialize, Deserialize)]
pub struct OamTableEntry {
    ycoord: i32,
    matrix2_d: i32,
    double_size_or_disabled: i32,
    mode: i32,
    mosaic: i32,
    monolithic_palette: i32,
    shape: i32,
    xcoord: i32,
    matrix_parameters: i32,
    horizontal_flip: i32,
    vertical_flip: i32,
    size: i32,
    tile_number: i32,
    priority: i32,
    palette_number: i32,
}

impl OamTableEntry {
    fn decode_array(array: Array) -> Vec<OamTableEntry> {
        let length: i32 = JsValueEncoder::decode(array.get("length")).expect("length");
        let length = length as usize;
        let mut table = Vec::with_capacity(length);

        for i in 0..length {
            table.push(OamTableEntry::decode(array.get_object(i)));
        }

        table
    }

    fn decode(object: Object) -> OamTableEntry {
        OamTableEntry {
            ycoord: JsValueEncoder::decode(object.get("ycoord")).expect("ycoord"),
            matrix2_d: JsValueEncoder::decode(object.get("matrix2D")).expect("matrix2D"),
            double_size_or_disabled: JsValueEncoder::decode(object.get("doubleSizeOrDisabled"))
                .expect("doubleSizeOrDisabled"),
            mode: JsValueEncoder::decode(object.get("mode")).expect("mode"),
            mosaic: JsValueEncoder::decode(object.get("mosaic")).expect("mosaic"),
            monolithic_palette: JsValueEncoder::decode(object.get("monolithicPalette"))
                .expect("monolithicPalette"),
            shape: JsValueEncoder::decode(object.get("shape")).expect("shape"),
            xcoord: JsValueEncoder::decode(object.get("xcoord")).expect("xcoord"),
            matrix_parameters: JsValueEncoder::decode(object.get("matrixParameters"))
                .expect("matrixParameters"),
            horizontal_flip: JsValueEncoder::decode(object.get("horizontalFlip"))
                .expect("horizontalFlip"),
            vertical_flip: JsValueEncoder::decode(object.get("verticalFlip"))
                .expect("verticalFlip"),
            size: JsValueEncoder::decode(object.get("size")).expect("size"),
            tile_number: JsValueEncoder::decode(object.get("tileNumber")).expect("tileNumber"),
            priority: JsValueEncoder::decode(object.get("priority")).expect("priority"),
            palette_number: JsValueEncoder::decode(object.get("paletteNumber"))
                .expect("paletteNumber"),
        }
    }

    fn encode_array(entries: Vec<OamTableEntry>, factory: &Factory) -> Array {
        let array = factory.array();

        for (index, entry) in entries.into_iter().enumerate() {
            array.set_object(index, OamTableEntry::encode(entry, factory));
        }

        array
    }

    fn encode(self, factory: &Factory) -> Object {
        let object = factory.object();

        object.set(
            "ycoord",
            JsValueEncoder::encode(self.ycoord).expect("ycoord"),
        );
        object.set(
            "matrix2D",
            JsValueEncoder::encode(self.matrix2_d).expect("matrix2_d"),
        );
        object.set(
            "doubleSizeOrDisabled",
            JsValueEncoder::encode(self.double_size_or_disabled).expect("double_size_or_disabled"),
        );
        object.set("mode", JsValueEncoder::encode(self.mode).expect("mode"));
        object.set(
            "mosaic",
            JsValueEncoder::encode(self.mosaic).expect("mosaic"),
        );
        object.set(
            "monolithicPalette",
            JsValueEncoder::encode(self.monolithic_palette).expect("monolithicPalette"),
        );
        object.set("shape", JsValueEncoder::encode(self.shape).expect("shape"));
        object.set(
            "xcoord",
            JsValueEncoder::encode(self.xcoord).expect("xcoord"),
        );
        object.set(
            "matrixParameters",
            JsValueEncoder::encode(self.matrix_parameters).expect("matrixParameters"),
        );
        object.set(
            "horizontalFlip",
            JsValueEncoder::encode(self.horizontal_flip).expect("horizontalFlip"),
        );
        object.set(
            "verticalFlip",
            JsValueEncoder::encode(self.vertical_flip).expect("vertical_flip"),
        );
        object.set("size", JsValueEncoder::encode(self.size).expect("size"));
        object.set(
            "tileNumber",
            JsValueEncoder::encode(self.tile_number).expect("tile_number"),
        );
        object.set(
            "priority",
            JsValueEncoder::encode(self.priority).expect("priority"),
        );
        object.set(
            "paletteNumber",
            JsValueEncoder::encode(self.palette_number).expect("palette_number"),
        );

        object
    }
}

#[derive(Serialize, Deserialize)]
pub struct SaveState {
    clock_cycles_since_start: i32,
    iocore_accumulated_clocks: i32,
    iocore_arm_decode: i32,
    iocore_arm_execute: i32,
    iocore_arm_fetch: i32,
    iocore_arm_registers: Vec<i32>,
    iocore_arm_registers_usr: Vec<i32>,
    iocore_cartridge_eepromstart: i32,
    iocore_cartridge_flash_is128: bool,
    iocore_cartridge_flash_is_atmel: bool,
    iocore_cartridge_name: String,
    iocore_cpu_mode_flags: i32,
    iocore_cpu_mul64_result_high: i32,
    iocore_cpu_mul64_result_low: i32,
    iocore_cpu_registers_abt: Vec<i32>,
    iocore_cpu_registers_fiq: Vec<i32>,
    iocore_cpu_registers_irq: Vec<i32>,
    iocore_cpu_registers_svc: Vec<i32>,
    iocore_cpu_registers_und: Vec<i32>,
    iocore_cpu_spsr: Vec<u16>,
    iocore_cpu_triggered_irq: i32,
    iocore_cycles_overiterated_previously: i32,
    iocore_cycles_to_iterate: i32,
    iocore_dma_current_match: i32,
    iocore_dma_fetch: i32,
    iocore_dma_channel0_destination: i32,
    iocore_dma_channel0_destination_control: i32,
    iocore_dma_channel0_destination_shadow: i32,
    iocore_dma_channel0_dma_type: i32,
    iocore_dma_channel0_enabled: i32,
    iocore_dma_channel0_irq_flagging: i32,
    iocore_dma_channel0_is32_bit: i32,
    iocore_dma_channel0_pending: i32,
    iocore_dma_channel0_repeat: i32,
    iocore_dma_channel0_source: i32,
    iocore_dma_channel0_source_control: i32,
    iocore_dma_channel0_source_shadow: i32,
    iocore_dma_channel0_word_count: i32,
    iocore_dma_channel0_word_count_shadow: i32,
    iocore_dma_channel1_destination: i32,
    iocore_dma_channel1_destination_shadow: i32,
    iocore_dma_channel1_dma_type: i32,
    iocore_dma_channel1_enabled: i32,
    iocore_dma_channel1_is32_bit: i32,
    iocore_dma_channel1_repeat: i32,
    iocore_dma_channel1_source: i32,
    iocore_dma_channel1_source_shadow: i32,
    iocore_dma_channel1_word_count: i32,
    iocore_dma_channel1_word_count_shadow: i32,
    iocore_dma_channel2_destination: i32,
    iocore_dma_channel2_destination_shadow: i32,
    iocore_dma_channel2_enabled: i32,
    iocore_dma_channel2_source: i32,
    iocore_dma_channel2_source_shadow: i32,
    iocore_dma_channel3_destination: i32,
    iocore_dma_channel3_destination_shadow: i32,
    iocore_dma_channel3_display_sync_enable_delay: i32,
    iocore_dma_channel3_game_pak_dma: i32,
    iocore_dma_channel3_source: i32,
    iocore_dma_channel3_source_control: i32,
    iocore_dma_channel3_source_shadow: i32,
    iocore_dma_channel3_word_count: i32,
    iocore_gfx_renderer_iodata16: Vec<u16>,
    iocore_gfx_renderer_iodata32: Vec<i32>,
    iocore_gfx_renderer_iodata8: Vec<u8>,
    iocore_gfx_renderer_renderer_backdrop: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_bgcharacter_base_block: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_bglayer: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_bgscreen_base_block: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_bgxcoord: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_bgycoord: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_do256: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_do_mosaic: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_offset: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_priority_flag: i32,
    iocore_gfx_renderer_renderer_bg0_renderer_scratch_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_bg0_renderer_tile_fetched: Vec<i32>,
    iocore_gfx_renderer_renderer_bg0_renderer_tile_mode: i32,
    iocore_gfx_renderer_renderer_bg1_renderer_bglayer: i32,
    iocore_gfx_renderer_renderer_bg1_renderer_bgscreen_base_block: i32,
    iocore_gfx_renderer_renderer_bg1_renderer_bgxcoord: i32,
    iocore_gfx_renderer_renderer_bg1_renderer_bgycoord: i32,
    iocore_gfx_renderer_renderer_bg1_renderer_offset: i32,
    iocore_gfx_renderer_renderer_bg1_renderer_priority_flag: i32,
    iocore_gfx_renderer_renderer_bg1_renderer_scratch_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_bg1_renderer_tile_fetched: Vec<i32>,
    iocore_gfx_renderer_renderer_bg2_frame_buffer_renderer_frame_select: i32,
    iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgcharacter_base_block: i32,
    iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgdisplay_overflow: i32,
    iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgscreen_base_block: i32,
    iocore_gfx_renderer_renderer_bg2_matrix_renderer_map_size: i32,
    iocore_gfx_renderer_renderer_bg2_matrix_renderer_map_size_comparer: i32,
    iocore_gfx_renderer_renderer_bg2_matrix_renderer_palette: Vec<i32>,
    iocore_gfx_renderer_renderer_bg2_text_renderer_bgcharacter_base_block: i32,
    iocore_gfx_renderer_renderer_bg2_text_renderer_bglayer: i32,
    iocore_gfx_renderer_renderer_bg2_text_renderer_bgscreen_base_block: i32,
    iocore_gfx_renderer_renderer_bg2_text_renderer_bgycoord: i32,
    iocore_gfx_renderer_renderer_bg2_text_renderer_offset: i32,
    iocore_gfx_renderer_renderer_bg2_text_renderer_priority_flag: i32,
    iocore_gfx_renderer_renderer_bg2_text_renderer_scratch_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_bg2_text_renderer_tile_fetched: Vec<i32>,
    iocore_gfx_renderer_renderer_bg2_text_renderer_tile_mode: i32,
    iocore_gfx_renderer_renderer_bg3_matrix_renderer_bgscreen_base_block: i32,
    iocore_gfx_renderer_renderer_bg3_matrix_renderer_map_size: i32,
    iocore_gfx_renderer_renderer_bg3_matrix_renderer_map_size_comparer: i32,
    iocore_gfx_renderer_renderer_bg3_text_renderer_bglayer: i32,
    iocore_gfx_renderer_renderer_bg3_text_renderer_bgscreen_base_block: i32,
    iocore_gfx_renderer_renderer_bg3_text_renderer_offset: i32,
    iocore_gfx_renderer_renderer_bg3_text_renderer_priority_flag: i32,
    iocore_gfx_renderer_renderer_bg3_text_renderer_scratch_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_bg3_text_renderer_tile_fetched: Vec<i32>,
    iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdmx: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdmy: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdx: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdy: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer0_bgreference_x: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer0_bgreference_y: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer0_pb: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer0_pd: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer0_scratch_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_bg_affine_renderer1_bgdmy: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer1_bgdx: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer1_bgreference_x: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer1_bgreference_y: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer1_pb: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer1_pd: i32,
    iocore_gfx_renderer_renderer_bg_affine_renderer1_scratch_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_color_effects_renderer_alpha_blend_amount_target1: i32,
    iocore_gfx_renderer_renderer_color_effects_renderer_alpha_blend_amount_target2: i32,
    iocore_gfx_renderer_renderer_color_effects_renderer_brightness_effect_amount: i32,
    iocore_gfx_renderer_renderer_color_effects_renderer_color_effects_type: i32,
    iocore_gfx_renderer_renderer_color_effects_renderer_effects_target1: i32,
    iocore_gfx_renderer_renderer_color_effects_renderer_effects_target2: i32,
    iocore_gfx_renderer_renderer_compositor_do_effects: i32,
    iocore_gfx_renderer_renderer_display: i32,
    iocore_gfx_renderer_renderer_display_control: i32,
    iocore_gfx_renderer_renderer_frame_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_green_swap: i32,
    iocore_gfx_renderer_renderer_last_unrendered_line: i32,
    iocore_gfx_renderer_renderer_line_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_mosaic_renderer_bgmosaic_hsize: i32,
    iocore_gfx_renderer_renderer_mosaic_renderer_bgmosaic_vsize: i32,
    iocore_gfx_renderer_renderer_mosaic_renderer_objmosaic_hsize: i32,
    iocore_gfx_renderer_renderer_mosaic_renderer_objmosaic_vsize: i32,
    iocore_gfx_renderer_renderer_obj_renderer_cycles_to_render: i32,
    iocore_gfx_renderer_renderer_obj_renderer_oamram: Vec<u8>,
    // iocore_gfx_renderer_renderer_obj_renderer_oamram16: Vec<u16>,
    // iocore_gfx_renderer_renderer_obj_renderer_oamram32: Vec<i32>,
    iocore_gfx_renderer_renderer_obj_renderer_oamtable: Vec<OamTableEntry>,
    iocore_gfx_renderer_renderer_obj_renderer_objmatrix_parameters: Vec<i32>,
    iocore_gfx_renderer_renderer_obj_renderer_scratch_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_obj_renderer_scratch_objbuffer: Vec<i32>,
    iocore_gfx_renderer_renderer_obj_renderer_scratch_window_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_obj_window_renderer_compositor_objwindow_buffer: Vec<i32>,
    iocore_gfx_renderer_renderer_obj_window_renderer_winobjoutside: i32,
    iocore_gfx_renderer_renderer_palette16: Vec<i32>,
    iocore_gfx_renderer_renderer_palette256: Vec<i32>,
    iocore_gfx_renderer_renderer_palette_obj16: Vec<i32>,
    iocore_gfx_renderer_renderer_palette_obj256: Vec<i32>,
    iocore_gfx_renderer_renderer_palette_ram: Vec<u8>,
    // iocore_gfx_renderer_renderer_palette_ram16: Vec<u16>,
    // iocore_gfx_renderer_renderer_palette_ram32: Vec<i32>,
    iocore_gfx_renderer_renderer_queued_scan_lines: i32,
    iocore_gfx_renderer_renderer_swizzled_frame: Vec<u8>,
    iocore_gfx_renderer_renderer_total_lines_passed: i32,
    iocore_gfx_renderer_renderer_vram: Vec<u8>,
    // iocore_gfx_renderer_renderer_vram16: Vec<u16>,
    // iocore_gfx_renderer_renderer_vram32: Vec<i32>,
    iocore_gfx_renderer_renderer_window0_renderer_compositor_do_effects: i32,
    iocore_gfx_renderer_renderer_window0_renderer_window_display_control: i32,
    iocore_gfx_renderer_renderer_window0_renderer_winxcoord_left: i32,
    iocore_gfx_renderer_renderer_window0_renderer_winxcoord_right: i32,
    iocore_gfx_renderer_renderer_window0_renderer_winycoord_bottom: i32,
    iocore_gfx_renderer_renderer_window0_renderer_winycoord_top: i32,
    iocore_gfx_renderer_renderer_winoutside: i32,
    iocore_gfx_state_current_scan_line: i32,
    iocore_gfx_state_irqflags: i32,
    iocore_gfx_state_lcdticks: i32,
    iocore_gfx_state_rendered_scan_line: bool,
    iocore_gfx_state_status_flags: i32,
    iocore_gfx_state_vcounter: i32,
    iocore_irq_interrupts_enabled: i32,
    iocore_irq_interrupts_requested: i32,
    iocore_joypad_key_input: i32,
    iocore_joypad_key_interrupt: i32,
    iocore_memory_external_ram: Vec<u8>,
    // iocore_memory_external_ram16: Vec<u16>,
    // iocore_memory_external_ram32: Vec<i32>,
    iocore_memory_internal_ram: Vec<u8>,
    // iocore_memory_internal_ram16: Vec<u16>,
    // iocore_memory_internal_ram32: Vec<i32>,
    iocore_memory_irq_ime: i32,
    iocore_memory_last_biosread: i32,
    iocore_memory_wramcontrol_flags: i32,
    iocore_next_event_clocks: i32,
    iocore_saves_eepromchip_address: i32,
    iocore_saves_eepromchip_bits_processed: i32,
    iocore_saves_eepromchip_buffer: Vec<u8>,
    iocore_saves_eepromchip_largest_size_possible: i32,
    iocore_saves_eepromchip_mode: i32,
    // iocore_saves_flashchip_bankoffset: i32,
    // iocore_saves_flashchip_flash_command: i32,
    // iocore_saves_flashchip_flash_command_unlock_stage: i32,
    // iocore_saves_flashchip_largest_size_possible: i32,
    // iocore_saves_flashchip_not_atmel: bool,
    // iocore_saves_flashchip_saves: Vec<u8>,
    // iocore_saves_flashchip_write_bytes_left: i32,
    iocore_saves_gpiochip_data: i32,
    iocore_saves_gpiochip_direction: i32,
    iocore_saves_gpiochip_read_write: i32,
    iocore_saves_gpiochip_type: i32,
    iocore_saves_save_type: i32,
    iocore_saves_undetermined_possible: i32,
    iocore_serial_joybus_cntl_flags: i32,
    iocore_serial_joybus_irq: i32,
    iocore_serial_joybus_recv0: i32,
    iocore_serial_joybus_recv1: i32,
    iocore_serial_joybus_recv2: i32,
    iocore_serial_joybus_recv3: i32,
    iocore_serial_joybus_send0: i32,
    iocore_serial_joybus_send1: i32,
    iocore_serial_joybus_send2: i32,
    iocore_serial_joybus_send3: i32,
    iocore_serial_joybus_stat: i32,
    iocore_serial_rcntdata_bit_flow: i32,
    iocore_serial_rcntdata_bits: i32,
    iocore_serial_rcntirq: bool,
    iocore_serial_rcntmode: i32,
    iocore_serial_serial_bits_shifted: i32,
    iocore_serial_shift_clocks: i32,
    iocore_serial_siobaud_rate: i32,
    iocore_serial_siocnt_irq: i32,
    iocore_serial_siocnt_mode: i32,
    iocore_serial_siocnt_uart_cts: bool,
    iocore_serial_siocnt_uart_fifo: i32,
    iocore_serial_siocnt_uart_fifo_enable: bool,
    iocore_serial_siocnt_uart_misc: i32,
    iocore_serial_siocnt_uart_parity_enable: bool,
    iocore_serial_siocnt_uart_recv_enable: bool,
    iocore_serial_siocnt_uart_send_enable: bool,
    iocore_serial_siocnt0_data: i32,
    iocore_serial_siocommerror: bool,
    iocore_serial_siodata_a: i32,
    iocore_serial_siodata_b: i32,
    iocore_serial_siodata_c: i32,
    iocore_serial_siodata_d: i32,
    iocore_serial_siodata8: i32,
    iocore_serial_siomult_player_number: i32,
    iocore_serial_sioshift_clock_divider: i32,
    iocore_serial_sioshift_clock_external: i32,
    iocore_serial_siotransfer_started: bool,
    iocore_serial_clocks: i32,
    // iocore_sound_agbdirect_sound_a: i32,
    // iocore_sound_agbdirect_sound_afolded: i32,
    // iocore_sound_agbdirect_sound_aleft_can_play: bool,
    // iocore_sound_agbdirect_sound_aright_can_play: bool,
    // iocore_sound_agbdirect_sound_ashifter: i32,
    // iocore_sound_agbdirect_sound_atimer: i32,
    // iocore_sound_agbdirect_sound_b: i32,
    // iocore_sound_agbdirect_sound_bfolded: i32,
    // iocore_sound_agbdirect_sound_bleft_can_play: bool,
    // iocore_sound_agbdirect_sound_bright_can_play: bool,
    // iocore_sound_agbdirect_sound_bshifter: i32,
    // iocore_sound_agbdirect_sound_btimer: i32,
    // iocore_sound_audio_clocks_until_next_event: i32,
    // iocore_sound_audio_clocks_until_next_event_counter: i32,
    // iocore_sound_audio_index: i32,
    // iocore_sound_audio_resampler_first_pass_factor: i32,
    // iocore_sound_audio_ticks: i32,
    // iocore_sound_cgbmixer_output_cache_left: i32,
    // iocore_sound_cgbmixer_output_cache_left_folded: i32,
    // iocore_sound_cgbmixer_output_cache_right: i32,
    // iocore_sound_cgbmixer_output_cache_right_folded: i32,
    // iocore_sound_cgboutput_ratio: i32,
    // iocore_sound_channel1_cached_duty: f64,
    // iocore_sound_channel1_can_play: bool,
    // iocore_sound_channel1_consecutive: bool,
    // iocore_sound_channel1_current_sample_left: i32,
    // iocore_sound_channel1_current_sample_right: i32,
    // iocore_sound_channel1_decrease_sweep: bool,
    // iocore_sound_channel1_duty_tracker: i32,
    // iocore_sound_channel1_enabled: i32,
    // iocore_sound_channel1_envelope_sweeps: i32,
    // iocore_sound_channel1_envelope_sweeps_last: i32,
    // iocore_sound_channel1_envelope_volume: i32,
    // iocore_sound_channel1_frequency: i32,
    // iocore_sound_channel1_frequency_counter: i32,
    // iocore_sound_channel1_frequency_sweep_divider: i32,
    // iocore_sound_channel1_frequency_tracker: i32,
    // iocore_sound_channel1_last_time_sweep: i32,
    // iocore_sound_channel1_left_enable: i32,
    // iocore_sound_channel1_nr10: i32,
    // iocore_sound_channel1_nr11: i32,
    // iocore_sound_channel1_nr12: i32,
    // iocore_sound_channel1_nr14: i32,
    // iocore_sound_channel1_right_enable: i32,
    // iocore_sound_channel1_shadow_frequency: i32,
    // iocore_sound_channel1_sweep_fault: bool,
    // iocore_sound_channel1_swept: bool,
    // iocore_sound_channel1_time_sweep: i32,
    // iocore_sound_channel1_total_length: i32,
    // iocore_sound_channel2_nr21: i32,
    // iocore_sound_channel2_nr22: i32,
    // iocore_sound_channel2_nr23: i32,
    // iocore_sound_channel2_nr24: i32,
    // iocore_sound_channel2_shadow_frequency: i32,
    // iocore_sound_channel3_cached_sample: i32,
    // iocore_sound_channel3_can_play: bool,
    // iocore_sound_channel3_counter: i32,
    // iocore_sound_channel3_frequency_period: i32,
    // iocore_sound_channel3_last_sample_lookup: i32,
    // iocore_sound_channel3_nr30: i32,
    // iocore_sound_channel3_nr31: i32,
    // iocore_sound_channel3_nr32: i32,
    // iocore_sound_channel3_nr33: i32,
    // iocore_sound_channel3_nr34: i32,
    // iocore_sound_channel3_pattern_type: i32,
    // iocore_sound_channel3_pcm: Vec<i8>,
    // iocore_sound_channel3_pcm16: Vec<u16>,
    // iocore_sound_channel3_pcm32: Vec<i32>,
    // iocore_sound_channel3_waveram16: Vec<u16>,
    // iocore_sound_channel3_waveram32: Vec<i32>,
    // iocore_sound_channel3_waveram8: Vec<u8>,
    // iocore_sound_channel3_waverambank_accessed: i32,
    // iocore_sound_channel3_wave_rambank_size: i32,
    // iocore_sound_channel3_waverambank_specified: i32,
    // iocore_sound_channel4_bit_range: i32,
    // iocore_sound_channel4_counter: i32,
    // iocore_sound_channel4_current_volume: i32,
    // iocore_sound_channel4_frequency_period: i32,
    // iocore_sound_channel4_last_sample_lookup: i32,
    // iocore_sound_channel4_lsfr15_table: Vec<i8>,
    // iocore_sound_channel4_lsfr7_table: Vec<i8>,
    // iocore_sound_channel4_noise_sample_table: Vec<i8>,
    // iocore_sound_channel4_nr42: i32,
    // iocore_sound_channel4_nr43: i32,
    // iocore_sound_channel4_nr44: i32,
    // iocore_sound_channel4_volume_shifter: i32,
    // iocore_sound_downsample_input_left: i32,
    // iocore_sound_downsample_input_right: i32,
    // iocore_sound_fifoabuffer_buffer: Vec<i8>,
    // iocore_sound_fifoabuffer_count: i32,
    // iocore_sound_fifoabuffer_position: i32,
    // iocore_sound_fifobbuffer_buffer: Vec<i8>,
    // iocore_sound_mixer_output_cache_left: i32,
    // iocore_sound_mixer_output_cache_right: i32,
    // iocore_sound_mixer_sound_bias: i32,
    // iocore_sound_nr50: i32,
    // iocore_sound_nr51: i32,
    // iocore_sound_nr52: i32,
    // iocore_sound_nr60: i32,
    // iocore_sound_nr61: i32,
    // iocore_sound_nr62: i32,
    // iocore_sound_nr63: i32,
    // iocore_sound_pwmbit_depth_mask: i32,
    // iocore_sound_pwmbit_depth_mask_shadow: i32,
    // iocore_sound_pwmwidth: i32,
    // iocore_sound_pwmwidth_old: i32,
    // iocore_sound_pwmwidth_shadow: i32,
    // iocore_sound_sequence_position: i32,
    // iocore_sound_sequencer_clocks: i32,
    // iocore_sound_sound_master_enabled: bool,
    // iocore_sound_vin_left_channel_master_volume: i32,
    // iocore_sound_vin_right_channel_master_volume: i32,
    iocore_system_status: i32,
    iocore_thumb_decode: i32,
    iocore_thumb_execute: i32,
    iocore_thumb_fetch: i32,
    iocore_timer_timer0_control: i32,
    iocore_timer_timer0_counter: i32,
    iocore_timer_timer0_enabled: bool,
    iocore_timer_timer0_irq: bool,
    iocore_timer_timer0_precounter: i32,
    iocore_timer_timer0_prescalar: i32,
    iocore_timer_timer0_prescalar_shifted: i32,
    iocore_timer_timer0_reload: i32,
    iocore_timer_timer1_control: i32,
    iocore_timer_timer1_counter: i32,
    iocore_timer_timer1_count_up: bool,
    iocore_timer_timer1_enabled: bool,
    iocore_timer_timer1_irq: bool,
    iocore_timer_timer1_precounter: i32,
    iocore_timer_timer1_prescalar: i32,
    iocore_timer_timer1_prescalar_shifted: i32,
    iocore_timer_timer1_reload: i32,
    iocore_timer_timer1_use_chained_clocks: bool,
    iocore_timer_timer1_use_main_clocks: bool,
    iocore_timer_timer2_control: i32,
    iocore_timer_timer2_counter: i32,
    iocore_timer_timer2_count_up: bool,
    iocore_timer_timer2_enabled: bool,
    iocore_timer_timer2_irq: bool,
    iocore_timer_timer2_precounter: i32,
    iocore_timer_timer2_prescalar: i32,
    iocore_timer_timer2_prescalar_shifted: i32,
    iocore_timer_timer2_reload: i32,
    iocore_timer_timer2_use_chained_clocks: bool,
    iocore_timer_timer2_use_main_clocks: bool,
    iocore_timer_timer3_control: i32,
    iocore_timer_timer3_counter: i32,
    iocore_timer_timer3_count_up: bool,
    iocore_timer_timer3_enabled: bool,
    iocore_timer_timer3_irq: bool,
    iocore_timer_timer3_precounter: i32,
    iocore_timer_timer3_prescalar: i32,
    iocore_timer_timer3_prescalar_shifted: i32,
    iocore_timer_timer3_reload: i32,
    iocore_timer_timer3_use_chained_clocks: bool,
    iocore_timer_timer3_use_main_clocks: bool,
    iocore_timer_clocks: i32,
    iocore_wait_buffer: i32,
    iocore_wait_clocks: i32,
    iocore_wait_is_oamrendering: i32,
    iocore_wait_is_rendering: i32,
    iocore_wait_non_sequential: i32,
    iocore_wait_postboot: i32,
    iocore_wait_sramwait_state: i32,
    iocore_wait_waitcnt0: i32,
    iocore_wait_waitcnt1: i32,
    iocore_wait_wait_state_clocks16: Vec<u8>,
    iocore_wait_wait_state_clocks32: Vec<u8>,
    iocore_wait_wramconfiguration: i32,
    iocore_wait_wramwait_state: i32,
    last_timestamp: i32,
    metric_start: i32,
}

impl SaveState {
    fn decode(object: Object) -> SaveState {
        SaveState {
            clock_cycles_since_start: JsValueEncoder::decode(object.get("clockCyclesSinceStart"))
                .expect("clockCyclesSinceStart"),
            iocore_accumulated_clocks: JsValueEncoder::decode(
                object.get("IOCore.accumulatedClocks"),
            )
            .expect("IOCore.accumulatedClocks"),
            iocore_arm_decode: JsValueEncoder::decode(object.get("IOCore.ARM.decode"))
                .expect("IOCore.ARM.decode"),
            iocore_arm_execute: JsValueEncoder::decode(object.get("IOCore.ARM.execute"))
                .expect("IOCore.ARM.execute"),
            iocore_arm_fetch: JsValueEncoder::decode(object.get("IOCore.ARM.fetch"))
                .expect("IOCore.ARM.fetch"),
            iocore_arm_registers: JsValueEncoder::decode(object.get("IOCore.ARM.registers"))
                .expect("IOCore.ARM.registers"),
            iocore_arm_registers_usr: JsValueEncoder::decode(object.get("IOCore.ARM.registersUSR"))
                .expect("IOCore.ARM.registersUSR"),
            iocore_cartridge_eepromstart: JsValueEncoder::decode(
                object.get("IOCore.cartridge.EEPROMStart"),
            )
            .expect("IOCore.cartridge.EEPROMStart"),
            iocore_cartridge_flash_is128: JsValueEncoder::decode(
                object.get("IOCore.cartridge.flash_is128"),
            )
            .expect("IOCore.cartridge.flash_is128"),
            iocore_cartridge_flash_is_atmel: JsValueEncoder::decode(
                object.get("IOCore.cartridge.flash_isAtmel"),
            )
            .expect("IOCore.cartridge.flash_isAtmel"),
            iocore_cartridge_name: JsValueEncoder::decode(object.get("IOCore.cartridge.name"))
                .expect("IOCore.cartridge.name"),
            iocore_cpu_mode_flags: JsValueEncoder::decode(object.get("IOCore.cpu.modeFlags"))
                .expect("IOCore.cpu.modeFlags"),
            iocore_cpu_mul64_result_high: JsValueEncoder::decode(
                object.get("IOCore.cpu.mul64ResultHigh"),
            )
            .expect("IOCore.cpu.mul64ResultHigh"),
            iocore_cpu_mul64_result_low: JsValueEncoder::decode(
                object.get("IOCore.cpu.mul64ResultLow"),
            )
            .expect("IOCore.cpu.mul64ResultLow"),
            iocore_cpu_registers_abt: JsValueEncoder::decode(object.get("IOCore.cpu.registersABT"))
                .expect("IOCore.cpu.registersABT"),
            iocore_cpu_registers_fiq: JsValueEncoder::decode(object.get("IOCore.cpu.registersFIQ"))
                .expect("IOCore.cpu.registersFIQ"),
            iocore_cpu_registers_irq: JsValueEncoder::decode(object.get("IOCore.cpu.registersIRQ"))
                .expect("IOCore.cpu.registersIRQ"),
            iocore_cpu_registers_svc: JsValueEncoder::decode(object.get("IOCore.cpu.registersSVC"))
                .expect("IOCore.cpu.registersSVC"),
            iocore_cpu_registers_und: JsValueEncoder::decode(object.get("IOCore.cpu.registersUND"))
                .expect("IOCore.cpu.registersUND"),
            iocore_cpu_spsr: JsValueEncoder::decode(object.get("IOCore.cpu.SPSR"))
                .expect("IOCore.cpu.SPSR"),
            iocore_cpu_triggered_irq: JsValueEncoder::decode(object.get("IOCore.cpu.triggeredIRQ"))
                .expect("IOCore.cpu.triggeredIRQ"),
            iocore_cycles_overiterated_previously: JsValueEncoder::decode(
                object.get("IOCore.cyclesOveriteratedPreviously"),
            )
            .expect("IOCore.cyclesOveriteratedPreviously"),
            iocore_cycles_to_iterate: JsValueEncoder::decode(object.get("IOCore.cyclesToIterate"))
                .expect("IOCore.cyclesToIterate"),
            iocore_dma_current_match: JsValueEncoder::decode(object.get("IOCore.dma.currentMatch"))
                .expect("IOCore.dma.currentMatch"),
            iocore_dma_fetch: JsValueEncoder::decode(object.get("IOCore.dma.fetch"))
                .expect("IOCore.dma.fetch"),
            iocore_dma_channel0_destination: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.destination"),
            )
            .expect("IOCore.dmaChannel0.destination"),
            iocore_dma_channel0_destination_control: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.destinationControl"),
            )
            .expect("IOCore.dmaChannel0.destinationControl"),
            iocore_dma_channel0_destination_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.destinationShadow"),
            )
            .expect("IOCore.dmaChannel0.destinationShadow"),
            iocore_dma_channel0_dma_type: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.dmaType"),
            )
            .expect("IOCore.dmaChannel0.dmaType"),
            iocore_dma_channel0_enabled: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.enabled"),
            )
            .expect("IOCore.dmaChannel0.enabled"),
            iocore_dma_channel0_irq_flagging: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.irqFlagging"),
            )
            .expect("IOCore.dmaChannel0.irqFlagging"),
            iocore_dma_channel0_is32_bit: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.is32Bit"),
            )
            .expect("IOCore.dmaChannel0.is32Bit"),
            iocore_dma_channel0_pending: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.pending"),
            )
            .expect("IOCore.dmaChannel0.pending"),
            iocore_dma_channel0_repeat: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.repeat"),
            )
            .expect("IOCore.dmaChannel0.repeat"),
            iocore_dma_channel0_source: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.source"),
            )
            .expect("IOCore.dmaChannel0.source"),
            iocore_dma_channel0_source_control: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.sourceControl"),
            )
            .expect("IOCore.dmaChannel0.sourceControl"),
            iocore_dma_channel0_source_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.sourceShadow"),
            )
            .expect("IOCore.dmaChannel0.sourceShadow"),
            iocore_dma_channel0_word_count: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.wordCount"),
            )
            .expect("IOCore.dmaChannel0.wordCount"),
            iocore_dma_channel0_word_count_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel0.wordCountShadow"),
            )
            .expect("IOCore.dmaChannel0.wordCountShadow"),
            iocore_dma_channel1_destination: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.destination"),
            )
            .expect("IOCore.dmaChannel1.destination"),
            iocore_dma_channel1_destination_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.destinationShadow"),
            )
            .expect("IOCore.dmaChannel1.destinationShadow"),
            iocore_dma_channel1_dma_type: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.dmaType"),
            )
            .expect("IOCore.dmaChannel1.dmaType"),
            iocore_dma_channel1_enabled: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.enabled"),
            )
            .expect("IOCore.dmaChannel1.enabled"),
            iocore_dma_channel1_is32_bit: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.is32Bit"),
            )
            .expect("IOCore.dmaChannel1.is32Bit"),
            iocore_dma_channel1_repeat: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.repeat"),
            )
            .expect("IOCore.dmaChannel1.repeat"),
            iocore_dma_channel1_source: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.source"),
            )
            .expect("IOCore.dmaChannel1.source"),
            iocore_dma_channel1_source_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.sourceShadow"),
            )
            .expect("IOCore.dmaChannel1.sourceShadow"),
            iocore_dma_channel1_word_count: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.wordCount"),
            )
            .expect("IOCore.dmaChannel1.wordCount"),
            iocore_dma_channel1_word_count_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel1.wordCountShadow"),
            )
            .expect("IOCore.dmaChannel1.wordCountShadow"),
            iocore_dma_channel2_destination: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel2.destination"),
            )
            .expect("IOCore.dmaChannel2.destination"),
            iocore_dma_channel2_destination_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel2.destinationShadow"),
            )
            .expect("IOCore.dmaChannel2.destinationShadow"),
            iocore_dma_channel2_enabled: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel2.enabled"),
            )
            .expect("IOCore.dmaChannel2.enabled"),
            iocore_dma_channel2_source: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel2.source"),
            )
            .expect("IOCore.dmaChannel2.source"),
            iocore_dma_channel2_source_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel2.sourceShadow"),
            )
            .expect("IOCore.dmaChannel2.sourceShadow"),
            iocore_dma_channel3_destination: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel3.destination"),
            )
            .expect("IOCore.dmaChannel3.destination"),
            iocore_dma_channel3_destination_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel3.destinationShadow"),
            )
            .expect("IOCore.dmaChannel3.destinationShadow"),
            iocore_dma_channel3_display_sync_enable_delay: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel3.displaySyncEnableDelay"),
            )
            .expect("IOCore.dmaChannel3.displaySyncEnableDelay"),
            iocore_dma_channel3_game_pak_dma: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel3.gamePakDMA"),
            )
            .expect("IOCore.dmaChannel3.gamePakDMA"),
            iocore_dma_channel3_source: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel3.source"),
            )
            .expect("IOCore.dmaChannel3.source"),
            iocore_dma_channel3_source_control: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel3.sourceControl"),
            )
            .expect("IOCore.dmaChannel3.sourceControl"),
            iocore_dma_channel3_source_shadow: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel3.sourceShadow"),
            )
            .expect("IOCore.dmaChannel3.sourceShadow"),
            iocore_dma_channel3_word_count: JsValueEncoder::decode(
                object.get("IOCore.dmaChannel3.wordCount"),
            )
            .expect("IOCore.dmaChannel3.wordCount"),
            iocore_gfx_renderer_iodata16: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.IOData16"),
            )
            .expect("IOCore.gfxRenderer.IOData16"),
            iocore_gfx_renderer_iodata32: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.IOData32"),
            )
            .expect("IOCore.gfxRenderer.IOData32"),
            iocore_gfx_renderer_iodata8: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.IOData8"),
            )
            .expect("IOCore.gfxRenderer.IOData8"),
            iocore_gfx_renderer_renderer_backdrop: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.backdrop"),
            )
            .expect("IOCore.gfxRenderer.renderer.backdrop"),
            iocore_gfx_renderer_renderer_bg0_renderer_bgcharacter_base_block:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg0Renderer.BGCharacterBaseBlock"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg0Renderer.BGCharacterBaseBlock"),
            iocore_gfx_renderer_renderer_bg0_renderer_bglayer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.BGLayer"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.BGLayer"),
            iocore_gfx_renderer_renderer_bg0_renderer_bgscreen_base_block: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.BGScreenBaseBlock"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.BGScreenBaseBlock"),
            iocore_gfx_renderer_renderer_bg0_renderer_bgxcoord: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.BGXCoord"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.BGXCoord"),
            iocore_gfx_renderer_renderer_bg0_renderer_bgycoord: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.BGYCoord"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.BGYCoord"),
            iocore_gfx_renderer_renderer_bg0_renderer_do256: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.do256"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.do256"),
            iocore_gfx_renderer_renderer_bg0_renderer_do_mosaic: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.doMosaic"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.doMosaic"),
            iocore_gfx_renderer_renderer_bg0_renderer_offset: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.offset"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.offset"),
            iocore_gfx_renderer_renderer_bg0_renderer_priority_flag: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.priorityFlag"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.priorityFlag"),
            iocore_gfx_renderer_renderer_bg0_renderer_scratch_buffer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.scratchBuffer"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.scratchBuffer"),
            iocore_gfx_renderer_renderer_bg0_renderer_tile_fetched: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.tileFetched"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.tileFetched"),
            iocore_gfx_renderer_renderer_bg0_renderer_tile_mode: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg0Renderer.tileMode"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg0Renderer.tileMode"),
            iocore_gfx_renderer_renderer_bg1_renderer_bglayer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg1Renderer.BGLayer"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg1Renderer.BGLayer"),
            iocore_gfx_renderer_renderer_bg1_renderer_bgscreen_base_block: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg1Renderer.BGScreenBaseBlock"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg1Renderer.BGScreenBaseBlock"),
            iocore_gfx_renderer_renderer_bg1_renderer_bgxcoord: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg1Renderer.BGXCoord"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg1Renderer.BGXCoord"),
            iocore_gfx_renderer_renderer_bg1_renderer_bgycoord: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg1Renderer.BGYCoord"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg1Renderer.BGYCoord"),
            iocore_gfx_renderer_renderer_bg1_renderer_offset: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg1Renderer.offset"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg1Renderer.offset"),
            iocore_gfx_renderer_renderer_bg1_renderer_priority_flag: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg1Renderer.priorityFlag"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg1Renderer.priorityFlag"),
            iocore_gfx_renderer_renderer_bg1_renderer_scratch_buffer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg1Renderer.scratchBuffer"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg1Renderer.scratchBuffer"),
            iocore_gfx_renderer_renderer_bg1_renderer_tile_fetched: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg1Renderer.tileFetched"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg1Renderer.tileFetched"),
            iocore_gfx_renderer_renderer_bg2_frame_buffer_renderer_frame_select:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg2FrameBufferRenderer.frameSelect"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg2FrameBufferRenderer.frameSelect"),
            iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgcharacter_base_block:
                JsValueEncoder::decode(
                    object
                        .get("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGCharacterBaseBlock"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGCharacterBaseBlock"),
            iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgdisplay_overflow:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGDisplayOverflow"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGDisplayOverflow"),
            iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgscreen_base_block:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGScreenBaseBlock"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGScreenBaseBlock"),
            iocore_gfx_renderer_renderer_bg2_matrix_renderer_map_size: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.mapSize"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.mapSize"),
            iocore_gfx_renderer_renderer_bg2_matrix_renderer_map_size_comparer:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.mapSizeComparer"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.mapSizeComparer"),
            iocore_gfx_renderer_renderer_bg2_matrix_renderer_palette: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.palette"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg2MatrixRenderer.palette"),
            iocore_gfx_renderer_renderer_bg2_text_renderer_bgcharacter_base_block:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg2TextRenderer.BGCharacterBaseBlock"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg2TextRenderer.BGCharacterBaseBlock"),
            iocore_gfx_renderer_renderer_bg2_text_renderer_bglayer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg2TextRenderer.BGLayer"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg2TextRenderer.BGLayer"),
            iocore_gfx_renderer_renderer_bg2_text_renderer_bgscreen_base_block:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg2TextRenderer.BGScreenBaseBlock"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg2TextRenderer.BGScreenBaseBlock"),
            iocore_gfx_renderer_renderer_bg2_text_renderer_bgycoord: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg2TextRenderer.BGYCoord"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg2TextRenderer.BGYCoord"),
            iocore_gfx_renderer_renderer_bg2_text_renderer_offset: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg2TextRenderer.offset"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg2TextRenderer.offset"),
            iocore_gfx_renderer_renderer_bg2_text_renderer_priority_flag: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg2TextRenderer.priorityFlag"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg2TextRenderer.priorityFlag"),
            iocore_gfx_renderer_renderer_bg2_text_renderer_scratch_buffer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg2TextRenderer.scratchBuffer"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg2TextRenderer.scratchBuffer"),
            iocore_gfx_renderer_renderer_bg2_text_renderer_tile_fetched: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg2TextRenderer.tileFetched"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg2TextRenderer.tileFetched"),
            iocore_gfx_renderer_renderer_bg2_text_renderer_tile_mode: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg2TextRenderer.tileMode"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg2TextRenderer.tileMode"),
            iocore_gfx_renderer_renderer_bg3_matrix_renderer_bgscreen_base_block:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg3MatrixRenderer.BGScreenBaseBlock"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg3MatrixRenderer.BGScreenBaseBlock"),
            iocore_gfx_renderer_renderer_bg3_matrix_renderer_map_size: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg3MatrixRenderer.mapSize"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg3MatrixRenderer.mapSize"),
            iocore_gfx_renderer_renderer_bg3_matrix_renderer_map_size_comparer:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg3MatrixRenderer.mapSizeComparer"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg3MatrixRenderer.mapSizeComparer"),
            iocore_gfx_renderer_renderer_bg3_text_renderer_bglayer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg3TextRenderer.BGLayer"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg3TextRenderer.BGLayer"),
            iocore_gfx_renderer_renderer_bg3_text_renderer_bgscreen_base_block:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bg3TextRenderer.BGScreenBaseBlock"),
                )
                .expect("IOCore.gfxRenderer.renderer.bg3TextRenderer.BGScreenBaseBlock"),
            iocore_gfx_renderer_renderer_bg3_text_renderer_offset: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg3TextRenderer.offset"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg3TextRenderer.offset"),
            iocore_gfx_renderer_renderer_bg3_text_renderer_priority_flag: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg3TextRenderer.priorityFlag"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg3TextRenderer.priorityFlag"),
            iocore_gfx_renderer_renderer_bg3_text_renderer_scratch_buffer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg3TextRenderer.scratchBuffer"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg3TextRenderer.scratchBuffer"),
            iocore_gfx_renderer_renderer_bg3_text_renderer_tile_fetched: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bg3TextRenderer.tileFetched"),
            )
            .expect("IOCore.gfxRenderer.renderer.bg3TextRenderer.tileFetched"),
            iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdmx: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdmx"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdmx"),
            iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdmy: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdmy"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdmy"),
            iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdx: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdx"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdx"),
            iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdy: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdy"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdy"),
            iocore_gfx_renderer_renderer_bg_affine_renderer0_bgreference_x: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGReferenceX"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGReferenceX"),
            iocore_gfx_renderer_renderer_bg_affine_renderer0_bgreference_y: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGReferenceY"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGReferenceY"),
            iocore_gfx_renderer_renderer_bg_affine_renderer0_pb: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer0.pb"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer0.pb"),
            iocore_gfx_renderer_renderer_bg_affine_renderer0_pd: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer0.pd"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer0.pd"),
            iocore_gfx_renderer_renderer_bg_affine_renderer0_scratch_buffer:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer0.scratchBuffer"),
                )
                .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer0.scratchBuffer"),
            iocore_gfx_renderer_renderer_bg_affine_renderer1_bgdmy: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGdmy"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGdmy"),
            iocore_gfx_renderer_renderer_bg_affine_renderer1_bgdx: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGdx"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGdx"),
            iocore_gfx_renderer_renderer_bg_affine_renderer1_bgreference_x: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGReferenceX"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGReferenceX"),
            iocore_gfx_renderer_renderer_bg_affine_renderer1_bgreference_y: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGReferenceY"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGReferenceY"),
            iocore_gfx_renderer_renderer_bg_affine_renderer1_pb: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer1.pb"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer1.pb"),
            iocore_gfx_renderer_renderer_bg_affine_renderer1_pd: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer1.pd"),
            )
            .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer1.pd"),
            iocore_gfx_renderer_renderer_bg_affine_renderer1_scratch_buffer:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.bgAffineRenderer1.scratchBuffer"),
                )
                .expect("IOCore.gfxRenderer.renderer.bgAffineRenderer1.scratchBuffer"),
            iocore_gfx_renderer_renderer_buffer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.buffer"),
            )
            .expect("IOCore.gfxRenderer.renderer.buffer"),
            iocore_gfx_renderer_renderer_color_effects_renderer_alpha_blend_amount_target1:
                JsValueEncoder::decode(object.get(
                    "IOCore.gfxRenderer.renderer.colorEffectsRenderer.alphaBlendAmountTarget1",
                ))
                .expect("IOCore.gfxRenderer.renderer.colorEffectsRenderer.alphaBlendAmountTarget1"),
            iocore_gfx_renderer_renderer_color_effects_renderer_alpha_blend_amount_target2:
                JsValueEncoder::decode(object.get(
                    "IOCore.gfxRenderer.renderer.colorEffectsRenderer.alphaBlendAmountTarget2",
                ))
                .expect("IOCore.gfxRenderer.renderer.colorEffectsRenderer.alphaBlendAmountTarget2"),
            iocore_gfx_renderer_renderer_color_effects_renderer_brightness_effect_amount:
                JsValueEncoder::decode(
                    object.get(
                        "IOCore.gfxRenderer.renderer.colorEffectsRenderer.brightnessEffectAmount",
                    ),
                )
                .expect("IOCore.gfxRenderer.renderer.colorEffectsRenderer.brightnessEffectAmount"),
            iocore_gfx_renderer_renderer_color_effects_renderer_color_effects_type:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.colorEffectsRenderer.colorEffectsType"),
                )
                .expect("IOCore.gfxRenderer.renderer.colorEffectsRenderer.colorEffectsType"),
            iocore_gfx_renderer_renderer_color_effects_renderer_effects_target1:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.colorEffectsRenderer.effectsTarget1"),
                )
                .expect("IOCore.gfxRenderer.renderer.colorEffectsRenderer.effectsTarget1"),
            iocore_gfx_renderer_renderer_color_effects_renderer_effects_target2:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.colorEffectsRenderer.effectsTarget2"),
                )
                .expect("IOCore.gfxRenderer.renderer.colorEffectsRenderer.effectsTarget2"),
            iocore_gfx_renderer_renderer_compositor_do_effects: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.compositor.doEffects"),
            )
            .expect("IOCore.gfxRenderer.renderer.compositor.doEffects"),
            iocore_gfx_renderer_renderer_display: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.display"),
            )
            .expect("IOCore.gfxRenderer.renderer.display"),
            iocore_gfx_renderer_renderer_display_control: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.displayControl"),
            )
            .expect("IOCore.gfxRenderer.renderer.displayControl"),
            iocore_gfx_renderer_renderer_frame_buffer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.frameBuffer"),
            )
            .expect("IOCore.gfxRenderer.renderer.frameBuffer"),
            iocore_gfx_renderer_renderer_green_swap: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.greenSwap"),
            )
            .expect("IOCore.gfxRenderer.renderer.greenSwap"),
            iocore_gfx_renderer_renderer_last_unrendered_line: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.lastUnrenderedLine"),
            )
            .expect("IOCore.gfxRenderer.renderer.lastUnrenderedLine"),
            iocore_gfx_renderer_renderer_line_buffer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.lineBuffer"),
            )
            .expect("IOCore.gfxRenderer.renderer.lineBuffer"),
            iocore_gfx_renderer_renderer_mosaic_renderer_bgmosaic_hsize: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.mosaicRenderer.BGMosaicHSize"),
            )
            .expect("IOCore.gfxRenderer.renderer.mosaicRenderer.BGMosaicHSize"),
            iocore_gfx_renderer_renderer_mosaic_renderer_bgmosaic_vsize: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.mosaicRenderer.BGMosaicVSize"),
            )
            .expect("IOCore.gfxRenderer.renderer.mosaicRenderer.BGMosaicVSize"),
            iocore_gfx_renderer_renderer_mosaic_renderer_objmosaic_hsize: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.mosaicRenderer.OBJMosaicHSize"),
            )
            .expect("IOCore.gfxRenderer.renderer.mosaicRenderer.OBJMosaicHSize"),
            iocore_gfx_renderer_renderer_mosaic_renderer_objmosaic_vsize: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.mosaicRenderer.OBJMosaicVSize"),
            )
            .expect("IOCore.gfxRenderer.renderer.mosaicRenderer.OBJMosaicVSize"),
            iocore_gfx_renderer_renderer_obj_renderer_cycles_to_render: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.objRenderer.cyclesToRender"),
            )
            .expect("IOCore.gfxRenderer.renderer.objRenderer.cyclesToRender"),
            iocore_gfx_renderer_renderer_obj_renderer_oamram: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.objRenderer.OAMRAM"),
            )
            .expect("IOCore.gfxRenderer.renderer.objRenderer.OAMRAM"),
            // iocore_gfx_renderer_renderer_obj_renderer_oamram16: JsValueEncoder::decode(object.get("IOCore.gfxRenderer.renderer.objRenderer.OAMRAM16")).expect("IOCore.gfxRenderer.renderer.objRenderer.OAMRAM16"),
            // iocore_gfx_renderer_renderer_obj_renderer_oamram32: JsValueEncoder::decode(object.get("IOCore.gfxRenderer.renderer.objRenderer.OAMRAM32")).expect("IOCore.gfxRenderer.renderer.objRenderer.OAMRAM32"),
            iocore_gfx_renderer_renderer_obj_renderer_oamtable: OamTableEntry::decode_array(
                object.get_array("IOCore.gfxRenderer.renderer.objRenderer.OAMTable"),
            ),
            iocore_gfx_renderer_renderer_obj_renderer_objmatrix_parameters: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.objRenderer.OBJMatrixParameters"),
            )
            .expect("IOCore.gfxRenderer.renderer.objRenderer.OBJMatrixParameters"),
            iocore_gfx_renderer_renderer_obj_renderer_scratch_buffer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.objRenderer.scratchBuffer"),
            )
            .expect("IOCore.gfxRenderer.renderer.objRenderer.scratchBuffer"),
            iocore_gfx_renderer_renderer_obj_renderer_scratch_objbuffer: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.objRenderer.scratchOBJBuffer"),
            )
            .expect("IOCore.gfxRenderer.renderer.objRenderer.scratchOBJBuffer"),
            iocore_gfx_renderer_renderer_obj_renderer_scratch_window_buffer:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.objRenderer.scratchWindowBuffer"),
                )
                .expect("IOCore.gfxRenderer.renderer.objRenderer.scratchWindowBuffer"),
            iocore_gfx_renderer_renderer_obj_window_renderer_compositor_objwindow_buffer:
                JsValueEncoder::decode(object.get(
                    "IOCore.gfxRenderer.renderer.objWindowRenderer.compositor.OBJWindowBuffer",
                ))
                .expect("IOCore.gfxRenderer.renderer.objWindowRenderer.compositor.OBJWindowBuffer"),
            iocore_gfx_renderer_renderer_obj_window_renderer_winobjoutside: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.objWindowRenderer.WINOBJOutside"),
            )
            .expect("IOCore.gfxRenderer.renderer.objWindowRenderer.WINOBJOutside"),
            iocore_gfx_renderer_renderer_palette16: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.palette16"),
            )
            .expect("IOCore.gfxRenderer.renderer.palette16"),
            iocore_gfx_renderer_renderer_palette256: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.palette256"),
            )
            .expect("IOCore.gfxRenderer.renderer.palette256"),
            iocore_gfx_renderer_renderer_palette_obj16: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.paletteOBJ16"),
            )
            .expect("IOCore.gfxRenderer.renderer.paletteOBJ16"),
            iocore_gfx_renderer_renderer_palette_obj256: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.paletteOBJ256"),
            )
            .expect("IOCore.gfxRenderer.renderer.paletteOBJ256"),
            iocore_gfx_renderer_renderer_palette_ram: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.paletteRAM"),
            )
            .expect("IOCore.gfxRenderer.renderer.paletteRAM"),
            // iocore_gfx_renderer_renderer_palette_ram16: JsValueEncoder::decode(object.get("IOCore.gfxRenderer.renderer.paletteRAM16")).expect("IOCore.gfxRenderer.renderer.paletteRAM16"),
            // iocore_gfx_renderer_renderer_palette_ram32: JsValueEncoder::decode(object.get("IOCore.gfxRenderer.renderer.paletteRAM32")).expect("IOCore.gfxRenderer.renderer.paletteRAM32"),
            iocore_gfx_renderer_renderer_queued_scan_lines: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.queuedScanLines"),
            )
            .expect("IOCore.gfxRenderer.renderer.queuedScanLines"),
            iocore_gfx_renderer_renderer_swizzled_frame: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.swizzledFrame"),
            )
            .expect("IOCore.gfxRenderer.renderer.swizzledFrame"),
            iocore_gfx_renderer_renderer_total_lines_passed: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.totalLinesPassed"),
            )
            .expect("IOCore.gfxRenderer.renderer.totalLinesPassed"),
            iocore_gfx_renderer_renderer_vram: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.VRAM"),
            )
            .expect("IOCore.gfxRenderer.renderer.VRAM"),
            // iocore_gfx_renderer_renderer_vram16: JsValueEncoder::decode(object.get("IOCore.gfxRenderer.renderer.VRAM16")).expect("IOCore.gfxRenderer.renderer.VRAM16"),
            // iocore_gfx_renderer_renderer_vram32: JsValueEncoder::decode(object.get("IOCore.gfxRenderer.renderer.VRAM32")).expect("IOCore.gfxRenderer.renderer.VRAM32"),
            iocore_gfx_renderer_renderer_window0_renderer_compositor_do_effects:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.window0Renderer.compositor.doEffects"),
                )
                .expect("IOCore.gfxRenderer.renderer.window0Renderer.compositor.doEffects"),
            iocore_gfx_renderer_renderer_window0_renderer_window_display_control:
                JsValueEncoder::decode(
                    object.get("IOCore.gfxRenderer.renderer.window0Renderer.windowDisplayControl"),
                )
                .expect("IOCore.gfxRenderer.renderer.window0Renderer.windowDisplayControl"),
            iocore_gfx_renderer_renderer_window0_renderer_winxcoord_left: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.window0Renderer.WINXCoordLeft"),
            )
            .expect("IOCore.gfxRenderer.renderer.window0Renderer.WINXCoordLeft"),
            iocore_gfx_renderer_renderer_window0_renderer_winxcoord_right: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.window0Renderer.WINXCoordRight"),
            )
            .expect("IOCore.gfxRenderer.renderer.window0Renderer.WINXCoordRight"),
            iocore_gfx_renderer_renderer_window0_renderer_winycoord_bottom: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.window0Renderer.WINYCoordBottom"),
            )
            .expect("IOCore.gfxRenderer.renderer.window0Renderer.WINYCoordBottom"),
            iocore_gfx_renderer_renderer_window0_renderer_winycoord_top: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.window0Renderer.WINYCoordTop"),
            )
            .expect("IOCore.gfxRenderer.renderer.window0Renderer.WINYCoordTop"),
            iocore_gfx_renderer_renderer_winoutside: JsValueEncoder::decode(
                object.get("IOCore.gfxRenderer.renderer.WINOutside"),
            )
            .expect("IOCore.gfxRenderer.renderer.WINOutside"),
            iocore_gfx_state_current_scan_line: JsValueEncoder::decode(
                object.get("IOCore.gfxState.currentScanLine"),
            )
            .expect("IOCore.gfxState.currentScanLine"),
            iocore_gfx_state_irqflags: JsValueEncoder::decode(
                object.get("IOCore.gfxState.IRQFlags"),
            )
            .expect("IOCore.gfxState.IRQFlags"),
            iocore_gfx_state_lcdticks: JsValueEncoder::decode(
                object.get("IOCore.gfxState.LCDTicks"),
            )
            .expect("IOCore.gfxState.LCDTicks"),
            iocore_gfx_state_rendered_scan_line: JsValueEncoder::decode(
                object.get("IOCore.gfxState.renderedScanLine"),
            )
            .expect("IOCore.gfxState.renderedScanLine"),
            iocore_gfx_state_status_flags: JsValueEncoder::decode(
                object.get("IOCore.gfxState.statusFlags"),
            )
            .expect("IOCore.gfxState.statusFlags"),
            iocore_gfx_state_vcounter: JsValueEncoder::decode(
                object.get("IOCore.gfxState.VCounter"),
            )
            .expect("IOCore.gfxState.VCounter"),
            iocore_irq_interrupts_enabled: JsValueEncoder::decode(
                object.get("IOCore.irq.interruptsEnabled"),
            )
            .expect("IOCore.irq.interruptsEnabled"),
            iocore_irq_interrupts_requested: JsValueEncoder::decode(
                object.get("IOCore.irq.interruptsRequested"),
            )
            .expect("IOCore.irq.interruptsRequested"),
            iocore_joypad_key_input: JsValueEncoder::decode(object.get("IOCore.joypad.keyInput"))
                .expect("IOCore.joypad.keyInput"),
            iocore_joypad_key_interrupt: JsValueEncoder::decode(
                object.get("IOCore.joypad.keyInterrupt"),
            )
            .expect("IOCore.joypad.keyInterrupt"),
            iocore_memory_external_ram: JsValueEncoder::decode(
                object.get("IOCore.memory.externalRAM"),
            )
            .expect("IOCore.memory.externalRAM"),
            // iocore_memory_external_ram16: JsValueEncoder::decode(object.get("IOCore.memory.externalRAM16")).expect("IOCore.memory.externalRAM16"),
            // iocore_memory_external_ram32: JsValueEncoder::decode(object.get("IOCore.memory.externalRAM32")).expect("IOCore.memory.externalRAM32"),
            iocore_memory_internal_ram: JsValueEncoder::decode(
                object.get("IOCore.memory.internalRAM"),
            )
            .expect("IOCore.memory.internalRAM"),
            // iocore_memory_internal_ram16: JsValueEncoder::decode(object.get("IOCore.memory.internalRAM16")).expect("IOCore.memory.internalRAM16"),
            // iocore_memory_internal_ram32: JsValueEncoder::decode(object.get("IOCore.memory.internalRAM32")).expect("IOCore.memory.internalRAM32"),
            iocore_memory_irq_ime: JsValueEncoder::decode(object.get("IOCore.memory.irq.IME"))
                .expect("IOCore.memory.irq.IME"),
            iocore_memory_last_biosread: JsValueEncoder::decode(
                object.get("IOCore.memory.lastBIOSREAD"),
            )
            .expect("IOCore.memory.lastBIOSREAD"),
            iocore_memory_wramcontrol_flags: JsValueEncoder::decode(
                object.get("IOCore.memory.WRAMControlFlags"),
            )
            .expect("IOCore.memory.WRAMControlFlags"),
            iocore_next_event_clocks: JsValueEncoder::decode(object.get("IOCore.nextEventClocks"))
                .expect("IOCore.nextEventClocks"),
            iocore_saves_eepromchip_address: JsValueEncoder::decode(
                object.get("IOCore.saves.EEPROMChip.address"),
            )
            .expect("IOCore.saves.EEPROMChip.address"),
            iocore_saves_eepromchip_bits_processed: JsValueEncoder::decode(
                object.get("IOCore.saves.EEPROMChip.bitsProcessed"),
            )
            .expect("IOCore.saves.EEPROMChip.bitsProcessed"),
            iocore_saves_eepromchip_buffer: JsValueEncoder::decode(
                object.get("IOCore.saves.EEPROMChip.buffer"),
            )
            .expect("IOCore.saves.EEPROMChip.buffer"),
            iocore_saves_eepromchip_largest_size_possible: JsValueEncoder::decode(
                object.get("IOCore.saves.EEPROMChip.largestSizePossible"),
            )
            .expect("IOCore.saves.EEPROMChip.largestSizePossible"),
            iocore_saves_eepromchip_mode: JsValueEncoder::decode(
                object.get("IOCore.saves.EEPROMChip.mode"),
            )
            .expect("IOCore.saves.EEPROMChip.mode"),
            // iocore_saves_flashchip_bankoffset: JsValueEncoder::decode(
            //     object.get("IOCore.saves.FLASHChip.BANKOffset"),
            // )
            // .expect("IOCore.saves.FLASHChip.BANKOffset"),
            // iocore_saves_flashchip_flash_command: JsValueEncoder::decode(
            //     object.get("IOCore.saves.FLASHChip.flashCommand"),
            // )
            // .expect("IOCore.saves.FLASHChip.flashCommand"),
            // iocore_saves_flashchip_flash_command_unlock_stage: JsValueEncoder::decode(
            //     object.get("IOCore.saves.FLASHChip.flashCommandUnlockStage"),
            // )
            // .expect("IOCore.saves.FLASHChip.flashCommandUnlockStage"),
            // iocore_saves_flashchip_largest_size_possible: JsValueEncoder::decode(
            //     object.get("IOCore.saves.FLASHChip.largestSizePossible"),
            // )
            // .expect("IOCore.saves.FLASHChip.largestSizePossible"),
            // iocore_saves_flashchip_not_atmel: JsValueEncoder::decode(
            //     object.get("IOCore.saves.FLASHChip.notATMEL"),
            // )
            // .expect("IOCore.saves.FLASHChip.notATMEL"),
            // iocore_saves_flashchip_saves: JsValueEncoder::decode(
            //     object.get("IOCore.saves.FLASHChip.saves"),
            // )
            // .expect("IOCore.saves.FLASHChip.saves"),
            // iocore_saves_flashchip_write_bytes_left: JsValueEncoder::decode(
            //     object.get("IOCore.saves.FLASHChip.writeBytesLeft"),
            // )
            // .expect("IOCore.saves.FLASHChip.writeBytesLeft"),
            iocore_saves_gpiochip_data: JsValueEncoder::decode(
                object.get("IOCore.saves.GPIOChip.data"),
            )
            .expect("IOCore.saves.GPIOChip.data"),
            iocore_saves_gpiochip_direction: JsValueEncoder::decode(
                object.get("IOCore.saves.GPIOChip.direction"),
            )
            .expect("IOCore.saves.GPIOChip.direction"),
            iocore_saves_gpiochip_read_write: JsValueEncoder::decode(
                object.get("IOCore.saves.GPIOChip.readWrite"),
            )
            .expect("IOCore.saves.GPIOChip.readWrite"),
            iocore_saves_gpiochip_type: JsValueEncoder::decode(
                object.get("IOCore.saves.GPIOChip.type"),
            )
            .expect("IOCore.saves.GPIOChip.type"),
            iocore_saves_save_type: JsValueEncoder::decode(object.get("IOCore.saves.saveType"))
                .expect("IOCore.saves.saveType"),
            iocore_saves_undetermined_possible: JsValueEncoder::decode(
                object.get("IOCore.saves.UNDETERMINED.possible"),
            )
            .expect("IOCore.saves.UNDETERMINED.possible"),
            iocore_serial_joybus_cntl_flags: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_CNTL_FLAGS"),
            )
            .expect("IOCore.serial.JOYBUS_CNTL_FLAGS"),
            iocore_serial_joybus_irq: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_IRQ"),
            )
            .expect("IOCore.serial.JOYBUS_IRQ"),
            iocore_serial_joybus_recv0: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_RECV0"),
            )
            .expect("IOCore.serial.JOYBUS_RECV0"),
            iocore_serial_joybus_recv1: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_RECV1"),
            )
            .expect("IOCore.serial.JOYBUS_RECV1"),
            iocore_serial_joybus_recv2: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_RECV2"),
            )
            .expect("IOCore.serial.JOYBUS_RECV2"),
            iocore_serial_joybus_recv3: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_RECV3"),
            )
            .expect("IOCore.serial.JOYBUS_RECV3"),
            iocore_serial_joybus_send0: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_SEND0"),
            )
            .expect("IOCore.serial.JOYBUS_SEND0"),
            iocore_serial_joybus_send1: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_SEND1"),
            )
            .expect("IOCore.serial.JOYBUS_SEND1"),
            iocore_serial_joybus_send2: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_SEND2"),
            )
            .expect("IOCore.serial.JOYBUS_SEND2"),
            iocore_serial_joybus_send3: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_SEND3"),
            )
            .expect("IOCore.serial.JOYBUS_SEND3"),
            iocore_serial_joybus_stat: JsValueEncoder::decode(
                object.get("IOCore.serial.JOYBUS_STAT"),
            )
            .expect("IOCore.serial.JOYBUS_STAT"),
            iocore_serial_rcntdata_bit_flow: JsValueEncoder::decode(
                object.get("IOCore.serial.RCNTDataBitFlow"),
            )
            .expect("IOCore.serial.RCNTDataBitFlow"),
            iocore_serial_rcntdata_bits: JsValueEncoder::decode(
                object.get("IOCore.serial.RCNTDataBits"),
            )
            .expect("IOCore.serial.RCNTDataBits"),
            iocore_serial_rcntirq: JsValueEncoder::decode(object.get("IOCore.serial.RCNTIRQ"))
                .expect("IOCore.serial.RCNTIRQ"),
            iocore_serial_rcntmode: JsValueEncoder::decode(object.get("IOCore.serial.RCNTMode"))
                .expect("IOCore.serial.RCNTMode"),
            iocore_serial_serial_bits_shifted: JsValueEncoder::decode(
                object.get("IOCore.serial.serialBitsShifted"),
            )
            .expect("IOCore.serial.serialBitsShifted"),
            iocore_serial_shift_clocks: JsValueEncoder::decode(
                object.get("IOCore.serial.shiftClocks"),
            )
            .expect("IOCore.serial.shiftClocks"),
            iocore_serial_siobaud_rate: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOBaudRate"),
            )
            .expect("IOCore.serial.SIOBaudRate"),
            iocore_serial_siocnt_irq: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT_IRQ"),
            )
            .expect("IOCore.serial.SIOCNT_IRQ"),
            iocore_serial_siocnt_mode: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT_MODE"),
            )
            .expect("IOCore.serial.SIOCNT_MODE"),
            iocore_serial_siocnt_uart_cts: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT_UART_CTS"),
            )
            .expect("IOCore.serial.SIOCNT_UART_CTS"),
            iocore_serial_siocnt_uart_fifo: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT_UART_FIFO"),
            )
            .expect("IOCore.serial.SIOCNT_UART_FIFO"),
            iocore_serial_siocnt_uart_fifo_enable: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT_UART_FIFO_ENABLE"),
            )
            .expect("IOCore.serial.SIOCNT_UART_FIFO_ENABLE"),
            iocore_serial_siocnt_uart_misc: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT_UART_MISC"),
            )
            .expect("IOCore.serial.SIOCNT_UART_MISC"),
            iocore_serial_siocnt_uart_parity_enable: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT_UART_PARITY_ENABLE"),
            )
            .expect("IOCore.serial.SIOCNT_UART_PARITY_ENABLE"),
            iocore_serial_siocnt_uart_recv_enable: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT_UART_RECV_ENABLE"),
            )
            .expect("IOCore.serial.SIOCNT_UART_RECV_ENABLE"),
            iocore_serial_siocnt_uart_send_enable: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT_UART_SEND_ENABLE"),
            )
            .expect("IOCore.serial.SIOCNT_UART_SEND_ENABLE"),
            iocore_serial_siocnt0_data: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCNT0_DATA"),
            )
            .expect("IOCore.serial.SIOCNT0_DATA"),
            iocore_serial_siocommerror: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOCOMMERROR"),
            )
            .expect("IOCore.serial.SIOCOMMERROR"),
            iocore_serial_siodata_a: JsValueEncoder::decode(object.get("IOCore.serial.SIODATA_A"))
                .expect("IOCore.serial.SIODATA_A"),
            iocore_serial_siodata_b: JsValueEncoder::decode(object.get("IOCore.serial.SIODATA_B"))
                .expect("IOCore.serial.SIODATA_B"),
            iocore_serial_siodata_c: JsValueEncoder::decode(object.get("IOCore.serial.SIODATA_C"))
                .expect("IOCore.serial.SIODATA_C"),
            iocore_serial_siodata_d: JsValueEncoder::decode(object.get("IOCore.serial.SIODATA_D"))
                .expect("IOCore.serial.SIODATA_D"),
            iocore_serial_siodata8: JsValueEncoder::decode(object.get("IOCore.serial.SIODATA8"))
                .expect("IOCore.serial.SIODATA8"),
            iocore_serial_siomult_player_number: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOMULT_PLAYER_NUMBER"),
            )
            .expect("IOCore.serial.SIOMULT_PLAYER_NUMBER"),
            iocore_serial_sioshift_clock_divider: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOShiftClockDivider"),
            )
            .expect("IOCore.serial.SIOShiftClockDivider"),
            iocore_serial_sioshift_clock_external: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOShiftClockExternal"),
            )
            .expect("IOCore.serial.SIOShiftClockExternal"),
            iocore_serial_siotransfer_started: JsValueEncoder::decode(
                object.get("IOCore.serial.SIOTransferStarted"),
            )
            .expect("IOCore.serial.SIOTransferStarted"),
            iocore_serial_clocks: JsValueEncoder::decode(object.get("IOCore.serialClocks"))
                .expect("IOCore.serialClocks"),
            // iocore_sound_agbdirect_sound_a: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundA"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundA"),
            // iocore_sound_agbdirect_sound_afolded: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundAFolded"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundAFolded"),
            // iocore_sound_agbdirect_sound_aleft_can_play: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundALeftCanPlay"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundALeftCanPlay"),
            // iocore_sound_agbdirect_sound_aright_can_play: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundARightCanPlay"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundARightCanPlay"),
            // iocore_sound_agbdirect_sound_ashifter: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundAShifter"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundAShifter"),
            // iocore_sound_agbdirect_sound_atimer: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundATimer"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundATimer"),
            // iocore_sound_agbdirect_sound_b: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundB"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundB"),
            // iocore_sound_agbdirect_sound_bfolded: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundBFolded"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundBFolded"),
            // iocore_sound_agbdirect_sound_bleft_can_play: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundBLeftCanPlay"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundBLeftCanPlay"),
            // iocore_sound_agbdirect_sound_bright_can_play: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundBRightCanPlay"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundBRightCanPlay"),
            // iocore_sound_agbdirect_sound_bshifter: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundBShifter"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundBShifter"),
            // iocore_sound_agbdirect_sound_btimer: JsValueEncoder::decode(
            //     object.get("IOCore.sound.AGBDirectSoundBTimer"),
            // )
            // .expect("IOCore.sound.AGBDirectSoundBTimer"),
            // iocore_sound_audio_clocks_until_next_event: JsValueEncoder::decode(
            //     object.get("IOCore.sound.audioClocksUntilNextEvent"),
            // )
            // .expect("IOCore.sound.audioClocksUntilNextEvent"),
            // iocore_sound_audio_clocks_until_next_event_counter: JsValueEncoder::decode(
            //     object.get("IOCore.sound.audioClocksUntilNextEventCounter"),
            // )
            // .expect("IOCore.sound.audioClocksUntilNextEventCounter"),
            // iocore_sound_audio_index: JsValueEncoder::decode(object.get("IOCore.sound.audioIndex"))
            //     .expect("IOCore.sound.audioIndex"),
            // iocore_sound_audio_resampler_first_pass_factor: JsValueEncoder::decode(
            //     object.get("IOCore.sound.audioResamplerFirstPassFactor"),
            // )
            // .expect("IOCore.sound.audioResamplerFirstPassFactor"),
            // iocore_sound_audio_ticks: JsValueEncoder::decode(object.get("IOCore.sound.audioTicks"))
            //     .expect("IOCore.sound.audioTicks"),
            // iocore_sound_cgbmixer_output_cache_left: JsValueEncoder::decode(
            //     object.get("IOCore.sound.CGBMixerOutputCacheLeft"),
            // )
            // .expect("IOCore.sound.CGBMixerOutputCacheLeft"),
            // iocore_sound_cgbmixer_output_cache_left_folded: JsValueEncoder::decode(
            //     object.get("IOCore.sound.CGBMixerOutputCacheLeftFolded"),
            // )
            // .expect("IOCore.sound.CGBMixerOutputCacheLeftFolded"),
            // iocore_sound_cgbmixer_output_cache_right: JsValueEncoder::decode(
            //     object.get("IOCore.sound.CGBMixerOutputCacheRight"),
            // )
            // .expect("IOCore.sound.CGBMixerOutputCacheRight"),
            // iocore_sound_cgbmixer_output_cache_right_folded: JsValueEncoder::decode(
            //     object.get("IOCore.sound.CGBMixerOutputCacheRightFolded"),
            // )
            // .expect("IOCore.sound.CGBMixerOutputCacheRightFolded"),
            // iocore_sound_cgboutput_ratio: JsValueEncoder::decode(
            //     object.get("IOCore.sound.CGBOutputRatio"),
            // )
            // .expect("IOCore.sound.CGBOutputRatio"),
            // iocore_sound_channel1_cached_duty: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.CachedDuty"),
            // )
            // .expect("IOCore.sound.channel1.CachedDuty"),
            // iocore_sound_channel1_can_play: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.canPlay"),
            // )
            // .expect("IOCore.sound.channel1.canPlay"),
            // iocore_sound_channel1_consecutive: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.consecutive"),
            // )
            // .expect("IOCore.sound.channel1.consecutive"),
            // iocore_sound_channel1_current_sample_left: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.currentSampleLeft"),
            // )
            // .expect("IOCore.sound.channel1.currentSampleLeft"),
            // iocore_sound_channel1_current_sample_right: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.currentSampleRight"),
            // )
            // .expect("IOCore.sound.channel1.currentSampleRight"),
            // iocore_sound_channel1_decrease_sweep: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.decreaseSweep"),
            // )
            // .expect("IOCore.sound.channel1.decreaseSweep"),
            // iocore_sound_channel1_duty_tracker: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.DutyTracker"),
            // )
            // .expect("IOCore.sound.channel1.DutyTracker"),
            // iocore_sound_channel1_enabled: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.Enabled"),
            // )
            // .expect("IOCore.sound.channel1.Enabled"),
            // iocore_sound_channel1_envelope_sweeps: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.envelopeSweeps"),
            // )
            // .expect("IOCore.sound.channel1.envelopeSweeps"),
            // iocore_sound_channel1_envelope_sweeps_last: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.envelopeSweepsLast"),
            // )
            // .expect("IOCore.sound.channel1.envelopeSweepsLast"),
            // iocore_sound_channel1_envelope_volume: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.envelopeVolume"),
            // )
            // .expect("IOCore.sound.channel1.envelopeVolume"),
            // iocore_sound_channel1_frequency: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.frequency"),
            // )
            // .expect("IOCore.sound.channel1.frequency"),
            // iocore_sound_channel1_frequency_counter: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.FrequencyCounter"),
            // )
            // .expect("IOCore.sound.channel1.FrequencyCounter"),
            // iocore_sound_channel1_frequency_sweep_divider: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.frequencySweepDivider"),
            // )
            // .expect("IOCore.sound.channel1.frequencySweepDivider"),
            // iocore_sound_channel1_frequency_tracker: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.FrequencyTracker"),
            // )
            // .expect("IOCore.sound.channel1.FrequencyTracker"),
            // iocore_sound_channel1_last_time_sweep: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.lastTimeSweep"),
            // )
            // .expect("IOCore.sound.channel1.lastTimeSweep"),
            // iocore_sound_channel1_left_enable: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.leftEnable"),
            // )
            // .expect("IOCore.sound.channel1.leftEnable"),
            // iocore_sound_channel1_nr10: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.nr10"),
            // )
            // .expect("IOCore.sound.channel1.nr10"),
            // iocore_sound_channel1_nr11: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.nr11"),
            // )
            // .expect("IOCore.sound.channel1.nr11"),
            // iocore_sound_channel1_nr12: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.nr12"),
            // )
            // .expect("IOCore.sound.channel1.nr12"),
            // iocore_sound_channel1_nr14: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.nr14"),
            // )
            // .expect("IOCore.sound.channel1.nr14"),
            // iocore_sound_channel1_right_enable: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.rightEnable"),
            // )
            // .expect("IOCore.sound.channel1.rightEnable"),
            // iocore_sound_channel1_shadow_frequency: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.ShadowFrequency"),
            // )
            // .expect("IOCore.sound.channel1.ShadowFrequency"),
            // iocore_sound_channel1_sweep_fault: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.SweepFault"),
            // )
            // .expect("IOCore.sound.channel1.SweepFault"),
            // iocore_sound_channel1_swept: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.Swept"),
            // )
            // .expect("IOCore.sound.channel1.Swept"),
            // iocore_sound_channel1_time_sweep: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.timeSweep"),
            // )
            // .expect("IOCore.sound.channel1.timeSweep"),
            // iocore_sound_channel1_total_length: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel1.totalLength"),
            // )
            // .expect("IOCore.sound.channel1.totalLength"),
            // iocore_sound_channel2_nr21: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel2.nr21"),
            // )
            // .expect("IOCore.sound.channel2.nr21"),
            // iocore_sound_channel2_nr22: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel2.nr22"),
            // )
            // .expect("IOCore.sound.channel2.nr22"),
            // iocore_sound_channel2_nr23: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel2.nr23"),
            // )
            // .expect("IOCore.sound.channel2.nr23"),
            // iocore_sound_channel2_nr24: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel2.nr24"),
            // )
            // .expect("IOCore.sound.channel2.nr24"),
            // iocore_sound_channel2_shadow_frequency: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel2.ShadowFrequency"),
            // )
            // .expect("IOCore.sound.channel2.ShadowFrequency"),
            // iocore_sound_channel3_cached_sample: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.cachedSample"),
            // )
            // .expect("IOCore.sound.channel3.cachedSample"),
            // iocore_sound_channel3_can_play: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.canPlay"),
            // )
            // .expect("IOCore.sound.channel3.canPlay"),
            // iocore_sound_channel3_counter: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.counter"),
            // )
            // .expect("IOCore.sound.channel3.counter"),
            // iocore_sound_channel3_frequency_period: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.FrequencyPeriod"),
            // )
            // .expect("IOCore.sound.channel3.FrequencyPeriod"),
            // iocore_sound_channel3_last_sample_lookup: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.lastSampleLookup"),
            // )
            // .expect("IOCore.sound.channel3.lastSampleLookup"),
            // iocore_sound_channel3_nr30: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.nr30"),
            // )
            // .expect("IOCore.sound.channel3.nr30"),
            // iocore_sound_channel3_nr31: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.nr31"),
            // )
            // .expect("IOCore.sound.channel3.nr31"),
            // iocore_sound_channel3_nr32: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.nr32"),
            // )
            // .expect("IOCore.sound.channel3.nr32"),
            // iocore_sound_channel3_nr33: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.nr33"),
            // )
            // .expect("IOCore.sound.channel3.nr33"),
            // iocore_sound_channel3_nr34: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.nr34"),
            // )
            // .expect("IOCore.sound.channel3.nr34"),
            // iocore_sound_channel3_pattern_type: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.patternType"),
            // )
            // .expect("IOCore.sound.channel3.patternType"),
            // iocore_sound_channel3_pcm: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.PCM"),
            // )
            // .expect("IOCore.sound.channel3.PCM"),
            // iocore_sound_channel3_pcm16: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.PCM16"),
            // )
            // .expect("IOCore.sound.channel3.PCM16"),
            // iocore_sound_channel3_pcm32: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.PCM32"),
            // )
            // .expect("IOCore.sound.channel3.PCM32"),
            // iocore_sound_channel3_waveram16: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.WAVERAM16"),
            // )
            // .expect("IOCore.sound.channel3.WAVERAM16"),
            // iocore_sound_channel3_waveram32: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.WAVERAM32"),
            // )
            // .expect("IOCore.sound.channel3.WAVERAM32"),
            // iocore_sound_channel3_waveram8: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.WAVERAM8"),
            // )
            // .expect("IOCore.sound.channel3.WAVERAM8"),
            // iocore_sound_channel3_waverambank_accessed: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.WAVERAMBankAccessed"),
            // )
            // .expect("IOCore.sound.channel3.WAVERAMBankAccessed"),
            // iocore_sound_channel3_wave_rambank_size: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.WaveRAMBankSize"),
            // )
            // .expect("IOCore.sound.channel3.WaveRAMBankSize"),
            // iocore_sound_channel3_waverambank_specified: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel3.WAVERAMBankSpecified"),
            // )
            // .expect("IOCore.sound.channel3.WAVERAMBankSpecified"),
            // iocore_sound_channel4_bit_range: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.BitRange"),
            // )
            // .expect("IOCore.sound.channel4.BitRange"),
            // iocore_sound_channel4_counter: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.counter"),
            // )
            // .expect("IOCore.sound.channel4.counter"),
            // iocore_sound_channel4_current_volume: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.currentVolume"),
            // )
            // .expect("IOCore.sound.channel4.currentVolume"),
            // iocore_sound_channel4_frequency_period: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.FrequencyPeriod"),
            // )
            // .expect("IOCore.sound.channel4.FrequencyPeriod"),
            // iocore_sound_channel4_last_sample_lookup: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.lastSampleLookup"),
            // )
            // .expect("IOCore.sound.channel4.lastSampleLookup"),
            // iocore_sound_channel4_lsfr15_table: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.LSFR15Table"),
            // )
            // .expect("IOCore.sound.channel4.LSFR15Table"),
            // iocore_sound_channel4_lsfr7_table: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.LSFR7Table"),
            // )
            // .expect("IOCore.sound.channel4.LSFR7Table"),
            // iocore_sound_channel4_noise_sample_table: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.noiseSampleTable"),
            // )
            // .expect("IOCore.sound.channel4.noiseSampleTable"),
            // iocore_sound_channel4_nr42: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.nr42"),
            // )
            // .expect("IOCore.sound.channel4.nr42"),
            // iocore_sound_channel4_nr43: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.nr43"),
            // )
            // .expect("IOCore.sound.channel4.nr43"),
            // iocore_sound_channel4_nr44: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.nr44"),
            // )
            // .expect("IOCore.sound.channel4.nr44"),
            // iocore_sound_channel4_volume_shifter: JsValueEncoder::decode(
            //     object.get("IOCore.sound.channel4.VolumeShifter"),
            // )
            // .expect("IOCore.sound.channel4.VolumeShifter"),
            // iocore_sound_downsample_input_left: JsValueEncoder::decode(
            //     object.get("IOCore.sound.downsampleInputLeft"),
            // )
            // .expect("IOCore.sound.downsampleInputLeft"),
            // iocore_sound_downsample_input_right: JsValueEncoder::decode(
            //     object.get("IOCore.sound.downsampleInputRight"),
            // )
            // .expect("IOCore.sound.downsampleInputRight"),
            // iocore_sound_fifoabuffer_buffer: JsValueEncoder::decode(
            //     object.get("IOCore.sound.FIFOABuffer.buffer"),
            // )
            // .expect("IOCore.sound.FIFOABuffer.buffer"),
            // iocore_sound_fifoabuffer_count: JsValueEncoder::decode(
            //     object.get("IOCore.sound.FIFOABuffer.count"),
            // )
            // .expect("IOCore.sound.FIFOABuffer.count"),
            // iocore_sound_fifoabuffer_position: JsValueEncoder::decode(
            //     object.get("IOCore.sound.FIFOABuffer.position"),
            // )
            // .expect("IOCore.sound.FIFOABuffer.position"),
            // iocore_sound_fifobbuffer_buffer: JsValueEncoder::decode(
            //     object.get("IOCore.sound.FIFOBBuffer.buffer"),
            // )
            // .expect("IOCore.sound.FIFOBBuffer.buffer"),
            // iocore_sound_mixer_output_cache_left: JsValueEncoder::decode(
            //     object.get("IOCore.sound.mixerOutputCacheLeft"),
            // )
            // .expect("IOCore.sound.mixerOutputCacheLeft"),
            // iocore_sound_mixer_output_cache_right: JsValueEncoder::decode(
            //     object.get("IOCore.sound.mixerOutputCacheRight"),
            // )
            // .expect("IOCore.sound.mixerOutputCacheRight"),
            // iocore_sound_mixer_sound_bias: JsValueEncoder::decode(
            //     object.get("IOCore.sound.mixerSoundBIAS"),
            // )
            // .expect("IOCore.sound.mixerSoundBIAS"),
            // iocore_sound_nr50: JsValueEncoder::decode(object.get("IOCore.sound.nr50"))
            //     .expect("IOCore.sound.nr50"),
            // iocore_sound_nr51: JsValueEncoder::decode(object.get("IOCore.sound.nr51"))
            //     .expect("IOCore.sound.nr51"),
            // iocore_sound_nr52: JsValueEncoder::decode(object.get("IOCore.sound.nr52"))
            //     .expect("IOCore.sound.nr52"),
            // iocore_sound_nr60: JsValueEncoder::decode(object.get("IOCore.sound.nr60"))
            //     .expect("IOCore.sound.nr60"),
            // iocore_sound_nr61: JsValueEncoder::decode(object.get("IOCore.sound.nr61"))
            //     .expect("IOCore.sound.nr61"),
            // iocore_sound_nr62: JsValueEncoder::decode(object.get("IOCore.sound.nr62"))
            //     .expect("IOCore.sound.nr62"),
            // iocore_sound_nr63: JsValueEncoder::decode(object.get("IOCore.sound.nr63"))
            //     .expect("IOCore.sound.nr63"),
            // iocore_sound_pwmbit_depth_mask: JsValueEncoder::decode(
            //     object.get("IOCore.sound.PWMBitDepthMask"),
            // )
            // .expect("IOCore.sound.PWMBitDepthMask"),
            // iocore_sound_pwmbit_depth_mask_shadow: JsValueEncoder::decode(
            //     object.get("IOCore.sound.PWMBitDepthMaskShadow"),
            // )
            // .expect("IOCore.sound.PWMBitDepthMaskShadow"),
            // iocore_sound_pwmwidth: JsValueEncoder::decode(object.get("IOCore.sound.PWMWidth"))
            //     .expect("IOCore.sound.PWMWidth"),
            // iocore_sound_pwmwidth_old: JsValueEncoder::decode(
            //     object.get("IOCore.sound.PWMWidthOld"),
            // )
            // .expect("IOCore.sound.PWMWidthOld"),
            // iocore_sound_pwmwidth_shadow: JsValueEncoder::decode(
            //     object.get("IOCore.sound.PWMWidthShadow"),
            // )
            // .expect("IOCore.sound.PWMWidthShadow"),
            // iocore_sound_sequence_position: JsValueEncoder::decode(
            //     object.get("IOCore.sound.sequencePosition"),
            // )
            // .expect("IOCore.sound.sequencePosition"),
            // iocore_sound_sequencer_clocks: JsValueEncoder::decode(
            //     object.get("IOCore.sound.sequencerClocks"),
            // )
            // .expect("IOCore.sound.sequencerClocks"),
            // iocore_sound_sound_master_enabled: JsValueEncoder::decode(
            //     object.get("IOCore.sound.soundMasterEnabled"),
            // )
            // .expect("IOCore.sound.soundMasterEnabled"),
            // iocore_sound_vin_left_channel_master_volume: JsValueEncoder::decode(
            //     object.get("IOCore.sound.VinLeftChannelMasterVolume"),
            // )
            // .expect("IOCore.sound.VinLeftChannelMasterVolume"),
            // iocore_sound_vin_right_channel_master_volume: JsValueEncoder::decode(
            //     object.get("IOCore.sound.VinRightChannelMasterVolume"),
            // )
            // .expect("IOCore.sound.VinRightChannelMasterVolume"),
            iocore_system_status: JsValueEncoder::decode(object.get("IOCore.systemStatus"))
                .expect("IOCore.systemStatus"),
            iocore_thumb_decode: JsValueEncoder::decode(object.get("IOCore.THUMB.decode"))
                .expect("IOCore.THUMB.decode"),
            iocore_thumb_execute: JsValueEncoder::decode(object.get("IOCore.THUMB.execute"))
                .expect("IOCore.THUMB.execute"),
            iocore_thumb_fetch: JsValueEncoder::decode(object.get("IOCore.THUMB.fetch"))
                .expect("IOCore.THUMB.fetch"),
            iocore_timer_timer0_control: JsValueEncoder::decode(
                object.get("IOCore.timer.timer0Control"),
            )
            .expect("IOCore.timer.timer0Control"),
            iocore_timer_timer0_counter: JsValueEncoder::decode(
                object.get("IOCore.timer.timer0Counter"),
            )
            .expect("IOCore.timer.timer0Counter"),
            iocore_timer_timer0_enabled: JsValueEncoder::decode(
                object.get("IOCore.timer.timer0Enabled"),
            )
            .expect("IOCore.timer.timer0Enabled"),
            iocore_timer_timer0_irq: JsValueEncoder::decode(object.get("IOCore.timer.timer0IRQ"))
                .expect("IOCore.timer.timer0IRQ"),
            iocore_timer_timer0_precounter: JsValueEncoder::decode(
                object.get("IOCore.timer.timer0Precounter"),
            )
            .expect("IOCore.timer.timer0Precounter"),
            iocore_timer_timer0_prescalar: JsValueEncoder::decode(
                object.get("IOCore.timer.timer0Prescalar"),
            )
            .expect("IOCore.timer.timer0Prescalar"),
            iocore_timer_timer0_prescalar_shifted: JsValueEncoder::decode(
                object.get("IOCore.timer.timer0PrescalarShifted"),
            )
            .expect("IOCore.timer.timer0PrescalarShifted"),
            iocore_timer_timer0_reload: JsValueEncoder::decode(
                object.get("IOCore.timer.timer0Reload"),
            )
            .expect("IOCore.timer.timer0Reload"),
            iocore_timer_timer1_control: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1Control"),
            )
            .expect("IOCore.timer.timer1Control"),
            iocore_timer_timer1_counter: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1Counter"),
            )
            .expect("IOCore.timer.timer1Counter"),
            iocore_timer_timer1_count_up: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1CountUp"),
            )
            .expect("IOCore.timer.timer1CountUp"),
            iocore_timer_timer1_enabled: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1Enabled"),
            )
            .expect("IOCore.timer.timer1Enabled"),
            iocore_timer_timer1_irq: JsValueEncoder::decode(object.get("IOCore.timer.timer1IRQ"))
                .expect("IOCore.timer.timer1IRQ"),
            iocore_timer_timer1_precounter: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1Precounter"),
            )
            .expect("IOCore.timer.timer1Precounter"),
            iocore_timer_timer1_prescalar: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1Prescalar"),
            )
            .expect("IOCore.timer.timer1Prescalar"),
            iocore_timer_timer1_prescalar_shifted: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1PrescalarShifted"),
            )
            .expect("IOCore.timer.timer1PrescalarShifted"),
            iocore_timer_timer1_reload: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1Reload"),
            )
            .expect("IOCore.timer.timer1Reload"),
            iocore_timer_timer1_use_chained_clocks: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1UseChainedClocks"),
            )
            .expect("IOCore.timer.timer1UseChainedClocks"),
            iocore_timer_timer1_use_main_clocks: JsValueEncoder::decode(
                object.get("IOCore.timer.timer1UseMainClocks"),
            )
            .expect("IOCore.timer.timer1UseMainClocks"),
            iocore_timer_timer2_control: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2Control"),
            )
            .expect("IOCore.timer.timer2Control"),
            iocore_timer_timer2_counter: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2Counter"),
            )
            .expect("IOCore.timer.timer2Counter"),
            iocore_timer_timer2_count_up: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2CountUp"),
            )
            .expect("IOCore.timer.timer2CountUp"),
            iocore_timer_timer2_enabled: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2Enabled"),
            )
            .expect("IOCore.timer.timer2Enabled"),
            iocore_timer_timer2_irq: JsValueEncoder::decode(object.get("IOCore.timer.timer2IRQ"))
                .expect("IOCore.timer.timer2IRQ"),
            iocore_timer_timer2_precounter: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2Precounter"),
            )
            .expect("IOCore.timer.timer2Precounter"),
            iocore_timer_timer2_prescalar: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2Prescalar"),
            )
            .expect("IOCore.timer.timer2Prescalar"),
            iocore_timer_timer2_prescalar_shifted: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2PrescalarShifted"),
            )
            .expect("IOCore.timer.timer2PrescalarShifted"),
            iocore_timer_timer2_reload: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2Reload"),
            )
            .expect("IOCore.timer.timer2Reload"),
            iocore_timer_timer2_use_chained_clocks: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2UseChainedClocks"),
            )
            .expect("IOCore.timer.timer2UseChainedClocks"),
            iocore_timer_timer2_use_main_clocks: JsValueEncoder::decode(
                object.get("IOCore.timer.timer2UseMainClocks"),
            )
            .expect("IOCore.timer.timer2UseMainClocks"),
            iocore_timer_timer3_control: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3Control"),
            )
            .expect("IOCore.timer.timer3Control"),
            iocore_timer_timer3_counter: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3Counter"),
            )
            .expect("IOCore.timer.timer3Counter"),
            iocore_timer_timer3_count_up: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3CountUp"),
            )
            .expect("IOCore.timer.timer3CountUp"),
            iocore_timer_timer3_enabled: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3Enabled"),
            )
            .expect("IOCore.timer.timer3Enabled"),
            iocore_timer_timer3_irq: JsValueEncoder::decode(object.get("IOCore.timer.timer3IRQ"))
                .expect("IOCore.timer.timer3IRQ"),
            iocore_timer_timer3_precounter: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3Precounter"),
            )
            .expect("IOCore.timer.timer3Precounter"),
            iocore_timer_timer3_prescalar: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3Prescalar"),
            )
            .expect("IOCore.timer.timer3Prescalar"),
            iocore_timer_timer3_prescalar_shifted: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3PrescalarShifted"),
            )
            .expect("IOCore.timer.timer3PrescalarShifted"),
            iocore_timer_timer3_reload: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3Reload"),
            )
            .expect("IOCore.timer.timer3Reload"),
            iocore_timer_timer3_use_chained_clocks: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3UseChainedClocks"),
            )
            .expect("IOCore.timer.timer3UseChainedClocks"),
            iocore_timer_timer3_use_main_clocks: JsValueEncoder::decode(
                object.get("IOCore.timer.timer3UseMainClocks"),
            )
            .expect("IOCore.timer.timer3UseMainClocks"),
            iocore_timer_clocks: JsValueEncoder::decode(object.get("IOCore.timerClocks"))
                .expect("IOCore.timerClocks"),
            iocore_wait_buffer: JsValueEncoder::decode(object.get("IOCore.wait.buffer"))
                .expect("IOCore.wait.buffer"),
            iocore_wait_clocks: JsValueEncoder::decode(object.get("IOCore.wait.clocks"))
                .expect("IOCore.wait.clocks"),
            iocore_wait_is_oamrendering: JsValueEncoder::decode(
                object.get("IOCore.wait.isOAMRendering"),
            )
            .expect("IOCore.wait.isOAMRendering"),
            iocore_wait_is_rendering: JsValueEncoder::decode(object.get("IOCore.wait.isRendering"))
                .expect("IOCore.wait.isRendering"),
            iocore_wait_non_sequential: JsValueEncoder::decode(
                object.get("IOCore.wait.nonSequential"),
            )
            .expect("IOCore.wait.nonSequential"),
            iocore_wait_postboot: JsValueEncoder::decode(object.get("IOCore.wait.POSTBOOT"))
                .expect("IOCore.wait.POSTBOOT"),
            iocore_wait_sramwait_state: JsValueEncoder::decode(
                object.get("IOCore.wait.SRAMWaitState"),
            )
            .expect("IOCore.wait.SRAMWaitState"),
            iocore_wait_waitcnt0: JsValueEncoder::decode(object.get("IOCore.wait.WAITCNT0"))
                .expect("IOCore.wait.WAITCNT0"),
            iocore_wait_waitcnt1: JsValueEncoder::decode(object.get("IOCore.wait.WAITCNT1"))
                .expect("IOCore.wait.WAITCNT1"),
            iocore_wait_wait_state_clocks16: JsValueEncoder::decode(
                object.get("IOCore.wait.waitStateClocks16"),
            )
            .expect("IOCore.wait.waitStateClocks16"),
            iocore_wait_wait_state_clocks32: JsValueEncoder::decode(
                object.get("IOCore.wait.waitStateClocks32"),
            )
            .expect("IOCore.wait.waitStateClocks32"),
            iocore_wait_wramconfiguration: JsValueEncoder::decode(
                object.get("IOCore.wait.WRAMConfiguration"),
            )
            .expect("IOCore.wait.WRAMConfiguration"),
            iocore_wait_wramwait_state: JsValueEncoder::decode(
                object.get("IOCore.wait.WRAMWaitState"),
            )
            .expect("IOCore.wait.WRAMWaitState"),
            last_timestamp: JsValueEncoder::decode(object.get("lastTimestamp"))
                .expect("lastTimestamp"),
            metric_start: JsValueEncoder::decode(object.get("metricStart")).expect("metricStart"),
        }
    }

    fn encode(self, factory: &Factory) -> Object {
        let object = factory.object();

        object.set(
            "clockCyclesSinceStart",
            JsValueEncoder::encode(self.clock_cycles_since_start)
                .expect("clock_cycles_since_start"),
        );
        object.set(
            "IOCore.accumulatedClocks",
            JsValueEncoder::encode(self.iocore_accumulated_clocks)
                .expect("iocore_accumulated_clocks"),
        );
        object.set(
            "IOCore.ARM.decode",
            JsValueEncoder::encode(self.iocore_arm_decode).expect("iocore_arm_decode"),
        );
        object.set(
            "IOCore.ARM.execute",
            JsValueEncoder::encode(self.iocore_arm_execute).expect("iocore_arm_execute"),
        );
        object.set(
            "IOCore.ARM.fetch",
            JsValueEncoder::encode(self.iocore_arm_fetch).expect("iocore_arm_fetch"),
        );
        object.set(
            "IOCore.ARM.registers",
            JsValueEncoder::encode(self.iocore_arm_registers).expect("iocore_arm_registers"),
        );
        object.set(
            "IOCore.ARM.registersUSR",
            JsValueEncoder::encode(self.iocore_arm_registers_usr)
                .expect("iocore_arm_registers_usr"),
        );
        object.set(
            "IOCore.cartridge.EEPROMStart",
            JsValueEncoder::encode(self.iocore_cartridge_eepromstart)
                .expect("iocore_cartridge_eepromstart"),
        );
        object.set(
            "IOCore.cartridge.flash_is128",
            JsValueEncoder::encode(self.iocore_cartridge_flash_is128)
                .expect("iocore_cartridge_flash_is128"),
        );
        object.set(
            "IOCore.cartridge.flash_isAtmel",
            JsValueEncoder::encode(self.iocore_cartridge_flash_is_atmel)
                .expect("iocore_cartridge_flash_is_atmel"),
        );
        object.set(
            "IOCore.cartridge.name",
            JsValueEncoder::encode(self.iocore_cartridge_name).expect("iocore_cartridge_name"),
        );
        object.set(
            "IOCore.cpu.modeFlags",
            JsValueEncoder::encode(self.iocore_cpu_mode_flags).expect("iocore_cpu_mode_flags"),
        );
        object.set(
            "IOCore.cpu.mul64ResultHigh",
            JsValueEncoder::encode(self.iocore_cpu_mul64_result_high)
                .expect("iocore_cpu_mul64_result_high"),
        );
        object.set(
            "IOCore.cpu.mul64ResultLow",
            JsValueEncoder::encode(self.iocore_cpu_mul64_result_low)
                .expect("iocore_cpu_mul64_result_low"),
        );
        object.set(
            "IOCore.cpu.registersABT",
            JsValueEncoder::encode(self.iocore_cpu_registers_abt)
                .expect("iocore_cpu_registers_abt"),
        );
        object.set(
            "IOCore.cpu.registersFIQ",
            JsValueEncoder::encode(self.iocore_cpu_registers_fiq)
                .expect("iocore_cpu_registers_fiq"),
        );
        object.set(
            "IOCore.cpu.registersIRQ",
            JsValueEncoder::encode(self.iocore_cpu_registers_irq)
                .expect("iocore_cpu_registers_irq"),
        );
        object.set(
            "IOCore.cpu.registersSVC",
            JsValueEncoder::encode(self.iocore_cpu_registers_svc)
                .expect("iocore_cpu_registers_svc"),
        );
        object.set(
            "IOCore.cpu.registersUND",
            JsValueEncoder::encode(self.iocore_cpu_registers_und)
                .expect("iocore_cpu_registers_und"),
        );
        object.set(
            "IOCore.cpu.SPSR",
            JsValueEncoder::encode(self.iocore_cpu_spsr).expect("iocore_cpu_spsr"),
        );
        object.set(
            "IOCore.cpu.triggeredIRQ",
            JsValueEncoder::encode(self.iocore_cpu_triggered_irq)
                .expect("iocore_cpu_triggered_irq"),
        );
        object.set(
            "IOCore.cyclesOveriteratedPreviously",
            JsValueEncoder::encode(self.iocore_cycles_overiterated_previously)
                .expect("iocore_cycles_overiterated_previously"),
        );
        object.set(
            "IOCore.cyclesToIterate",
            JsValueEncoder::encode(self.iocore_cycles_to_iterate)
                .expect("iocore_cycles_to_iterate"),
        );
        object.set(
            "IOCore.dma.currentMatch",
            JsValueEncoder::encode(self.iocore_dma_current_match)
                .expect("iocore_dma_current_match"),
        );
        object.set(
            "IOCore.dma.fetch",
            JsValueEncoder::encode(self.iocore_dma_fetch).expect("iocore_dma_fetch"),
        );
        object.set(
            "IOCore.dmaChannel0.destination",
            JsValueEncoder::encode(self.iocore_dma_channel0_destination)
                .expect("iocore_dma_channel0_destination"),
        );
        object.set(
            "IOCore.dmaChannel0.destinationControl",
            JsValueEncoder::encode(self.iocore_dma_channel0_destination_control)
                .expect("iocore_dma_channel0_destination_control"),
        );
        object.set(
            "IOCore.dmaChannel0.destinationShadow",
            JsValueEncoder::encode(self.iocore_dma_channel0_destination_shadow)
                .expect("iocore_dma_channel0_destination_shadow"),
        );
        object.set(
            "IOCore.dmaChannel0.dmaType",
            JsValueEncoder::encode(self.iocore_dma_channel0_dma_type)
                .expect("iocore_dma_channel0_dma_type"),
        );
        object.set(
            "IOCore.dmaChannel0.enabled",
            JsValueEncoder::encode(self.iocore_dma_channel0_enabled)
                .expect("iocore_dma_channel0_enabled"),
        );
        object.set(
            "IOCore.dmaChannel0.irqFlagging",
            JsValueEncoder::encode(self.iocore_dma_channel0_irq_flagging)
                .expect("iocore_dma_channel0_irq_flagging"),
        );
        object.set(
            "IOCore.dmaChannel0.is32Bit",
            JsValueEncoder::encode(self.iocore_dma_channel0_is32_bit)
                .expect("iocore_dma_channel0_is32_bit"),
        );
        object.set(
            "IOCore.dmaChannel0.pending",
            JsValueEncoder::encode(self.iocore_dma_channel0_pending)
                .expect("iocore_dma_channel0_pending"),
        );
        object.set(
            "IOCore.dmaChannel0.repeat",
            JsValueEncoder::encode(self.iocore_dma_channel0_repeat)
                .expect("iocore_dma_channel0_repeat"),
        );
        object.set(
            "IOCore.dmaChannel0.source",
            JsValueEncoder::encode(self.iocore_dma_channel0_source)
                .expect("iocore_dma_channel0_source"),
        );
        object.set(
            "IOCore.dmaChannel0.sourceControl",
            JsValueEncoder::encode(self.iocore_dma_channel0_source_control)
                .expect("iocore_dma_channel0_source_control"),
        );
        object.set(
            "IOCore.dmaChannel0.sourceShadow",
            JsValueEncoder::encode(self.iocore_dma_channel0_source_shadow)
                .expect("iocore_dma_channel0_source_shadow"),
        );
        object.set(
            "IOCore.dmaChannel0.wordCount",
            JsValueEncoder::encode(self.iocore_dma_channel0_word_count)
                .expect("iocore_dma_channel0_word_count"),
        );
        object.set(
            "IOCore.dmaChannel0.wordCountShadow",
            JsValueEncoder::encode(self.iocore_dma_channel0_word_count_shadow)
                .expect("iocore_dma_channel0_word_count_shadow"),
        );
        object.set(
            "IOCore.dmaChannel1.destination",
            JsValueEncoder::encode(self.iocore_dma_channel1_destination)
                .expect("iocore_dma_channel1_destination"),
        );
        object.set(
            "IOCore.dmaChannel1.destinationShadow",
            JsValueEncoder::encode(self.iocore_dma_channel1_destination_shadow)
                .expect("iocore_dma_channel1_destination_shadow"),
        );
        object.set(
            "IOCore.dmaChannel1.dmaType",
            JsValueEncoder::encode(self.iocore_dma_channel1_dma_type)
                .expect("iocore_dma_channel1_dma_type"),
        );
        object.set(
            "IOCore.dmaChannel1.enabled",
            JsValueEncoder::encode(self.iocore_dma_channel1_enabled)
                .expect("iocore_dma_channel1_enabled"),
        );
        object.set(
            "IOCore.dmaChannel1.is32Bit",
            JsValueEncoder::encode(self.iocore_dma_channel1_is32_bit)
                .expect("iocore_dma_channel1_is32_bit"),
        );
        object.set(
            "IOCore.dmaChannel1.repeat",
            JsValueEncoder::encode(self.iocore_dma_channel1_repeat)
                .expect("iocore_dma_channel1_repeat"),
        );
        object.set(
            "IOCore.dmaChannel1.source",
            JsValueEncoder::encode(self.iocore_dma_channel1_source)
                .expect("iocore_dma_channel1_source"),
        );
        object.set(
            "IOCore.dmaChannel1.sourceShadow",
            JsValueEncoder::encode(self.iocore_dma_channel1_source_shadow)
                .expect("iocore_dma_channel1_source_shadow"),
        );
        object.set(
            "IOCore.dmaChannel1.wordCount",
            JsValueEncoder::encode(self.iocore_dma_channel1_word_count)
                .expect("iocore_dma_channel1_word_count"),
        );
        object.set(
            "IOCore.dmaChannel1.wordCountShadow",
            JsValueEncoder::encode(self.iocore_dma_channel1_word_count_shadow)
                .expect("iocore_dma_channel1_word_count_shadow"),
        );
        object.set(
            "IOCore.dmaChannel2.destination",
            JsValueEncoder::encode(self.iocore_dma_channel2_destination)
                .expect("iocore_dma_channel2_destination"),
        );
        object.set(
            "IOCore.dmaChannel2.destinationShadow",
            JsValueEncoder::encode(self.iocore_dma_channel2_destination_shadow)
                .expect("iocore_dma_channel2_destination_shadow"),
        );
        object.set(
            "IOCore.dmaChannel2.enabled",
            JsValueEncoder::encode(self.iocore_dma_channel2_enabled)
                .expect("iocore_dma_channel2_enabled"),
        );
        object.set(
            "IOCore.dmaChannel2.source",
            JsValueEncoder::encode(self.iocore_dma_channel2_source)
                .expect("iocore_dma_channel2_source"),
        );
        object.set(
            "IOCore.dmaChannel2.sourceShadow",
            JsValueEncoder::encode(self.iocore_dma_channel2_source_shadow)
                .expect("iocore_dma_channel2_source_shadow"),
        );
        object.set(
            "IOCore.dmaChannel3.destination",
            JsValueEncoder::encode(self.iocore_dma_channel3_destination)
                .expect("iocore_dma_channel3_destination"),
        );
        object.set(
            "IOCore.dmaChannel3.destinationShadow",
            JsValueEncoder::encode(self.iocore_dma_channel3_destination_shadow)
                .expect("iocore_dma_channel3_destination_shadow"),
        );
        object.set(
            "IOCore.dmaChannel3.displaySyncEnableDelay",
            JsValueEncoder::encode(self.iocore_dma_channel3_display_sync_enable_delay)
                .expect("iocore_dma_channel3_display_sync_enable_delay"),
        );
        object.set(
            "IOCore.dmaChannel3.gamePakDMA",
            JsValueEncoder::encode(self.iocore_dma_channel3_game_pak_dma)
                .expect("iocore_dma_channel3_game_pak_dma"),
        );
        object.set(
            "IOCore.dmaChannel3.source",
            JsValueEncoder::encode(self.iocore_dma_channel3_source)
                .expect("iocore_dma_channel3_source"),
        );
        object.set(
            "IOCore.dmaChannel3.sourceControl",
            JsValueEncoder::encode(self.iocore_dma_channel3_source_control)
                .expect("iocore_dma_channel3_source_control"),
        );
        object.set(
            "IOCore.dmaChannel3.sourceShadow",
            JsValueEncoder::encode(self.iocore_dma_channel3_source_shadow)
                .expect("iocore_dma_channel3_source_shadow"),
        );
        object.set(
            "IOCore.dmaChannel3.wordCount",
            JsValueEncoder::encode(self.iocore_dma_channel3_word_count)
                .expect("iocore_dma_channel3_word_count"),
        );
        object.set(
            "IOCore.gfxRenderer.IOData16",
            JsValueEncoder::encode(self.iocore_gfx_renderer_iodata16)
                .expect("iocore_gfx_renderer_iodata16"),
        );
        object.set(
            "IOCore.gfxRenderer.IOData32",
            JsValueEncoder::encode(self.iocore_gfx_renderer_iodata32)
                .expect("iocore_gfx_renderer_iodata32"),
        );
        object.set(
            "IOCore.gfxRenderer.IOData8",
            JsValueEncoder::encode(self.iocore_gfx_renderer_iodata8)
                .expect("iocore_gfx_renderer_iodata8"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.backdrop",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_backdrop)
                .expect("iocore_gfx_renderer_renderer_backdrop"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.BGCharacterBaseBlock",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg0_renderer_bgcharacter_base_block,
            )
            .expect("iocore_gfx_renderer_renderer_bg0_renderer_bgcharacter_base_block"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.BGLayer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_bglayer)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_bglayer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.BGScreenBaseBlock",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg0_renderer_bgscreen_base_block,
            )
            .expect("iocore_gfx_renderer_renderer_bg0_renderer_bgscreen_base_block"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.BGXCoord",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_bgxcoord)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_bgxcoord"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.BGYCoord",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_bgycoord)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_bgycoord"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.do256",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_do256)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_do256"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.doMosaic",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_do_mosaic)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_do_mosaic"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.offset",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_offset)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_offset"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.priorityFlag",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_priority_flag)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_priority_flag"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.scratchBuffer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_scratch_buffer)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_scratch_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.tileFetched",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_tile_fetched)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_tile_fetched"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg0Renderer.tileMode",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg0_renderer_tile_mode)
                .expect("iocore_gfx_renderer_renderer_bg0_renderer_tile_mode"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg1Renderer.BGLayer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg1_renderer_bglayer)
                .expect("iocore_gfx_renderer_renderer_bg1_renderer_bglayer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg1Renderer.BGScreenBaseBlock",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg1_renderer_bgscreen_base_block,
            )
            .expect("iocore_gfx_renderer_renderer_bg1_renderer_bgscreen_base_block"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg1Renderer.BGXCoord",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg1_renderer_bgxcoord)
                .expect("iocore_gfx_renderer_renderer_bg1_renderer_bgxcoord"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg1Renderer.BGYCoord",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg1_renderer_bgycoord)
                .expect("iocore_gfx_renderer_renderer_bg1_renderer_bgycoord"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg1Renderer.offset",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg1_renderer_offset)
                .expect("iocore_gfx_renderer_renderer_bg1_renderer_offset"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg1Renderer.priorityFlag",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg1_renderer_priority_flag)
                .expect("iocore_gfx_renderer_renderer_bg1_renderer_priority_flag"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg1Renderer.scratchBuffer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg1_renderer_scratch_buffer)
                .expect("iocore_gfx_renderer_renderer_bg1_renderer_scratch_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg1Renderer.tileFetched",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg1_renderer_tile_fetched)
                .expect("iocore_gfx_renderer_renderer_bg1_renderer_tile_fetched"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2FrameBufferRenderer.frameSelect",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_frame_buffer_renderer_frame_select,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_frame_buffer_renderer_frame_select"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGCharacterBaseBlock",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgcharacter_base_block,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgcharacter_base_block"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGDisplayOverflow",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgdisplay_overflow,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgdisplay_overflow"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGScreenBaseBlock",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgscreen_base_block,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_matrix_renderer_bgscreen_base_block"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.mapSize",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg2_matrix_renderer_map_size)
                .expect("iocore_gfx_renderer_renderer_bg2_matrix_renderer_map_size"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.mapSizeComparer",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_matrix_renderer_map_size_comparer,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_matrix_renderer_map_size_comparer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.palette",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg2_matrix_renderer_palette)
                .expect("iocore_gfx_renderer_renderer_bg2_matrix_renderer_palette"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2TextRenderer.BGCharacterBaseBlock",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_text_renderer_bgcharacter_base_block,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_text_renderer_bgcharacter_base_block"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2TextRenderer.BGLayer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg2_text_renderer_bglayer)
                .expect("iocore_gfx_renderer_renderer_bg2_text_renderer_bglayer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2TextRenderer.BGScreenBaseBlock",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_text_renderer_bgscreen_base_block,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_text_renderer_bgscreen_base_block"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2TextRenderer.BGYCoord",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg2_text_renderer_bgycoord)
                .expect("iocore_gfx_renderer_renderer_bg2_text_renderer_bgycoord"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2TextRenderer.offset",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg2_text_renderer_offset)
                .expect("iocore_gfx_renderer_renderer_bg2_text_renderer_offset"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2TextRenderer.priorityFlag",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_text_renderer_priority_flag,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_text_renderer_priority_flag"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2TextRenderer.scratchBuffer",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_text_renderer_scratch_buffer,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_text_renderer_scratch_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2TextRenderer.tileFetched",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg2_text_renderer_tile_fetched,
            )
            .expect("iocore_gfx_renderer_renderer_bg2_text_renderer_tile_fetched"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg2TextRenderer.tileMode",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg2_text_renderer_tile_mode)
                .expect("iocore_gfx_renderer_renderer_bg2_text_renderer_tile_mode"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg3MatrixRenderer.BGScreenBaseBlock",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg3_matrix_renderer_bgscreen_base_block,
            )
            .expect("iocore_gfx_renderer_renderer_bg3_matrix_renderer_bgscreen_base_block"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg3MatrixRenderer.mapSize",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg3_matrix_renderer_map_size)
                .expect("iocore_gfx_renderer_renderer_bg3_matrix_renderer_map_size"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg3MatrixRenderer.mapSizeComparer",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg3_matrix_renderer_map_size_comparer,
            )
            .expect("iocore_gfx_renderer_renderer_bg3_matrix_renderer_map_size_comparer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg3TextRenderer.BGLayer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg3_text_renderer_bglayer)
                .expect("iocore_gfx_renderer_renderer_bg3_text_renderer_bglayer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg3TextRenderer.BGScreenBaseBlock",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg3_text_renderer_bgscreen_base_block,
            )
            .expect("iocore_gfx_renderer_renderer_bg3_text_renderer_bgscreen_base_block"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg3TextRenderer.offset",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg3_text_renderer_offset)
                .expect("iocore_gfx_renderer_renderer_bg3_text_renderer_offset"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg3TextRenderer.priorityFlag",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg3_text_renderer_priority_flag,
            )
            .expect("iocore_gfx_renderer_renderer_bg3_text_renderer_priority_flag"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg3TextRenderer.scratchBuffer",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg3_text_renderer_scratch_buffer,
            )
            .expect("iocore_gfx_renderer_renderer_bg3_text_renderer_scratch_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bg3TextRenderer.tileFetched",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg3_text_renderer_tile_fetched,
            )
            .expect("iocore_gfx_renderer_renderer_bg3_text_renderer_tile_fetched"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdmx",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdmx)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdmx"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdmy",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdmy)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdmy"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdx",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdx)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdx"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdy",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdy)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer0_bgdy"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGReferenceX",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg_affine_renderer0_bgreference_x,
            )
            .expect("iocore_gfx_renderer_renderer_bg_affine_renderer0_bgreference_x"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGReferenceY",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg_affine_renderer0_bgreference_y,
            )
            .expect("iocore_gfx_renderer_renderer_bg_affine_renderer0_bgreference_y"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer0.pb",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer0_pb)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer0_pb"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer0.pd",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer0_pd)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer0_pd"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer0.scratchBuffer",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg_affine_renderer0_scratch_buffer,
            )
            .expect("iocore_gfx_renderer_renderer_bg_affine_renderer0_scratch_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGdmy",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer1_bgdmy)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer1_bgdmy"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGdx",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer1_bgdx)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer1_bgdx"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGReferenceX",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg_affine_renderer1_bgreference_x,
            )
            .expect("iocore_gfx_renderer_renderer_bg_affine_renderer1_bgreference_x"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGReferenceY",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg_affine_renderer1_bgreference_y,
            )
            .expect("iocore_gfx_renderer_renderer_bg_affine_renderer1_bgreference_y"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer1.pb",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer1_pb)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer1_pb"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer1.pd",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_bg_affine_renderer1_pd)
                .expect("iocore_gfx_renderer_renderer_bg_affine_renderer1_pd"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.bgAffineRenderer1.scratchBuffer",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_bg_affine_renderer1_scratch_buffer,
            )
            .expect("iocore_gfx_renderer_renderer_bg_affine_renderer1_scratch_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.buffer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_buffer)
                .expect("iocore_gfx_renderer_renderer_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.colorEffectsRenderer.alphaBlendAmountTarget1",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_color_effects_renderer_alpha_blend_amount_target1,
            )
            .expect(
                "iocore_gfx_renderer_renderer_color_effects_renderer_alpha_blend_amount_target1",
            ),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.colorEffectsRenderer.alphaBlendAmountTarget2",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_color_effects_renderer_alpha_blend_amount_target2,
            )
            .expect(
                "iocore_gfx_renderer_renderer_color_effects_renderer_alpha_blend_amount_target2",
            ),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.colorEffectsRenderer.brightnessEffectAmount",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_color_effects_renderer_brightness_effect_amount,
            )
            .expect("iocore_gfx_renderer_renderer_color_effects_renderer_brightness_effect_amount"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.colorEffectsRenderer.colorEffectsType",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_color_effects_renderer_color_effects_type,
            )
            .expect("iocore_gfx_renderer_renderer_color_effects_renderer_color_effects_type"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.colorEffectsRenderer.effectsTarget1",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_color_effects_renderer_effects_target1,
            )
            .expect("iocore_gfx_renderer_renderer_color_effects_renderer_effects_target1"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.colorEffectsRenderer.effectsTarget2",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_color_effects_renderer_effects_target2,
            )
            .expect("iocore_gfx_renderer_renderer_color_effects_renderer_effects_target2"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.compositor.doEffects",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_compositor_do_effects)
                .expect("iocore_gfx_renderer_renderer_compositor_do_effects"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.display",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_display)
                .expect("iocore_gfx_renderer_renderer_display"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.displayControl",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_display_control)
                .expect("iocore_gfx_renderer_renderer_display_control"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.frameBuffer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_frame_buffer)
                .expect("iocore_gfx_renderer_renderer_frame_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.greenSwap",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_green_swap)
                .expect("iocore_gfx_renderer_renderer_green_swap"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.lastUnrenderedLine",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_last_unrendered_line)
                .expect("iocore_gfx_renderer_renderer_last_unrendered_line"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.lineBuffer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_line_buffer)
                .expect("iocore_gfx_renderer_renderer_line_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.mosaicRenderer.BGMosaicHSize",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_mosaic_renderer_bgmosaic_hsize,
            )
            .expect("iocore_gfx_renderer_renderer_mosaic_renderer_bgmosaic_hsize"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.mosaicRenderer.BGMosaicVSize",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_mosaic_renderer_bgmosaic_vsize,
            )
            .expect("iocore_gfx_renderer_renderer_mosaic_renderer_bgmosaic_vsize"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.mosaicRenderer.OBJMosaicHSize",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_mosaic_renderer_objmosaic_hsize,
            )
            .expect("iocore_gfx_renderer_renderer_mosaic_renderer_objmosaic_hsize"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.mosaicRenderer.OBJMosaicVSize",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_mosaic_renderer_objmosaic_vsize,
            )
            .expect("iocore_gfx_renderer_renderer_mosaic_renderer_objmosaic_vsize"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objRenderer.cyclesToRender",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_obj_renderer_cycles_to_render)
                .expect("iocore_gfx_renderer_renderer_obj_renderer_cycles_to_render"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objRenderer.OAMRAM16",
            JsValueEncoder::encode(vu8_to_vu16(
                &self.iocore_gfx_renderer_renderer_obj_renderer_oamram,
            ))
            .expect("iocore_gfx_renderer_renderer_obj_renderer_oamram16"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objRenderer.OAMRAM32",
            JsValueEncoder::encode(vu8_to_vi32(
                &self.iocore_gfx_renderer_renderer_obj_renderer_oamram,
            ))
            .expect("iocore_gfx_renderer_renderer_obj_renderer_oamram32"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objRenderer.OAMRAM",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_obj_renderer_oamram)
                .expect("iocore_gfx_renderer_renderer_obj_renderer_oamram"),
        );
        object.set_array(
            "IOCore.gfxRenderer.renderer.objRenderer.OAMTable",
            OamTableEntry::encode_array(
                self.iocore_gfx_renderer_renderer_obj_renderer_oamtable,
                factory,
            ),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objRenderer.OBJMatrixParameters",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_obj_renderer_objmatrix_parameters,
            )
            .expect("iocore_gfx_renderer_renderer_obj_renderer_objmatrix_parameters"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objRenderer.scratchBuffer",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_obj_renderer_scratch_buffer)
                .expect("iocore_gfx_renderer_renderer_obj_renderer_scratch_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objRenderer.scratchOBJBuffer",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_obj_renderer_scratch_objbuffer,
            )
            .expect("iocore_gfx_renderer_renderer_obj_renderer_scratch_objbuffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objRenderer.scratchWindowBuffer",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_obj_renderer_scratch_window_buffer,
            )
            .expect("iocore_gfx_renderer_renderer_obj_renderer_scratch_window_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objWindowRenderer.compositor.OBJWindowBuffer",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_obj_window_renderer_compositor_objwindow_buffer,
            )
            .expect("iocore_gfx_renderer_renderer_obj_window_renderer_compositor_objwindow_buffer"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.objWindowRenderer.WINOBJOutside",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_obj_window_renderer_winobjoutside,
            )
            .expect("iocore_gfx_renderer_renderer_obj_window_renderer_winobjoutside"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.palette16",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_palette16)
                .expect("iocore_gfx_renderer_renderer_palette16"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.palette256",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_palette256)
                .expect("iocore_gfx_renderer_renderer_palette256"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.paletteOBJ16",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_palette_obj16)
                .expect("iocore_gfx_renderer_renderer_palette_obj16"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.paletteOBJ256",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_palette_obj256)
                .expect("iocore_gfx_renderer_renderer_palette_obj256"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.paletteRAM16",
            JsValueEncoder::encode(vu8_to_vu16(&self.iocore_gfx_renderer_renderer_palette_ram))
                .expect("iocore_gfx_renderer_renderer_palette_ram16"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.paletteRAM32",
            JsValueEncoder::encode(vu8_to_vi32(&self.iocore_gfx_renderer_renderer_palette_ram))
                .expect("iocore_gfx_renderer_renderer_palette_ram32"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.paletteRAM",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_palette_ram)
                .expect("iocore_gfx_renderer_renderer_palette_ram"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.queuedScanLines",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_queued_scan_lines)
                .expect("iocore_gfx_renderer_renderer_queued_scan_lines"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.swizzledFrame",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_swizzled_frame)
                .expect("iocore_gfx_renderer_renderer_swizzled_frame"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.totalLinesPassed",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_total_lines_passed)
                .expect("iocore_gfx_renderer_renderer_total_lines_passed"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.VRAM16",
            JsValueEncoder::encode(vu8_to_vu16(&self.iocore_gfx_renderer_renderer_vram))
                .expect("iocore_gfx_renderer_renderer_vram16"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.VRAM32",
            JsValueEncoder::encode(vu8_to_vi32(&self.iocore_gfx_renderer_renderer_vram))
                .expect("iocore_gfx_renderer_renderer_vram32"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.VRAM",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_vram)
                .expect("iocore_gfx_renderer_renderer_vram"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.window0Renderer.compositor.doEffects",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_window0_renderer_compositor_do_effects,
            )
            .expect("iocore_gfx_renderer_renderer_window0_renderer_compositor_do_effects"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.window0Renderer.windowDisplayControl",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_window0_renderer_window_display_control,
            )
            .expect("iocore_gfx_renderer_renderer_window0_renderer_window_display_control"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.window0Renderer.WINXCoordLeft",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_window0_renderer_winxcoord_left,
            )
            .expect("iocore_gfx_renderer_renderer_window0_renderer_winxcoord_left"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.window0Renderer.WINXCoordRight",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_window0_renderer_winxcoord_right,
            )
            .expect("iocore_gfx_renderer_renderer_window0_renderer_winxcoord_right"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.window0Renderer.WINYCoordBottom",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_window0_renderer_winycoord_bottom,
            )
            .expect("iocore_gfx_renderer_renderer_window0_renderer_winycoord_bottom"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.window0Renderer.WINYCoordTop",
            JsValueEncoder::encode(
                self.iocore_gfx_renderer_renderer_window0_renderer_winycoord_top,
            )
            .expect("iocore_gfx_renderer_renderer_window0_renderer_winycoord_top"),
        );
        object.set(
            "IOCore.gfxRenderer.renderer.WINOutside",
            JsValueEncoder::encode(self.iocore_gfx_renderer_renderer_winoutside)
                .expect("iocore_gfx_renderer_renderer_winoutside"),
        );
        object.set(
            "IOCore.gfxState.currentScanLine",
            JsValueEncoder::encode(self.iocore_gfx_state_current_scan_line)
                .expect("iocore_gfx_state_current_scan_line"),
        );
        object.set(
            "IOCore.gfxState.IRQFlags",
            JsValueEncoder::encode(self.iocore_gfx_state_irqflags)
                .expect("iocore_gfx_state_irqflags"),
        );
        object.set(
            "IOCore.gfxState.LCDTicks",
            JsValueEncoder::encode(self.iocore_gfx_state_lcdticks)
                .expect("iocore_gfx_state_lcdticks"),
        );
        object.set(
            "IOCore.gfxState.renderedScanLine",
            JsValueEncoder::encode(self.iocore_gfx_state_rendered_scan_line)
                .expect("iocore_gfx_state_rendered_scan_line"),
        );
        object.set(
            "IOCore.gfxState.statusFlags",
            JsValueEncoder::encode(self.iocore_gfx_state_status_flags)
                .expect("iocore_gfx_state_status_flags"),
        );
        object.set(
            "IOCore.gfxState.VCounter",
            JsValueEncoder::encode(self.iocore_gfx_state_vcounter)
                .expect("iocore_gfx_state_vcounter"),
        );
        object.set(
            "IOCore.irq.interruptsEnabled",
            JsValueEncoder::encode(self.iocore_irq_interrupts_enabled)
                .expect("iocore_irq_interrupts_enabled"),
        );
        object.set(
            "IOCore.irq.interruptsRequested",
            JsValueEncoder::encode(self.iocore_irq_interrupts_requested)
                .expect("iocore_irq_interrupts_requested"),
        );
        object.set(
            "IOCore.joypad.keyInput",
            JsValueEncoder::encode(self.iocore_joypad_key_input).expect("iocore_joypad_key_input"),
        );
        object.set(
            "IOCore.joypad.keyInterrupt",
            JsValueEncoder::encode(self.iocore_joypad_key_interrupt)
                .expect("iocore_joypad_key_interrupt"),
        );
        object.set(
            "IOCore.memory.externalRAM16",
            JsValueEncoder::encode(vu8_to_vu16(&self.iocore_memory_external_ram))
                .expect("iocore_memory_external_ram16"),
        );
        object.set(
            "IOCore.memory.externalRAM32",
            JsValueEncoder::encode(vu8_to_vi32(&self.iocore_memory_external_ram))
                .expect("iocore_memory_external_ram32"),
        );
        object.set(
            "IOCore.memory.externalRAM",
            JsValueEncoder::encode(self.iocore_memory_external_ram)
                .expect("iocore_memory_external_ram"),
        );
        object.set(
            "IOCore.memory.internalRAM16",
            JsValueEncoder::encode(vu8_to_vu16(&self.iocore_memory_internal_ram))
                .expect("iocore_memory_internal_ram16"),
        );
        object.set(
            "IOCore.memory.internalRAM32",
            JsValueEncoder::encode(vu8_to_vi32(&self.iocore_memory_internal_ram))
                .expect("iocore_memory_internal_ram32"),
        );
        object.set(
            "IOCore.memory.internalRAM",
            JsValueEncoder::encode(self.iocore_memory_internal_ram)
                .expect("iocore_memory_internal_ram"),
        );
        object.set(
            "IOCore.memory.irq.IME",
            JsValueEncoder::encode(self.iocore_memory_irq_ime).expect("iocore_memory_irq_ime"),
        );
        object.set(
            "IOCore.memory.lastBIOSREAD",
            JsValueEncoder::encode(self.iocore_memory_last_biosread)
                .expect("iocore_memory_last_biosread"),
        );
        object.set(
            "IOCore.memory.WRAMControlFlags",
            JsValueEncoder::encode(self.iocore_memory_wramcontrol_flags)
                .expect("iocore_memory_wramcontrol_flags"),
        );
        object.set(
            "IOCore.nextEventClocks",
            JsValueEncoder::encode(self.iocore_next_event_clocks)
                .expect("iocore_next_event_clocks"),
        );
        object.set(
            "IOCore.saves.EEPROMChip.address",
            JsValueEncoder::encode(self.iocore_saves_eepromchip_address)
                .expect("iocore_saves_eepromchip_address"),
        );
        object.set(
            "IOCore.saves.EEPROMChip.bitsProcessed",
            JsValueEncoder::encode(self.iocore_saves_eepromchip_bits_processed)
                .expect("iocore_saves_eepromchip_bits_processed"),
        );
        object.set(
            "IOCore.saves.EEPROMChip.buffer",
            JsValueEncoder::encode(self.iocore_saves_eepromchip_buffer)
                .expect("iocore_saves_eepromchip_buffer"),
        );
        object.set(
            "IOCore.saves.EEPROMChip.largestSizePossible",
            JsValueEncoder::encode(self.iocore_saves_eepromchip_largest_size_possible)
                .expect("iocore_saves_eepromchip_largest_size_possible"),
        );
        object.set(
            "IOCore.saves.EEPROMChip.mode",
            JsValueEncoder::encode(self.iocore_saves_eepromchip_mode)
                .expect("iocore_saves_eepromchip_mode"),
        );
        // object.set(
        //     "IOCore.saves.FLASHChip.BANKOffset",
        //     JsValueEncoder::encode(self.iocore_saves_flashchip_bankoffset)
        //         .expect("iocore_saves_flashchip_bankoffset"),
        // );
        // object.set(
        //     "IOCore.saves.FLASHChip.flashCommand",
        //     JsValueEncoder::encode(self.iocore_saves_flashchip_flash_command)
        //         .expect("iocore_saves_flashchip_flash_command"),
        // );
        // object.set(
        //     "IOCore.saves.FLASHChip.flashCommandUnlockStage",
        //     JsValueEncoder::encode(self.iocore_saves_flashchip_flash_command_unlock_stage)
        //         .expect("iocore_saves_flashchip_flash_command_unlock_stage"),
        // );
        // object.set(
        //     "IOCore.saves.FLASHChip.largestSizePossible",
        //     JsValueEncoder::encode(self.iocore_saves_flashchip_largest_size_possible)
        //         .expect("iocore_saves_flashchip_largest_size_possible"),
        // );
        // object.set(
        //     "IOCore.saves.FLASHChip.notATMEL",
        //     JsValueEncoder::encode(self.iocore_saves_flashchip_not_atmel)
        //         .expect("iocore_saves_flashchip_not_atmel"),
        // );
        // object.set(
        //     "IOCore.saves.FLASHChip.saves",
        //     JsValueEncoder::encode(self.iocore_saves_flashchip_saves)
        //         .expect("iocore_saves_flashchip_saves"),
        // );
        // object.set(
        //     "IOCore.saves.FLASHChip.writeBytesLeft",
        //     JsValueEncoder::encode(self.iocore_saves_flashchip_write_bytes_left)
        //         .expect("iocore_saves_flashchip_write_bytes_left"),
        // );
        object.set(
            "IOCore.saves.GPIOChip.data",
            JsValueEncoder::encode(self.iocore_saves_gpiochip_data)
                .expect("iocore_saves_gpiochip_data"),
        );
        object.set(
            "IOCore.saves.GPIOChip.direction",
            JsValueEncoder::encode(self.iocore_saves_gpiochip_direction)
                .expect("iocore_saves_gpiochip_direction"),
        );
        object.set(
            "IOCore.saves.GPIOChip.readWrite",
            JsValueEncoder::encode(self.iocore_saves_gpiochip_read_write)
                .expect("iocore_saves_gpiochip_read_write"),
        );
        object.set(
            "IOCore.saves.GPIOChip.type",
            JsValueEncoder::encode(self.iocore_saves_gpiochip_type)
                .expect("iocore_saves_gpiochip_type"),
        );
        object.set(
            "IOCore.saves.saveType",
            JsValueEncoder::encode(self.iocore_saves_save_type).expect("iocore_saves_save_type"),
        );
        object.set(
            "IOCore.saves.UNDETERMINED.possible",
            JsValueEncoder::encode(self.iocore_saves_undetermined_possible)
                .expect("iocore_saves_undetermined_possible"),
        );
        object.set(
            "IOCore.serial.JOYBUS_CNTL_FLAGS",
            JsValueEncoder::encode(self.iocore_serial_joybus_cntl_flags)
                .expect("iocore_serial_joybus_cntl_flags"),
        );
        object.set(
            "IOCore.serial.JOYBUS_IRQ",
            JsValueEncoder::encode(self.iocore_serial_joybus_irq)
                .expect("iocore_serial_joybus_irq"),
        );
        object.set(
            "IOCore.serial.JOYBUS_RECV0",
            JsValueEncoder::encode(self.iocore_serial_joybus_recv0)
                .expect("iocore_serial_joybus_recv0"),
        );
        object.set(
            "IOCore.serial.JOYBUS_RECV1",
            JsValueEncoder::encode(self.iocore_serial_joybus_recv1)
                .expect("iocore_serial_joybus_recv1"),
        );
        object.set(
            "IOCore.serial.JOYBUS_RECV2",
            JsValueEncoder::encode(self.iocore_serial_joybus_recv2)
                .expect("iocore_serial_joybus_recv2"),
        );
        object.set(
            "IOCore.serial.JOYBUS_RECV3",
            JsValueEncoder::encode(self.iocore_serial_joybus_recv3)
                .expect("iocore_serial_joybus_recv3"),
        );
        object.set(
            "IOCore.serial.JOYBUS_SEND0",
            JsValueEncoder::encode(self.iocore_serial_joybus_send0)
                .expect("iocore_serial_joybus_send0"),
        );
        object.set(
            "IOCore.serial.JOYBUS_SEND1",
            JsValueEncoder::encode(self.iocore_serial_joybus_send1)
                .expect("iocore_serial_joybus_send1"),
        );
        object.set(
            "IOCore.serial.JOYBUS_SEND2",
            JsValueEncoder::encode(self.iocore_serial_joybus_send2)
                .expect("iocore_serial_joybus_send2"),
        );
        object.set(
            "IOCore.serial.JOYBUS_SEND3",
            JsValueEncoder::encode(self.iocore_serial_joybus_send3)
                .expect("iocore_serial_joybus_send3"),
        );
        object.set(
            "IOCore.serial.JOYBUS_STAT",
            JsValueEncoder::encode(self.iocore_serial_joybus_stat)
                .expect("iocore_serial_joybus_stat"),
        );
        object.set(
            "IOCore.serial.RCNTDataBitFlow",
            JsValueEncoder::encode(self.iocore_serial_rcntdata_bit_flow)
                .expect("iocore_serial_rcntdata_bit_flow"),
        );
        object.set(
            "IOCore.serial.RCNTDataBits",
            JsValueEncoder::encode(self.iocore_serial_rcntdata_bits)
                .expect("iocore_serial_rcntdata_bits"),
        );
        object.set(
            "IOCore.serial.RCNTIRQ",
            JsValueEncoder::encode(self.iocore_serial_rcntirq).expect("iocore_serial_rcntirq"),
        );
        object.set(
            "IOCore.serial.RCNTMode",
            JsValueEncoder::encode(self.iocore_serial_rcntmode).expect("iocore_serial_rcntmode"),
        );
        object.set(
            "IOCore.serial.serialBitsShifted",
            JsValueEncoder::encode(self.iocore_serial_serial_bits_shifted)
                .expect("iocore_serial_serial_bits_shifted"),
        );
        object.set(
            "IOCore.serial.shiftClocks",
            JsValueEncoder::encode(self.iocore_serial_shift_clocks)
                .expect("iocore_serial_shift_clocks"),
        );
        object.set(
            "IOCore.serial.SIOBaudRate",
            JsValueEncoder::encode(self.iocore_serial_siobaud_rate)
                .expect("iocore_serial_siobaud_rate"),
        );
        object.set(
            "IOCore.serial.SIOCNT_IRQ",
            JsValueEncoder::encode(self.iocore_serial_siocnt_irq)
                .expect("iocore_serial_siocnt_irq"),
        );
        object.set(
            "IOCore.serial.SIOCNT_MODE",
            JsValueEncoder::encode(self.iocore_serial_siocnt_mode)
                .expect("iocore_serial_siocnt_mode"),
        );
        object.set(
            "IOCore.serial.SIOCNT_UART_CTS",
            JsValueEncoder::encode(self.iocore_serial_siocnt_uart_cts)
                .expect("iocore_serial_siocnt_uart_cts"),
        );
        object.set(
            "IOCore.serial.SIOCNT_UART_FIFO",
            JsValueEncoder::encode(self.iocore_serial_siocnt_uart_fifo)
                .expect("iocore_serial_siocnt_uart_fifo"),
        );
        object.set(
            "IOCore.serial.SIOCNT_UART_FIFO_ENABLE",
            JsValueEncoder::encode(self.iocore_serial_siocnt_uart_fifo_enable)
                .expect("iocore_serial_siocnt_uart_fifo_enable"),
        );
        object.set(
            "IOCore.serial.SIOCNT_UART_MISC",
            JsValueEncoder::encode(self.iocore_serial_siocnt_uart_misc)
                .expect("iocore_serial_siocnt_uart_misc"),
        );
        object.set(
            "IOCore.serial.SIOCNT_UART_PARITY_ENABLE",
            JsValueEncoder::encode(self.iocore_serial_siocnt_uart_parity_enable)
                .expect("iocore_serial_siocnt_uart_parity_enable"),
        );
        object.set(
            "IOCore.serial.SIOCNT_UART_RECV_ENABLE",
            JsValueEncoder::encode(self.iocore_serial_siocnt_uart_recv_enable)
                .expect("iocore_serial_siocnt_uart_recv_enable"),
        );
        object.set(
            "IOCore.serial.SIOCNT_UART_SEND_ENABLE",
            JsValueEncoder::encode(self.iocore_serial_siocnt_uart_send_enable)
                .expect("iocore_serial_siocnt_uart_send_enable"),
        );
        object.set(
            "IOCore.serial.SIOCNT0_DATA",
            JsValueEncoder::encode(self.iocore_serial_siocnt0_data)
                .expect("iocore_serial_siocnt0_data"),
        );
        object.set(
            "IOCore.serial.SIOCOMMERROR",
            JsValueEncoder::encode(self.iocore_serial_siocommerror)
                .expect("iocore_serial_siocommerror"),
        );
        object.set(
            "IOCore.serial.SIODATA_A",
            JsValueEncoder::encode(self.iocore_serial_siodata_a).expect("iocore_serial_siodata_a"),
        );
        object.set(
            "IOCore.serial.SIODATA_B",
            JsValueEncoder::encode(self.iocore_serial_siodata_b).expect("iocore_serial_siodata_b"),
        );
        object.set(
            "IOCore.serial.SIODATA_C",
            JsValueEncoder::encode(self.iocore_serial_siodata_c).expect("iocore_serial_siodata_c"),
        );
        object.set(
            "IOCore.serial.SIODATA_D",
            JsValueEncoder::encode(self.iocore_serial_siodata_d).expect("iocore_serial_siodata_d"),
        );
        object.set(
            "IOCore.serial.SIODATA8",
            JsValueEncoder::encode(self.iocore_serial_siodata8).expect("iocore_serial_siodata8"),
        );
        object.set(
            "IOCore.serial.SIOMULT_PLAYER_NUMBER",
            JsValueEncoder::encode(self.iocore_serial_siomult_player_number)
                .expect("iocore_serial_siomult_player_number"),
        );
        object.set(
            "IOCore.serial.SIOShiftClockDivider",
            JsValueEncoder::encode(self.iocore_serial_sioshift_clock_divider)
                .expect("iocore_serial_sioshift_clock_divider"),
        );
        object.set(
            "IOCore.serial.SIOShiftClockExternal",
            JsValueEncoder::encode(self.iocore_serial_sioshift_clock_external)
                .expect("iocore_serial_sioshift_clock_external"),
        );
        object.set(
            "IOCore.serial.SIOTransferStarted",
            JsValueEncoder::encode(self.iocore_serial_siotransfer_started)
                .expect("iocore_serial_siotransfer_started"),
        );
        object.set(
            "IOCore.serialClocks",
            JsValueEncoder::encode(self.iocore_serial_clocks).expect("iocore_serial_clocks"),
        );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundA",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_a)
        //         .expect("iocore_sound_agbdirect_sound_a"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundAFolded",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_afolded)
        //         .expect("iocore_sound_agbdirect_sound_afolded"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundALeftCanPlay",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_aleft_can_play)
        //         .expect("iocore_sound_agbdirect_sound_aleft_can_play"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundARightCanPlay",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_aright_can_play)
        //         .expect("iocore_sound_agbdirect_sound_aright_can_play"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundAShifter",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_ashifter)
        //         .expect("iocore_sound_agbdirect_sound_ashifter"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundATimer",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_atimer)
        //         .expect("iocore_sound_agbdirect_sound_atimer"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundB",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_b)
        //         .expect("iocore_sound_agbdirect_sound_b"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundBFolded",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_bfolded)
        //         .expect("iocore_sound_agbdirect_sound_bfolded"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundBLeftCanPlay",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_bleft_can_play)
        //         .expect("iocore_sound_agbdirect_sound_bleft_can_play"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundBRightCanPlay",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_bright_can_play)
        //         .expect("iocore_sound_agbdirect_sound_bright_can_play"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundBShifter",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_bshifter)
        //         .expect("iocore_sound_agbdirect_sound_bshifter"),
        // );
        // object.set(
        //     "IOCore.sound.AGBDirectSoundBTimer",
        //     JsValueEncoder::encode(self.iocore_sound_agbdirect_sound_btimer)
        //         .expect("iocore_sound_agbdirect_sound_btimer"),
        // );
        // object.set(
        //     "IOCore.sound.audioClocksUntilNextEvent",
        //     JsValueEncoder::encode(self.iocore_sound_audio_clocks_until_next_event)
        //         .expect("iocore_sound_audio_clocks_until_next_event"),
        // );
        // object.set(
        //     "IOCore.sound.audioClocksUntilNextEventCounter",
        //     JsValueEncoder::encode(self.iocore_sound_audio_clocks_until_next_event_counter)
        //         .expect("iocore_sound_audio_clocks_until_next_event_counter"),
        // );
        // object.set(
        //     "IOCore.sound.audioIndex",
        //     JsValueEncoder::encode(self.iocore_sound_audio_index)
        //         .expect("iocore_sound_audio_index"),
        // );
        // object.set(
        //     "IOCore.sound.audioResamplerFirstPassFactor",
        //     JsValueEncoder::encode(self.iocore_sound_audio_resampler_first_pass_factor)
        //         .expect("iocore_sound_audio_resampler_first_pass_factor"),
        // );
        // object.set(
        //     "IOCore.sound.audioTicks",
        //     JsValueEncoder::encode(self.iocore_sound_audio_ticks)
        //         .expect("iocore_sound_audio_ticks"),
        // );
        // object.set(
        //     "IOCore.sound.CGBMixerOutputCacheLeft",
        //     JsValueEncoder::encode(self.iocore_sound_cgbmixer_output_cache_left)
        //         .expect("iocore_sound_cgbmixer_output_cache_left"),
        // );
        // object.set(
        //     "IOCore.sound.CGBMixerOutputCacheLeftFolded",
        //     JsValueEncoder::encode(self.iocore_sound_cgbmixer_output_cache_left_folded)
        //         .expect("iocore_sound_cgbmixer_output_cache_left_folded"),
        // );
        // object.set(
        //     "IOCore.sound.CGBMixerOutputCacheRight",
        //     JsValueEncoder::encode(self.iocore_sound_cgbmixer_output_cache_right)
        //         .expect("iocore_sound_cgbmixer_output_cache_right"),
        // );
        // object.set(
        //     "IOCore.sound.CGBMixerOutputCacheRightFolded",
        //     JsValueEncoder::encode(self.iocore_sound_cgbmixer_output_cache_right_folded)
        //         .expect("iocore_sound_cgbmixer_output_cache_right_folded"),
        // );
        // object.set(
        //     "IOCore.sound.CGBOutputRatio",
        //     JsValueEncoder::encode(self.iocore_sound_cgboutput_ratio)
        //         .expect("iocore_sound_cgboutput_ratio"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.CachedDuty",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_cached_duty)
        //         .expect("iocore_sound_channel1_cached_duty"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.canPlay",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_can_play)
        //         .expect("iocore_sound_channel1_can_play"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.consecutive",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_consecutive)
        //         .expect("iocore_sound_channel1_consecutive"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.currentSampleLeft",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_current_sample_left)
        //         .expect("iocore_sound_channel1_current_sample_left"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.currentSampleRight",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_current_sample_right)
        //         .expect("iocore_sound_channel1_current_sample_right"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.decreaseSweep",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_decrease_sweep)
        //         .expect("iocore_sound_channel1_decrease_sweep"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.DutyTracker",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_duty_tracker)
        //         .expect("iocore_sound_channel1_duty_tracker"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.Enabled",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_enabled)
        //         .expect("iocore_sound_channel1_enabled"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.envelopeSweeps",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_envelope_sweeps)
        //         .expect("iocore_sound_channel1_envelope_sweeps"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.envelopeSweepsLast",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_envelope_sweeps_last)
        //         .expect("iocore_sound_channel1_envelope_sweeps_last"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.envelopeVolume",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_envelope_volume)
        //         .expect("iocore_sound_channel1_envelope_volume"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.frequency",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_frequency)
        //         .expect("iocore_sound_channel1_frequency"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.FrequencyCounter",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_frequency_counter)
        //         .expect("iocore_sound_channel1_frequency_counter"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.frequencySweepDivider",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_frequency_sweep_divider)
        //         .expect("iocore_sound_channel1_frequency_sweep_divider"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.FrequencyTracker",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_frequency_tracker)
        //         .expect("iocore_sound_channel1_frequency_tracker"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.lastTimeSweep",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_last_time_sweep)
        //         .expect("iocore_sound_channel1_last_time_sweep"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.leftEnable",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_left_enable)
        //         .expect("iocore_sound_channel1_left_enable"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.nr10",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_nr10)
        //         .expect("iocore_sound_channel1_nr10"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.nr11",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_nr11)
        //         .expect("iocore_sound_channel1_nr11"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.nr12",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_nr12)
        //         .expect("iocore_sound_channel1_nr12"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.nr14",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_nr14)
        //         .expect("iocore_sound_channel1_nr14"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.rightEnable",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_right_enable)
        //         .expect("iocore_sound_channel1_right_enable"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.ShadowFrequency",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_shadow_frequency)
        //         .expect("iocore_sound_channel1_shadow_frequency"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.SweepFault",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_sweep_fault)
        //         .expect("iocore_sound_channel1_sweep_fault"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.Swept",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_swept)
        //         .expect("iocore_sound_channel1_swept"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.timeSweep",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_time_sweep)
        //         .expect("iocore_sound_channel1_time_sweep"),
        // );
        // object.set(
        //     "IOCore.sound.channel1.totalLength",
        //     JsValueEncoder::encode(self.iocore_sound_channel1_total_length)
        //         .expect("iocore_sound_channel1_total_length"),
        // );
        // object.set(
        //     "IOCore.sound.channel2.nr21",
        //     JsValueEncoder::encode(self.iocore_sound_channel2_nr21)
        //         .expect("iocore_sound_channel2_nr21"),
        // );
        // object.set(
        //     "IOCore.sound.channel2.nr22",
        //     JsValueEncoder::encode(self.iocore_sound_channel2_nr22)
        //         .expect("iocore_sound_channel2_nr22"),
        // );
        // object.set(
        //     "IOCore.sound.channel2.nr23",
        //     JsValueEncoder::encode(self.iocore_sound_channel2_nr23)
        //         .expect("iocore_sound_channel2_nr23"),
        // );
        // object.set(
        //     "IOCore.sound.channel2.nr24",
        //     JsValueEncoder::encode(self.iocore_sound_channel2_nr24)
        //         .expect("iocore_sound_channel2_nr24"),
        // );
        // object.set(
        //     "IOCore.sound.channel2.ShadowFrequency",
        //     JsValueEncoder::encode(self.iocore_sound_channel2_shadow_frequency)
        //         .expect("iocore_sound_channel2_shadow_frequency"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.cachedSample",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_cached_sample)
        //         .expect("iocore_sound_channel3_cached_sample"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.canPlay",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_can_play)
        //         .expect("iocore_sound_channel3_can_play"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.counter",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_counter)
        //         .expect("iocore_sound_channel3_counter"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.FrequencyPeriod",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_frequency_period)
        //         .expect("iocore_sound_channel3_frequency_period"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.lastSampleLookup",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_last_sample_lookup)
        //         .expect("iocore_sound_channel3_last_sample_lookup"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.nr30",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_nr30)
        //         .expect("iocore_sound_channel3_nr30"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.nr31",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_nr31)
        //         .expect("iocore_sound_channel3_nr31"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.nr32",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_nr32)
        //         .expect("iocore_sound_channel3_nr32"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.nr33",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_nr33)
        //         .expect("iocore_sound_channel3_nr33"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.nr34",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_nr34)
        //         .expect("iocore_sound_channel3_nr34"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.patternType",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_pattern_type)
        //         .expect("iocore_sound_channel3_pattern_type"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.PCM",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_pcm)
        //         .expect("iocore_sound_channel3_pcm"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.PCM16",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_pcm16)
        //         .expect("iocore_sound_channel3_pcm16"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.PCM32",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_pcm32)
        //         .expect("iocore_sound_channel3_pcm32"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.WAVERAM16",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_waveram16)
        //         .expect("iocore_sound_channel3_waveram16"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.WAVERAM32",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_waveram32)
        //         .expect("iocore_sound_channel3_waveram32"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.WAVERAM8",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_waveram8)
        //         .expect("iocore_sound_channel3_waveram8"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.WAVERAMBankAccessed",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_waverambank_accessed)
        //         .expect("iocore_sound_channel3_waverambank_accessed"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.WaveRAMBankSize",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_wave_rambank_size)
        //         .expect("iocore_sound_channel3_wave_rambank_size"),
        // );
        // object.set(
        //     "IOCore.sound.channel3.WAVERAMBankSpecified",
        //     JsValueEncoder::encode(self.iocore_sound_channel3_waverambank_specified)
        //         .expect("iocore_sound_channel3_waverambank_specified"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.BitRange",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_bit_range)
        //         .expect("iocore_sound_channel4_bit_range"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.counter",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_counter)
        //         .expect("iocore_sound_channel4_counter"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.currentVolume",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_current_volume)
        //         .expect("iocore_sound_channel4_current_volume"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.FrequencyPeriod",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_frequency_period)
        //         .expect("iocore_sound_channel4_frequency_period"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.lastSampleLookup",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_last_sample_lookup)
        //         .expect("iocore_sound_channel4_last_sample_lookup"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.LSFR15Table",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_lsfr15_table)
        //         .expect("iocore_sound_channel4_lsfr15_table"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.LSFR7Table",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_lsfr7_table)
        //         .expect("iocore_sound_channel4_lsfr7_table"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.noiseSampleTable",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_noise_sample_table)
        //         .expect("iocore_sound_channel4_noise_sample_table"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.nr42",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_nr42)
        //         .expect("iocore_sound_channel4_nr42"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.nr43",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_nr43)
        //         .expect("iocore_sound_channel4_nr43"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.nr44",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_nr44)
        //         .expect("iocore_sound_channel4_nr44"),
        // );
        // object.set(
        //     "IOCore.sound.channel4.VolumeShifter",
        //     JsValueEncoder::encode(self.iocore_sound_channel4_volume_shifter)
        //         .expect("iocore_sound_channel4_volume_shifter"),
        // );
        // object.set(
        //     "IOCore.sound.downsampleInputLeft",
        //     JsValueEncoder::encode(self.iocore_sound_downsample_input_left)
        //         .expect("iocore_sound_downsample_input_left"),
        // );
        // object.set(
        //     "IOCore.sound.downsampleInputRight",
        //     JsValueEncoder::encode(self.iocore_sound_downsample_input_right)
        //         .expect("iocore_sound_downsample_input_right"),
        // );
        // object.set(
        //     "IOCore.sound.FIFOABuffer.buffer",
        //     JsValueEncoder::encode(self.iocore_sound_fifoabuffer_buffer)
        //         .expect("iocore_sound_fifoabuffer_buffer"),
        // );
        // object.set(
        //     "IOCore.sound.FIFOABuffer.count",
        //     JsValueEncoder::encode(self.iocore_sound_fifoabuffer_count)
        //         .expect("iocore_sound_fifoabuffer_count"),
        // );
        // object.set(
        //     "IOCore.sound.FIFOABuffer.position",
        //     JsValueEncoder::encode(self.iocore_sound_fifoabuffer_position)
        //         .expect("iocore_sound_fifoabuffer_position"),
        // );
        // object.set(
        //     "IOCore.sound.FIFOBBuffer.buffer",
        //     JsValueEncoder::encode(self.iocore_sound_fifobbuffer_buffer)
        //         .expect("iocore_sound_fifobbuffer_buffer"),
        // );
        // object.set(
        //     "IOCore.sound.mixerOutputCacheLeft",
        //     JsValueEncoder::encode(self.iocore_sound_mixer_output_cache_left)
        //         .expect("iocore_sound_mixer_output_cache_left"),
        // );
        // object.set(
        //     "IOCore.sound.mixerOutputCacheRight",
        //     JsValueEncoder::encode(self.iocore_sound_mixer_output_cache_right)
        //         .expect("iocore_sound_mixer_output_cache_right"),
        // );
        // object.set(
        //     "IOCore.sound.mixerSoundBIAS",
        //     JsValueEncoder::encode(self.iocore_sound_mixer_sound_bias)
        //         .expect("iocore_sound_mixer_sound_bias"),
        // );
        // object.set(
        //     "IOCore.sound.nr50",
        //     JsValueEncoder::encode(self.iocore_sound_nr50).expect("iocore_sound_nr50"),
        // );
        // object.set(
        //     "IOCore.sound.nr51",
        //     JsValueEncoder::encode(self.iocore_sound_nr51).expect("iocore_sound_nr51"),
        // );
        // object.set(
        //     "IOCore.sound.nr52",
        //     JsValueEncoder::encode(self.iocore_sound_nr52).expect("iocore_sound_nr52"),
        // );
        // object.set(
        //     "IOCore.sound.nr60",
        //     JsValueEncoder::encode(self.iocore_sound_nr60).expect("iocore_sound_nr60"),
        // );
        // object.set(
        //     "IOCore.sound.nr61",
        //     JsValueEncoder::encode(self.iocore_sound_nr61).expect("iocore_sound_nr61"),
        // );
        // object.set(
        //     "IOCore.sound.nr62",
        //     JsValueEncoder::encode(self.iocore_sound_nr62).expect("iocore_sound_nr62"),
        // );
        // object.set(
        //     "IOCore.sound.nr63",
        //     JsValueEncoder::encode(self.iocore_sound_nr63).expect("iocore_sound_nr63"),
        // );
        // object.set(
        //     "IOCore.sound.PWMBitDepthMask",
        //     JsValueEncoder::encode(self.iocore_sound_pwmbit_depth_mask)
        //         .expect("iocore_sound_pwmbit_depth_mask"),
        // );
        // object.set(
        //     "IOCore.sound.PWMBitDepthMaskShadow",
        //     JsValueEncoder::encode(self.iocore_sound_pwmbit_depth_mask_shadow)
        //         .expect("iocore_sound_pwmbit_depth_mask_shadow"),
        // );
        // object.set(
        //     "IOCore.sound.PWMWidth",
        //     JsValueEncoder::encode(self.iocore_sound_pwmwidth).expect("iocore_sound_pwmwidth"),
        // );
        // object.set(
        //     "IOCore.sound.PWMWidthOld",
        //     JsValueEncoder::encode(self.iocore_sound_pwmwidth_old)
        //         .expect("iocore_sound_pwmwidth_old"),
        // );
        // object.set(
        //     "IOCore.sound.PWMWidthShadow",
        //     JsValueEncoder::encode(self.iocore_sound_pwmwidth_shadow)
        //         .expect("iocore_sound_pwmwidth_shadow"),
        // );
        // object.set(
        //     "IOCore.sound.sequencePosition",
        //     JsValueEncoder::encode(self.iocore_sound_sequence_position)
        //         .expect("iocore_sound_sequence_position"),
        // );
        // object.set(
        //     "IOCore.sound.sequencerClocks",
        //     JsValueEncoder::encode(self.iocore_sound_sequencer_clocks)
        //         .expect("iocore_sound_sequencer_clocks"),
        // );
        // object.set(
        //     "IOCore.sound.soundMasterEnabled",
        //     JsValueEncoder::encode(self.iocore_sound_sound_master_enabled)
        //         .expect("iocore_sound_sound_master_enabled"),
        // );
        // object.set(
        //     "IOCore.sound.VinLeftChannelMasterVolume",
        //     JsValueEncoder::encode(self.iocore_sound_vin_left_channel_master_volume)
        //         .expect("iocore_sound_vin_left_channel_master_volume"),
        // );
        // object.set(
        //     "IOCore.sound.VinRightChannelMasterVolume",
        //     JsValueEncoder::encode(self.iocore_sound_vin_right_channel_master_volume)
        //         .expect("iocore_sound_vin_right_channel_master_volume"),
        // );
        object.set(
            "IOCore.systemStatus",
            JsValueEncoder::encode(self.iocore_system_status).expect("iocore_system_status"),
        );
        object.set(
            "IOCore.THUMB.decode",
            JsValueEncoder::encode(self.iocore_thumb_decode).expect("iocore_thumb_decode"),
        );
        object.set(
            "IOCore.THUMB.execute",
            JsValueEncoder::encode(self.iocore_thumb_execute).expect("iocore_thumb_execute"),
        );
        object.set(
            "IOCore.THUMB.fetch",
            JsValueEncoder::encode(self.iocore_thumb_fetch).expect("iocore_thumb_fetch"),
        );
        object.set(
            "IOCore.timer.timer0Control",
            JsValueEncoder::encode(self.iocore_timer_timer0_control)
                .expect("iocore_timer_timer0_control"),
        );
        object.set(
            "IOCore.timer.timer0Counter",
            JsValueEncoder::encode(self.iocore_timer_timer0_counter)
                .expect("iocore_timer_timer0_counter"),
        );
        object.set(
            "IOCore.timer.timer0Enabled",
            JsValueEncoder::encode(self.iocore_timer_timer0_enabled)
                .expect("iocore_timer_timer0_enabled"),
        );
        object.set(
            "IOCore.timer.timer0IRQ",
            JsValueEncoder::encode(self.iocore_timer_timer0_irq).expect("iocore_timer_timer0_irq"),
        );
        object.set(
            "IOCore.timer.timer0Precounter",
            JsValueEncoder::encode(self.iocore_timer_timer0_precounter)
                .expect("iocore_timer_timer0_precounter"),
        );
        object.set(
            "IOCore.timer.timer0Prescalar",
            JsValueEncoder::encode(self.iocore_timer_timer0_prescalar)
                .expect("iocore_timer_timer0_prescalar"),
        );
        object.set(
            "IOCore.timer.timer0PrescalarShifted",
            JsValueEncoder::encode(self.iocore_timer_timer0_prescalar_shifted)
                .expect("iocore_timer_timer0_prescalar_shifted"),
        );
        object.set(
            "IOCore.timer.timer0Reload",
            JsValueEncoder::encode(self.iocore_timer_timer0_reload)
                .expect("iocore_timer_timer0_reload"),
        );
        object.set(
            "IOCore.timer.timer1Control",
            JsValueEncoder::encode(self.iocore_timer_timer1_control)
                .expect("iocore_timer_timer1_control"),
        );
        object.set(
            "IOCore.timer.timer1Counter",
            JsValueEncoder::encode(self.iocore_timer_timer1_counter)
                .expect("iocore_timer_timer1_counter"),
        );
        object.set(
            "IOCore.timer.timer1CountUp",
            JsValueEncoder::encode(self.iocore_timer_timer1_count_up)
                .expect("iocore_timer_timer1_count_up"),
        );
        object.set(
            "IOCore.timer.timer1Enabled",
            JsValueEncoder::encode(self.iocore_timer_timer1_enabled)
                .expect("iocore_timer_timer1_enabled"),
        );
        object.set(
            "IOCore.timer.timer1IRQ",
            JsValueEncoder::encode(self.iocore_timer_timer1_irq).expect("iocore_timer_timer1_irq"),
        );
        object.set(
            "IOCore.timer.timer1Precounter",
            JsValueEncoder::encode(self.iocore_timer_timer1_precounter)
                .expect("iocore_timer_timer1_precounter"),
        );
        object.set(
            "IOCore.timer.timer1Prescalar",
            JsValueEncoder::encode(self.iocore_timer_timer1_prescalar)
                .expect("iocore_timer_timer1_prescalar"),
        );
        object.set(
            "IOCore.timer.timer1PrescalarShifted",
            JsValueEncoder::encode(self.iocore_timer_timer1_prescalar_shifted)
                .expect("iocore_timer_timer1_prescalar_shifted"),
        );
        object.set(
            "IOCore.timer.timer1Reload",
            JsValueEncoder::encode(self.iocore_timer_timer1_reload)
                .expect("iocore_timer_timer1_reload"),
        );
        object.set(
            "IOCore.timer.timer1UseChainedClocks",
            JsValueEncoder::encode(self.iocore_timer_timer1_use_chained_clocks)
                .expect("iocore_timer_timer1_use_chained_clocks"),
        );
        object.set(
            "IOCore.timer.timer1UseMainClocks",
            JsValueEncoder::encode(self.iocore_timer_timer1_use_main_clocks)
                .expect("iocore_timer_timer1_use_main_clocks"),
        );
        object.set(
            "IOCore.timer.timer2Control",
            JsValueEncoder::encode(self.iocore_timer_timer2_control)
                .expect("iocore_timer_timer2_control"),
        );
        object.set(
            "IOCore.timer.timer2Counter",
            JsValueEncoder::encode(self.iocore_timer_timer2_counter)
                .expect("iocore_timer_timer2_counter"),
        );
        object.set(
            "IOCore.timer.timer2CountUp",
            JsValueEncoder::encode(self.iocore_timer_timer2_count_up)
                .expect("iocore_timer_timer2_count_up"),
        );
        object.set(
            "IOCore.timer.timer2Enabled",
            JsValueEncoder::encode(self.iocore_timer_timer2_enabled)
                .expect("iocore_timer_timer2_enabled"),
        );
        object.set(
            "IOCore.timer.timer2IRQ",
            JsValueEncoder::encode(self.iocore_timer_timer2_irq).expect("iocore_timer_timer2_irq"),
        );
        object.set(
            "IOCore.timer.timer2Precounter",
            JsValueEncoder::encode(self.iocore_timer_timer2_precounter)
                .expect("iocore_timer_timer2_precounter"),
        );
        object.set(
            "IOCore.timer.timer2Prescalar",
            JsValueEncoder::encode(self.iocore_timer_timer2_prescalar)
                .expect("iocore_timer_timer2_prescalar"),
        );
        object.set(
            "IOCore.timer.timer2PrescalarShifted",
            JsValueEncoder::encode(self.iocore_timer_timer2_prescalar_shifted)
                .expect("iocore_timer_timer2_prescalar_shifted"),
        );
        object.set(
            "IOCore.timer.timer2Reload",
            JsValueEncoder::encode(self.iocore_timer_timer2_reload)
                .expect("iocore_timer_timer2_reload"),
        );
        object.set(
            "IOCore.timer.timer2UseChainedClocks",
            JsValueEncoder::encode(self.iocore_timer_timer2_use_chained_clocks)
                .expect("iocore_timer_timer2_use_chained_clocks"),
        );
        object.set(
            "IOCore.timer.timer2UseMainClocks",
            JsValueEncoder::encode(self.iocore_timer_timer2_use_main_clocks)
                .expect("iocore_timer_timer2_use_main_clocks"),
        );
        object.set(
            "IOCore.timer.timer3Control",
            JsValueEncoder::encode(self.iocore_timer_timer3_control)
                .expect("iocore_timer_timer3_control"),
        );
        object.set(
            "IOCore.timer.timer3Counter",
            JsValueEncoder::encode(self.iocore_timer_timer3_counter)
                .expect("iocore_timer_timer3_counter"),
        );
        object.set(
            "IOCore.timer.timer3CountUp",
            JsValueEncoder::encode(self.iocore_timer_timer3_count_up)
                .expect("iocore_timer_timer3_count_up"),
        );
        object.set(
            "IOCore.timer.timer3Enabled",
            JsValueEncoder::encode(self.iocore_timer_timer3_enabled)
                .expect("iocore_timer_timer3_enabled"),
        );
        object.set(
            "IOCore.timer.timer3IRQ",
            JsValueEncoder::encode(self.iocore_timer_timer3_irq).expect("iocore_timer_timer3_irq"),
        );
        object.set(
            "IOCore.timer.timer3Precounter",
            JsValueEncoder::encode(self.iocore_timer_timer3_precounter)
                .expect("iocore_timer_timer3_precounter"),
        );
        object.set(
            "IOCore.timer.timer3Prescalar",
            JsValueEncoder::encode(self.iocore_timer_timer3_prescalar)
                .expect("iocore_timer_timer3_prescalar"),
        );
        object.set(
            "IOCore.timer.timer3PrescalarShifted",
            JsValueEncoder::encode(self.iocore_timer_timer3_prescalar_shifted)
                .expect("iocore_timer_timer3_prescalar_shifted"),
        );
        object.set(
            "IOCore.timer.timer3Reload",
            JsValueEncoder::encode(self.iocore_timer_timer3_reload)
                .expect("iocore_timer_timer3_reload"),
        );
        object.set(
            "IOCore.timer.timer3UseChainedClocks",
            JsValueEncoder::encode(self.iocore_timer_timer3_use_chained_clocks)
                .expect("iocore_timer_timer3_use_chained_clocks"),
        );
        object.set(
            "IOCore.timer.timer3UseMainClocks",
            JsValueEncoder::encode(self.iocore_timer_timer3_use_main_clocks)
                .expect("iocore_timer_timer3_use_main_clocks"),
        );
        object.set(
            "IOCore.timerClocks",
            JsValueEncoder::encode(self.iocore_timer_clocks).expect("iocore_timer_clocks"),
        );
        object.set(
            "IOCore.wait.buffer",
            JsValueEncoder::encode(self.iocore_wait_buffer).expect("iocore_wait_buffer"),
        );
        object.set(
            "IOCore.wait.clocks",
            JsValueEncoder::encode(self.iocore_wait_clocks).expect("iocore_wait_clocks"),
        );
        object.set(
            "IOCore.wait.isOAMRendering",
            JsValueEncoder::encode(self.iocore_wait_is_oamrendering)
                .expect("iocore_wait_is_oamrendering"),
        );
        object.set(
            "IOCore.wait.isRendering",
            JsValueEncoder::encode(self.iocore_wait_is_rendering)
                .expect("iocore_wait_is_rendering"),
        );
        object.set(
            "IOCore.wait.nonSequential",
            JsValueEncoder::encode(self.iocore_wait_non_sequential)
                .expect("iocore_wait_non_sequential"),
        );
        object.set(
            "IOCore.wait.POSTBOOT",
            JsValueEncoder::encode(self.iocore_wait_postboot).expect("iocore_wait_postboot"),
        );
        object.set(
            "IOCore.wait.SRAMWaitState",
            JsValueEncoder::encode(self.iocore_wait_sramwait_state)
                .expect("iocore_wait_sramwait_state"),
        );
        object.set(
            "IOCore.wait.WAITCNT0",
            JsValueEncoder::encode(self.iocore_wait_waitcnt0).expect("iocore_wait_waitcnt0"),
        );
        object.set(
            "IOCore.wait.WAITCNT1",
            JsValueEncoder::encode(self.iocore_wait_waitcnt1).expect("iocore_wait_waitcnt1"),
        );
        object.set(
            "IOCore.wait.waitStateClocks16",
            JsValueEncoder::encode(self.iocore_wait_wait_state_clocks16)
                .expect("iocore_wait_wait_state_clocks16"),
        );
        object.set(
            "IOCore.wait.waitStateClocks32",
            JsValueEncoder::encode(self.iocore_wait_wait_state_clocks32)
                .expect("iocore_wait_wait_state_clocks32"),
        );
        object.set(
            "IOCore.wait.WRAMConfiguration",
            JsValueEncoder::encode(self.iocore_wait_wramconfiguration)
                .expect("iocore_wait_wramconfiguration"),
        );
        object.set(
            "IOCore.wait.WRAMWaitState",
            JsValueEncoder::encode(self.iocore_wait_wramwait_state)
                .expect("iocore_wait_wramwait_state"),
        );
        object.set(
            "lastTimestamp",
            JsValueEncoder::encode(self.last_timestamp).expect("last_timestamp"),
        );
        object.set(
            "metricStart",
            JsValueEncoder::encode(self.metric_start).expect("metric_start"),
        );

        object
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
