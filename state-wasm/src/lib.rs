use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Snapshotter {
    last_snapshot: SaveState,
}

#[wasm_bindgen]
impl Snapshotter {
    #[wasm_bindgen(constructor)]
    pub fn new(js_value: &JsValue) -> Snapshotter {
        Snapshotter {
            last_snapshot: js_value.into_serde::<SaveState>().unwrap(),
        }
    }

    pub fn store_snapshot(&mut self, js_value: &JsValue) {
        self.last_snapshot = js_value.into_serde::<SaveState>().unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub struct SaveState {
    #[serde(rename = "clockCyclesSinceStart")]
    clockCyclesSinceStart: u32,
    #[serde(rename = "IOCore.accumulatedClocks")]
    IOCore_accumulatedClocks: u32,
    #[serde(rename = "IOCore.ARM.decode")]
    IOCore_ARM_decode: u32,
    #[serde(rename = "IOCore.ARM.execute")]
    IOCore_ARM_execute: u32,
    #[serde(rename = "IOCore.ARM.fetch")]
    IOCore_ARM_fetch: u32,
    #[serde(rename = "IOCore.ARM.registers")]
    IOCore_ARM_registers: u32,
    #[serde(rename = "IOCore.ARM.registersUSR")]
    IOCore_ARM_registersUSR: u32,
    #[serde(rename = "IOCore.cartridge.EEPROMStart")]
    IOCore_cartridge_EEPROMStart: u32,
    #[serde(rename = "IOCore.cartridge.flash_is128")]
    IOCore_cartridge_flash_is128: u32,
    #[serde(rename = "IOCore.cartridge.flash_isAtmel")]
    IOCore_cartridge_flash_isAtmel: u32,
    #[serde(rename = "IOCore.cartridge.name")]
    IOCore_cartridge_name: u32,
    #[serde(rename = "IOCore.cpu.modeFlags")]
    IOCore_cpu_modeFlags: u32,
    #[serde(rename = "IOCore.cpu.mul64ResultHigh")]
    IOCore_cpu_mul64ResultHigh: u32,
    #[serde(rename = "IOCore.cpu.mul64ResultLow")]
    IOCore_cpu_mul64ResultLow: u32,
    #[serde(rename = "IOCore.cpu.registersABT")]
    IOCore_cpu_registersABT: u32,
    #[serde(rename = "IOCore.cpu.registersFIQ")]
    IOCore_cpu_registersFIQ: u32,
    #[serde(rename = "IOCore.cpu.registersIRQ")]
    IOCore_cpu_registersIRQ: u32,
    #[serde(rename = "IOCore.cpu.registersSVC")]
    IOCore_cpu_registersSVC: u32,
    #[serde(rename = "IOCore.cpu.registersUND")]
    IOCore_cpu_registersUND: u32,
    #[serde(rename = "IOCore.cpu.SPSR")]
    IOCore_cpu_SPSR: u32,
    #[serde(rename = "IOCore.cpu.triggeredIRQ")]
    IOCore_cpu_triggeredIRQ: u32,
    #[serde(rename = "IOCore.cyclesOveriteratedPreviously")]
    IOCore_cyclesOveriteratedPreviously: u32,
    #[serde(rename = "IOCore.cyclesToIterate")]
    IOCore_cyclesToIterate: u32,
    #[serde(rename = "IOCore.dma.currentMatch")]
    IOCore_dma_currentMatch: u32,
    #[serde(rename = "IOCore.dma.fetch")]
    IOCore_dma_fetch: u32,
    #[serde(rename = "IOCore.dmaChannel0.destination")]
    IOCore_dmaChannel0_destination: u32,
    #[serde(rename = "IOCore.dmaChannel0.destinationControl")]
    IOCore_dmaChannel0_destinationControl: u32,
    #[serde(rename = "IOCore.dmaChannel0.destinationShadow")]
    IOCore_dmaChannel0_destinationShadow: u32,
    #[serde(rename = "IOCore.dmaChannel0.dmaType")]
    IOCore_dmaChannel0_dmaType: u32,
    #[serde(rename = "IOCore.dmaChannel0.enabled")]
    IOCore_dmaChannel0_enabled: u32,
    #[serde(rename = "IOCore.dmaChannel0.irqFlagging")]
    IOCore_dmaChannel0_irqFlagging: u32,
    #[serde(rename = "IOCore.dmaChannel0.is32Bit")]
    IOCore_dmaChannel0_is32Bit: u32,
    #[serde(rename = "IOCore.dmaChannel0.pending")]
    IOCore_dmaChannel0_pending: u32,
    #[serde(rename = "IOCore.dmaChannel0.repeat")]
    IOCore_dmaChannel0_repeat: u32,
    #[serde(rename = "IOCore.dmaChannel0.source")]
    IOCore_dmaChannel0_source: u32,
    #[serde(rename = "IOCore.dmaChannel0.sourceControl")]
    IOCore_dmaChannel0_sourceControl: u32,
    #[serde(rename = "IOCore.dmaChannel0.sourceShadow")]
    IOCore_dmaChannel0_sourceShadow: u32,
    #[serde(rename = "IOCore.dmaChannel0.wordCount")]
    IOCore_dmaChannel0_wordCount: u32,
    #[serde(rename = "IOCore.dmaChannel0.wordCountShadow")]
    IOCore_dmaChannel0_wordCountShadow: u32,
    #[serde(rename = "IOCore.dmaChannel1.destination")]
    IOCore_dmaChannel1_destination: u32,
    #[serde(rename = "IOCore.dmaChannel1.destinationShadow")]
    IOCore_dmaChannel1_destinationShadow: u32,
    #[serde(rename = "IOCore.dmaChannel1.dmaType")]
    IOCore_dmaChannel1_dmaType: u32,
    #[serde(rename = "IOCore.dmaChannel1.enabled")]
    IOCore_dmaChannel1_enabled: u32,
    #[serde(rename = "IOCore.dmaChannel1.is32Bit")]
    IOCore_dmaChannel1_is32Bit: u32,
    #[serde(rename = "IOCore.dmaChannel1.repeat")]
    IOCore_dmaChannel1_repeat: u32,
    #[serde(rename = "IOCore.dmaChannel1.source")]
    IOCore_dmaChannel1_source: u32,
    #[serde(rename = "IOCore.dmaChannel1.sourceShadow")]
    IOCore_dmaChannel1_sourceShadow: u32,
    #[serde(rename = "IOCore.dmaChannel1.wordCount")]
    IOCore_dmaChannel1_wordCount: u32,
    #[serde(rename = "IOCore.dmaChannel1.wordCountShadow")]
    IOCore_dmaChannel1_wordCountShadow: u32,
    #[serde(rename = "IOCore.dmaChannel2.destination")]
    IOCore_dmaChannel2_destination: u32,
    #[serde(rename = "IOCore.dmaChannel2.destinationShadow")]
    IOCore_dmaChannel2_destinationShadow: u32,
    #[serde(rename = "IOCore.dmaChannel2.enabled")]
    IOCore_dmaChannel2_enabled: u32,
    #[serde(rename = "IOCore.dmaChannel2.source")]
    IOCore_dmaChannel2_source: u32,
    #[serde(rename = "IOCore.dmaChannel2.sourceShadow")]
    IOCore_dmaChannel2_sourceShadow: u32,
    #[serde(rename = "IOCore.dmaChannel3.destination")]
    IOCore_dmaChannel3_destination: u32,
    #[serde(rename = "IOCore.dmaChannel3.destinationShadow")]
    IOCore_dmaChannel3_destinationShadow: u32,
    #[serde(rename = "IOCore.dmaChannel3.displaySyncEnableDelay")]
    IOCore_dmaChannel3_displaySyncEnableDelay: u32,
    #[serde(rename = "IOCore.dmaChannel3.gamePakDMA")]
    IOCore_dmaChannel3_gamePakDMA: u32,
    #[serde(rename = "IOCore.dmaChannel3.source")]
    IOCore_dmaChannel3_source: u32,
    #[serde(rename = "IOCore.dmaChannel3.sourceControl")]
    IOCore_dmaChannel3_sourceControl: u32,
    #[serde(rename = "IOCore.dmaChannel3.sourceShadow")]
    IOCore_dmaChannel3_sourceShadow: u32,
    #[serde(rename = "IOCore.dmaChannel3.wordCount")]
    IOCore_dmaChannel3_wordCount: u32,
    #[serde(rename = "IOCore.gfxRenderer.IOData16")]
    IOCore_gfxRenderer_IOData16: u32,
    #[serde(rename = "IOCore.gfxRenderer.IOData32")]
    IOCore_gfxRenderer_IOData32: u32,
    #[serde(rename = "IOCore.gfxRenderer.IOData8")]
    IOCore_gfxRenderer_IOData8: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.backdrop")]
    IOCore_gfxRenderer_renderer_backdrop: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.BGCharacterBaseBlock")]
    IOCore_gfxRenderer_renderer_bg0Renderer_BGCharacterBaseBlock: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.BGLayer")]
    IOCore_gfxRenderer_renderer_bg0Renderer_BGLayer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.BGScreenBaseBlock")]
    IOCore_gfxRenderer_renderer_bg0Renderer_BGScreenBaseBlock: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.BGXCoord")]
    IOCore_gfxRenderer_renderer_bg0Renderer_BGXCoord: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.BGYCoord")]
    IOCore_gfxRenderer_renderer_bg0Renderer_BGYCoord: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.do256")]
    IOCore_gfxRenderer_renderer_bg0Renderer_do256: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.doMosaic")]
    IOCore_gfxRenderer_renderer_bg0Renderer_doMosaic: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.offset")]
    IOCore_gfxRenderer_renderer_bg0Renderer_offset: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.priorityFlag")]
    IOCore_gfxRenderer_renderer_bg0Renderer_priorityFlag: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.scratchBuffer")]
    IOCore_gfxRenderer_renderer_bg0Renderer_scratchBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.tileFetched")]
    IOCore_gfxRenderer_renderer_bg0Renderer_tileFetched: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg0Renderer.tileMode")]
    IOCore_gfxRenderer_renderer_bg0Renderer_tileMode: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg1Renderer.BGLayer")]
    IOCore_gfxRenderer_renderer_bg1Renderer_BGLayer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg1Renderer.BGScreenBaseBlock")]
    IOCore_gfxRenderer_renderer_bg1Renderer_BGScreenBaseBlock: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg1Renderer.BGXCoord")]
    IOCore_gfxRenderer_renderer_bg1Renderer_BGXCoord: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg1Renderer.BGYCoord")]
    IOCore_gfxRenderer_renderer_bg1Renderer_BGYCoord: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg1Renderer.offset")]
    IOCore_gfxRenderer_renderer_bg1Renderer_offset: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg1Renderer.priorityFlag")]
    IOCore_gfxRenderer_renderer_bg1Renderer_priorityFlag: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg1Renderer.scratchBuffer")]
    IOCore_gfxRenderer_renderer_bg1Renderer_scratchBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg1Renderer.tileFetched")]
    IOCore_gfxRenderer_renderer_bg1Renderer_tileFetched: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2FrameBufferRenderer.frameSelect")]
    IOCore_gfxRenderer_renderer_bg2FrameBufferRenderer_frameSelect: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGCharacterBaseBlock")]
    IOCore_gfxRenderer_renderer_bg2MatrixRenderer_BGCharacterBaseBlock: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGDisplayOverflow")]
    IOCore_gfxRenderer_renderer_bg2MatrixRenderer_BGDisplayOverflow: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.BGScreenBaseBlock")]
    IOCore_gfxRenderer_renderer_bg2MatrixRenderer_BGScreenBaseBlock: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.mapSize")]
    IOCore_gfxRenderer_renderer_bg2MatrixRenderer_mapSize: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.mapSizeComparer")]
    IOCore_gfxRenderer_renderer_bg2MatrixRenderer_mapSizeComparer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2MatrixRenderer.palette")]
    IOCore_gfxRenderer_renderer_bg2MatrixRenderer_palette: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2TextRenderer.BGCharacterBaseBlock")]
    IOCore_gfxRenderer_renderer_bg2TextRenderer_BGCharacterBaseBlock: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2TextRenderer.BGLayer")]
    IOCore_gfxRenderer_renderer_bg2TextRenderer_BGLayer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2TextRenderer.BGScreenBaseBlock")]
    IOCore_gfxRenderer_renderer_bg2TextRenderer_BGScreenBaseBlock: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2TextRenderer.BGYCoord")]
    IOCore_gfxRenderer_renderer_bg2TextRenderer_BGYCoord: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2TextRenderer.offset")]
    IOCore_gfxRenderer_renderer_bg2TextRenderer_offset: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2TextRenderer.priorityFlag")]
    IOCore_gfxRenderer_renderer_bg2TextRenderer_priorityFlag: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2TextRenderer.scratchBuffer")]
    IOCore_gfxRenderer_renderer_bg2TextRenderer_scratchBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2TextRenderer.tileFetched")]
    IOCore_gfxRenderer_renderer_bg2TextRenderer_tileFetched: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg2TextRenderer.tileMode")]
    IOCore_gfxRenderer_renderer_bg2TextRenderer_tileMode: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg3MatrixRenderer.BGScreenBaseBlock")]
    IOCore_gfxRenderer_renderer_bg3MatrixRenderer_BGScreenBaseBlock: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg3MatrixRenderer.mapSize")]
    IOCore_gfxRenderer_renderer_bg3MatrixRenderer_mapSize: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg3MatrixRenderer.mapSizeComparer")]
    IOCore_gfxRenderer_renderer_bg3MatrixRenderer_mapSizeComparer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg3TextRenderer.BGLayer")]
    IOCore_gfxRenderer_renderer_bg3TextRenderer_BGLayer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg3TextRenderer.BGScreenBaseBlock")]
    IOCore_gfxRenderer_renderer_bg3TextRenderer_BGScreenBaseBlock: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg3TextRenderer.offset")]
    IOCore_gfxRenderer_renderer_bg3TextRenderer_offset: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg3TextRenderer.priorityFlag")]
    IOCore_gfxRenderer_renderer_bg3TextRenderer_priorityFlag: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg3TextRenderer.scratchBuffer")]
    IOCore_gfxRenderer_renderer_bg3TextRenderer_scratchBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bg3TextRenderer.tileFetched")]
    IOCore_gfxRenderer_renderer_bg3TextRenderer_tileFetched: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdmx")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer0_BGdmx: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdmy")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer0_BGdmy: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdx")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer0_BGdx: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGdy")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer0_BGdy: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGReferenceX")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer0_BGReferenceX: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer0.BGReferenceY")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer0_BGReferenceY: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer0.pb")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer0_pb: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer0.pd")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer0_pd: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer0.scratchBuffer")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer0_scratchBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGdmy")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer1_BGdmy: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGdx")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer1_BGdx: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGReferenceX")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer1_BGReferenceX: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer1.BGReferenceY")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer1_BGReferenceY: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer1.pb")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer1_pb: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer1.pd")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer1_pd: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.bgAffineRenderer1.scratchBuffer")]
    IOCore_gfxRenderer_renderer_bgAffineRenderer1_scratchBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.buffer")]
    IOCore_gfxRenderer_renderer_buffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.colorEffectsRenderer.alphaBlendAmountTarget1")]
    IOCore_gfxRenderer_renderer_colorEffectsRenderer_alphaBlendAmountTarget1: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.colorEffectsRenderer.alphaBlendAmountTarget2")]
    IOCore_gfxRenderer_renderer_colorEffectsRenderer_alphaBlendAmountTarget2: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.colorEffectsRenderer.brightnessEffectAmount")]
    IOCore_gfxRenderer_renderer_colorEffectsRenderer_brightnessEffectAmount: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.colorEffectsRenderer.colorEffectsType")]
    IOCore_gfxRenderer_renderer_colorEffectsRenderer_colorEffectsType: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.colorEffectsRenderer.effectsTarget1")]
    IOCore_gfxRenderer_renderer_colorEffectsRenderer_effectsTarget1: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.colorEffectsRenderer.effectsTarget2")]
    IOCore_gfxRenderer_renderer_colorEffectsRenderer_effectsTarget2: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.compositor.doEffects")]
    IOCore_gfxRenderer_renderer_compositor_doEffects: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.display")]
    IOCore_gfxRenderer_renderer_display: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.displayControl")]
    IOCore_gfxRenderer_renderer_displayControl: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.frameBuffer")]
    IOCore_gfxRenderer_renderer_frameBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.greenSwap")]
    IOCore_gfxRenderer_renderer_greenSwap: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.lastUnrenderedLine")]
    IOCore_gfxRenderer_renderer_lastUnrenderedLine: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.lineBuffer")]
    IOCore_gfxRenderer_renderer_lineBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.mosaicRenderer.BGMosaicHSize")]
    IOCore_gfxRenderer_renderer_mosaicRenderer_BGMosaicHSize: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.mosaicRenderer.BGMosaicVSize")]
    IOCore_gfxRenderer_renderer_mosaicRenderer_BGMosaicVSize: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.mosaicRenderer.OBJMosaicHSize")]
    IOCore_gfxRenderer_renderer_mosaicRenderer_OBJMosaicHSize: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.mosaicRenderer.OBJMosaicVSize")]
    IOCore_gfxRenderer_renderer_mosaicRenderer_OBJMosaicVSize: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objRenderer.cyclesToRender")]
    IOCore_gfxRenderer_renderer_objRenderer_cyclesToRender: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objRenderer.OAMRAM")]
    IOCore_gfxRenderer_renderer_objRenderer_OAMRAM: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objRenderer.OAMRAM16")]
    IOCore_gfxRenderer_renderer_objRenderer_OAMRAM16: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objRenderer.OAMRAM32")]
    IOCore_gfxRenderer_renderer_objRenderer_OAMRAM32: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objRenderer.OAMTable")]
    IOCore_gfxRenderer_renderer_objRenderer_OAMTable: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objRenderer.OBJMatrixParameters")]
    IOCore_gfxRenderer_renderer_objRenderer_OBJMatrixParameters: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objRenderer.scratchBuffer")]
    IOCore_gfxRenderer_renderer_objRenderer_scratchBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objRenderer.scratchOBJBuffer")]
    IOCore_gfxRenderer_renderer_objRenderer_scratchOBJBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objRenderer.scratchWindowBuffer")]
    IOCore_gfxRenderer_renderer_objRenderer_scratchWindowBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objWindowRenderer.compositor.OBJWindowBuffer")]
    IOCore_gfxRenderer_renderer_objWindowRenderer_compositor_OBJWindowBuffer: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.objWindowRenderer.WINOBJOutside")]
    IOCore_gfxRenderer_renderer_objWindowRenderer_WINOBJOutside: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.palette16")]
    IOCore_gfxRenderer_renderer_palette16: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.palette256")]
    IOCore_gfxRenderer_renderer_palette256: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.paletteOBJ16")]
    IOCore_gfxRenderer_renderer_paletteOBJ16: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.paletteOBJ256")]
    IOCore_gfxRenderer_renderer_paletteOBJ256: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.paletteRAM")]
    IOCore_gfxRenderer_renderer_paletteRAM: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.paletteRAM16")]
    IOCore_gfxRenderer_renderer_paletteRAM16: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.paletteRAM32")]
    IOCore_gfxRenderer_renderer_paletteRAM32: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.queuedScanLines")]
    IOCore_gfxRenderer_renderer_queuedScanLines: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.swizzledFrame")]
    IOCore_gfxRenderer_renderer_swizzledFrame: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.totalLinesPassed")]
    IOCore_gfxRenderer_renderer_totalLinesPassed: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.VRAM")]
    IOCore_gfxRenderer_renderer_VRAM: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.VRAM16")]
    IOCore_gfxRenderer_renderer_VRAM16: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.VRAM32")]
    IOCore_gfxRenderer_renderer_VRAM32: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.window0Renderer.compositor.doEffects")]
    IOCore_gfxRenderer_renderer_window0Renderer_compositor_doEffects: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.window0Renderer.windowDisplayControl")]
    IOCore_gfxRenderer_renderer_window0Renderer_windowDisplayControl: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.window0Renderer.WINXCoordLeft")]
    IOCore_gfxRenderer_renderer_window0Renderer_WINXCoordLeft: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.window0Renderer.WINXCoordRight")]
    IOCore_gfxRenderer_renderer_window0Renderer_WINXCoordRight: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.window0Renderer.WINYCoordBottom")]
    IOCore_gfxRenderer_renderer_window0Renderer_WINYCoordBottom: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.window0Renderer.WINYCoordTop")]
    IOCore_gfxRenderer_renderer_window0Renderer_WINYCoordTop: u32,
    #[serde(rename = "IOCore.gfxRenderer.renderer.WINOutside")]
    IOCore_gfxRenderer_renderer_WINOutside: u32,
    #[serde(rename = "IOCore.gfxState.currentScanLine")]
    IOCore_gfxState_currentScanLine: u32,
    #[serde(rename = "IOCore.gfxState.IRQFlags")]
    IOCore_gfxState_IRQFlags: u32,
    #[serde(rename = "IOCore.gfxState.LCDTicks")]
    IOCore_gfxState_LCDTicks: u32,
    #[serde(rename = "IOCore.gfxState.renderedScanLine")]
    IOCore_gfxState_renderedScanLine: u32,
    #[serde(rename = "IOCore.gfxState.statusFlags")]
    IOCore_gfxState_statusFlags: u32,
    #[serde(rename = "IOCore.gfxState.VCounter")]
    IOCore_gfxState_VCounter: u32,
    #[serde(rename = "IOCore.irq.interruptsEnabled")]
    IOCore_irq_interruptsEnabled: u32,
    #[serde(rename = "IOCore.irq.interruptsRequested")]
    IOCore_irq_interruptsRequested: u32,
    #[serde(rename = "IOCore.joypad.keyInput")]
    IOCore_joypad_keyInput: u32,
    #[serde(rename = "IOCore.joypad.keyInterrupt")]
    IOCore_joypad_keyInterrupt: u32,
    #[serde(rename = "IOCore.memory.externalRAM")]
    IOCore_memory_externalRAM: u32,
    #[serde(rename = "IOCore.memory.externalRAM16")]
    IOCore_memory_externalRAM16: u32,
    #[serde(rename = "IOCore.memory.externalRAM32")]
    IOCore_memory_externalRAM32: u32,
    #[serde(rename = "IOCore.memory.internalRAM")]
    IOCore_memory_internalRAM: u32,
    #[serde(rename = "IOCore.memory.internalRAM16")]
    IOCore_memory_internalRAM16: u32,
    #[serde(rename = "IOCore.memory.internalRAM32")]
    IOCore_memory_internalRAM32: u32,
    #[serde(rename = "IOCore.memory.irq.IME")]
    IOCore_memory_irq_IME: u32,
    #[serde(rename = "IOCore.memory.lastBIOSREAD")]
    IOCore_memory_lastBIOSREAD: u32,
    #[serde(rename = "IOCore.memory.WRAMControlFlags")]
    IOCore_memory_WRAMControlFlags: u32,
    #[serde(rename = "IOCore.nextEventClocks")]
    IOCore_nextEventClocks: u32,
    #[serde(rename = "IOCore.saves.EEPROMChip.address")]
    IOCore_saves_EEPROMChip_address: u32,
    #[serde(rename = "IOCore.saves.EEPROMChip.bitsProcessed")]
    IOCore_saves_EEPROMChip_bitsProcessed: u32,
    #[serde(rename = "IOCore.saves.EEPROMChip.buffer")]
    IOCore_saves_EEPROMChip_buffer: u32,
    #[serde(rename = "IOCore.saves.EEPROMChip.largestSizePossible")]
    IOCore_saves_EEPROMChip_largestSizePossible: u32,
    #[serde(rename = "IOCore.saves.EEPROMChip.mode")]
    IOCore_saves_EEPROMChip_mode: u32,
    #[serde(rename = "IOCore.saves.FLASHChip.BANKOffset")]
    IOCore_saves_FLASHChip_BANKOffset: u32,
    #[serde(rename = "IOCore.saves.FLASHChip.flashCommand")]
    IOCore_saves_FLASHChip_flashCommand: u32,
    #[serde(rename = "IOCore.saves.FLASHChip.flashCommandUnlockStage")]
    IOCore_saves_FLASHChip_flashCommandUnlockStage: u32,
    #[serde(rename = "IOCore.saves.FLASHChip.largestSizePossible")]
    IOCore_saves_FLASHChip_largestSizePossible: u32,
    #[serde(rename = "IOCore.saves.FLASHChip.notATMEL")]
    IOCore_saves_FLASHChip_notATMEL: u32,
    #[serde(rename = "IOCore.saves.FLASHChip.saves")]
    IOCore_saves_FLASHChip_saves: u32,
    #[serde(rename = "IOCore.saves.FLASHChip.writeBytesLeft")]
    IOCore_saves_FLASHChip_writeBytesLeft: u32,
    #[serde(rename = "IOCore.saves.GPIOChip.data")]
    IOCore_saves_GPIOChip_data: u32,
    #[serde(rename = "IOCore.saves.GPIOChip.direction")]
    IOCore_saves_GPIOChip_direction: u32,
    #[serde(rename = "IOCore.saves.GPIOChip.readWrite")]
    IOCore_saves_GPIOChip_readWrite: u32,
    #[serde(rename = "IOCore.saves.GPIOChip.type")]
    IOCore_saves_GPIOChip_type: u32,
    #[serde(rename = "IOCore.saves.saveType")]
    IOCore_saves_saveType: u32,
    #[serde(rename = "IOCore.saves.SRAMChip.saves")]
    IOCore_saves_SRAMChip_saves: u32,
    #[serde(rename = "IOCore.saves.SRAMChip.TILTChip")]
    IOCore_saves_SRAMChip_TILTChip: u32,
    #[serde(rename = "IOCore.saves.SRAMChip.TILTChipUnlocked")]
    IOCore_saves_SRAMChip_TILTChipUnlocked: u32,
    #[serde(rename = "IOCore.saves.UNDETERMINED.possible")]
    IOCore_saves_UNDETERMINED_possible: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_CNTL_FLAGS")]
    IOCore_serial_JOYBUS_CNTL_FLAGS: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_IRQ")]
    IOCore_serial_JOYBUS_IRQ: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_RECV0")]
    IOCore_serial_JOYBUS_RECV0: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_RECV1")]
    IOCore_serial_JOYBUS_RECV1: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_RECV2")]
    IOCore_serial_JOYBUS_RECV2: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_RECV3")]
    IOCore_serial_JOYBUS_RECV3: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_SEND0")]
    IOCore_serial_JOYBUS_SEND0: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_SEND1")]
    IOCore_serial_JOYBUS_SEND1: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_SEND2")]
    IOCore_serial_JOYBUS_SEND2: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_SEND3")]
    IOCore_serial_JOYBUS_SEND3: u32,
    #[serde(rename = "IOCore.serial.JOYBUS_STAT")]
    IOCore_serial_JOYBUS_STAT: u32,
    #[serde(rename = "IOCore.serial.RCNTDataBitFlow")]
    IOCore_serial_RCNTDataBitFlow: u32,
    #[serde(rename = "IOCore.serial.RCNTDataBits")]
    IOCore_serial_RCNTDataBits: u32,
    #[serde(rename = "IOCore.serial.RCNTIRQ")]
    IOCore_serial_RCNTIRQ: u32,
    #[serde(rename = "IOCore.serial.RCNTMode")]
    IOCore_serial_RCNTMode: u32,
    #[serde(rename = "IOCore.serial.serialBitsShifted")]
    IOCore_serial_serialBitsShifted: u32,
    #[serde(rename = "IOCore.serial.shiftClocks")]
    IOCore_serial_shiftClocks: u32,
    #[serde(rename = "IOCore.serial.SIOBaudRate")]
    IOCore_serial_SIOBaudRate: u32,
    #[serde(rename = "IOCore.serial.SIOCNT_IRQ")]
    IOCore_serial_SIOCNT_IRQ: u32,
    #[serde(rename = "IOCore.serial.SIOCNT_MODE")]
    IOCore_serial_SIOCNT_MODE: u32,
    #[serde(rename = "IOCore.serial.SIOCNT_UART_CTS")]
    IOCore_serial_SIOCNT_UART_CTS: u32,
    #[serde(rename = "IOCore.serial.SIOCNT_UART_FIFO")]
    IOCore_serial_SIOCNT_UART_FIFO: u32,
    #[serde(rename = "IOCore.serial.SIOCNT_UART_FIFO_ENABLE")]
    IOCore_serial_SIOCNT_UART_FIFO_ENABLE: u32,
    #[serde(rename = "IOCore.serial.SIOCNT_UART_MISC")]
    IOCore_serial_SIOCNT_UART_MISC: u32,
    #[serde(rename = "IOCore.serial.SIOCNT_UART_PARITY_ENABLE")]
    IOCore_serial_SIOCNT_UART_PARITY_ENABLE: u32,
    #[serde(rename = "IOCore.serial.SIOCNT_UART_RECV_ENABLE")]
    IOCore_serial_SIOCNT_UART_RECV_ENABLE: u32,
    #[serde(rename = "IOCore.serial.SIOCNT_UART_SEND_ENABLE")]
    IOCore_serial_SIOCNT_UART_SEND_ENABLE: u32,
    #[serde(rename = "IOCore.serial.SIOCNT0_DATA")]
    IOCore_serial_SIOCNT0_DATA: u32,
    #[serde(rename = "IOCore.serial.SIOCOMMERROR")]
    IOCore_serial_SIOCOMMERROR: u32,
    #[serde(rename = "IOCore.serial.SIODATA_A")]
    IOCore_serial_SIODATA_A: u32,
    #[serde(rename = "IOCore.serial.SIODATA_B")]
    IOCore_serial_SIODATA_B: u32,
    #[serde(rename = "IOCore.serial.SIODATA_C")]
    IOCore_serial_SIODATA_C: u32,
    #[serde(rename = "IOCore.serial.SIODATA_D")]
    IOCore_serial_SIODATA_D: u32,
    #[serde(rename = "IOCore.serial.SIODATA8")]
    IOCore_serial_SIODATA8: u32,
    #[serde(rename = "IOCore.serial.SIOMULT_PLAYER_NUMBER")]
    IOCore_serial_SIOMULT_PLAYER_NUMBER: u32,
    #[serde(rename = "IOCore.serial.SIOShiftClockDivider")]
    IOCore_serial_SIOShiftClockDivider: u32,
    #[serde(rename = "IOCore.serial.SIOShiftClockExternal")]
    IOCore_serial_SIOShiftClockExternal: u32,
    #[serde(rename = "IOCore.serial.SIOTransferStarted")]
    IOCore_serial_SIOTransferStarted: u32,
    #[serde(rename = "IOCore.serialClocks")]
    IOCore_serialClocks: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundA")]
    IOCore_sound_AGBDirectSoundA: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundAFolded")]
    IOCore_sound_AGBDirectSoundAFolded: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundALeftCanPlay")]
    IOCore_sound_AGBDirectSoundALeftCanPlay: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundARightCanPlay")]
    IOCore_sound_AGBDirectSoundARightCanPlay: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundAShifter")]
    IOCore_sound_AGBDirectSoundAShifter: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundATimer")]
    IOCore_sound_AGBDirectSoundATimer: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundB")]
    IOCore_sound_AGBDirectSoundB: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundBFolded")]
    IOCore_sound_AGBDirectSoundBFolded: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundBLeftCanPlay")]
    IOCore_sound_AGBDirectSoundBLeftCanPlay: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundBRightCanPlay")]
    IOCore_sound_AGBDirectSoundBRightCanPlay: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundBShifter")]
    IOCore_sound_AGBDirectSoundBShifter: u32,
    #[serde(rename = "IOCore.sound.AGBDirectSoundBTimer")]
    IOCore_sound_AGBDirectSoundBTimer: u32,
    #[serde(rename = "IOCore.sound.audioClocksUntilNextEvent")]
    IOCore_sound_audioClocksUntilNextEvent: u32,
    #[serde(rename = "IOCore.sound.audioClocksUntilNextEventCounter")]
    IOCore_sound_audioClocksUntilNextEventCounter: u32,
    #[serde(rename = "IOCore.sound.audioIndex")]
    IOCore_sound_audioIndex: u32,
    #[serde(rename = "IOCore.sound.audioResamplerFirstPassFactor")]
    IOCore_sound_audioResamplerFirstPassFactor: u32,
    #[serde(rename = "IOCore.sound.audioTicks")]
    IOCore_sound_audioTicks: u32,
    #[serde(rename = "IOCore.sound.CGBMixerOutputCacheLeft")]
    IOCore_sound_CGBMixerOutputCacheLeft: u32,
    #[serde(rename = "IOCore.sound.CGBMixerOutputCacheLeftFolded")]
    IOCore_sound_CGBMixerOutputCacheLeftFolded: u32,
    #[serde(rename = "IOCore.sound.CGBMixerOutputCacheRight")]
    IOCore_sound_CGBMixerOutputCacheRight: u32,
    #[serde(rename = "IOCore.sound.CGBMixerOutputCacheRightFolded")]
    IOCore_sound_CGBMixerOutputCacheRightFolded: u32,
    #[serde(rename = "IOCore.sound.CGBOutputRatio")]
    IOCore_sound_CGBOutputRatio: u32,
    #[serde(rename = "IOCore.sound.channel1.CachedDuty")]
    IOCore_sound_channel1_CachedDuty: u32,
    #[serde(rename = "IOCore.sound.channel1.canPlay")]
    IOCore_sound_channel1_canPlay: u32,
    #[serde(rename = "IOCore.sound.channel1.consecutive")]
    IOCore_sound_channel1_consecutive: u32,
    #[serde(rename = "IOCore.sound.channel1.currentSampleLeft")]
    IOCore_sound_channel1_currentSampleLeft: u32,
    #[serde(rename = "IOCore.sound.channel1.currentSampleRight")]
    IOCore_sound_channel1_currentSampleRight: u32,
    #[serde(rename = "IOCore.sound.channel1.decreaseSweep")]
    IOCore_sound_channel1_decreaseSweep: u32,
    #[serde(rename = "IOCore.sound.channel1.DutyTracker")]
    IOCore_sound_channel1_DutyTracker: u32,
    #[serde(rename = "IOCore.sound.channel1.Enabled")]
    IOCore_sound_channel1_Enabled: u32,
    #[serde(rename = "IOCore.sound.channel1.envelopeSweeps")]
    IOCore_sound_channel1_envelopeSweeps: u32,
    #[serde(rename = "IOCore.sound.channel1.envelopeSweepsLast")]
    IOCore_sound_channel1_envelopeSweepsLast: u32,
    #[serde(rename = "IOCore.sound.channel1.envelopeVolume")]
    IOCore_sound_channel1_envelopeVolume: u32,
    #[serde(rename = "IOCore.sound.channel1.frequency")]
    IOCore_sound_channel1_frequency: u32,
    #[serde(rename = "IOCore.sound.channel1.FrequencyCounter")]
    IOCore_sound_channel1_FrequencyCounter: u32,
    #[serde(rename = "IOCore.sound.channel1.frequencySweepDivider")]
    IOCore_sound_channel1_frequencySweepDivider: u32,
    #[serde(rename = "IOCore.sound.channel1.FrequencyTracker")]
    IOCore_sound_channel1_FrequencyTracker: u32,
    #[serde(rename = "IOCore.sound.channel1.lastTimeSweep")]
    IOCore_sound_channel1_lastTimeSweep: u32,
    #[serde(rename = "IOCore.sound.channel1.leftEnable")]
    IOCore_sound_channel1_leftEnable: u32,
    #[serde(rename = "IOCore.sound.channel1.nr10")]
    IOCore_sound_channel1_nr10: u32,
    #[serde(rename = "IOCore.sound.channel1.nr11")]
    IOCore_sound_channel1_nr11: u32,
    #[serde(rename = "IOCore.sound.channel1.nr12")]
    IOCore_sound_channel1_nr12: u32,
    #[serde(rename = "IOCore.sound.channel1.nr14")]
    IOCore_sound_channel1_nr14: u32,
    #[serde(rename = "IOCore.sound.channel1.rightEnable")]
    IOCore_sound_channel1_rightEnable: u32,
    #[serde(rename = "IOCore.sound.channel1.ShadowFrequency")]
    IOCore_sound_channel1_ShadowFrequency: u32,
    #[serde(rename = "IOCore.sound.channel1.SweepFault")]
    IOCore_sound_channel1_SweepFault: u32,
    #[serde(rename = "IOCore.sound.channel1.Swept")]
    IOCore_sound_channel1_Swept: u32,
    #[serde(rename = "IOCore.sound.channel1.timeSweep")]
    IOCore_sound_channel1_timeSweep: u32,
    #[serde(rename = "IOCore.sound.channel1.totalLength")]
    IOCore_sound_channel1_totalLength: u32,
    #[serde(rename = "IOCore.sound.channel2.nr21")]
    IOCore_sound_channel2_nr21: u32,
    #[serde(rename = "IOCore.sound.channel2.nr22")]
    IOCore_sound_channel2_nr22: u32,
    #[serde(rename = "IOCore.sound.channel2.nr23")]
    IOCore_sound_channel2_nr23: u32,
    #[serde(rename = "IOCore.sound.channel2.nr24")]
    IOCore_sound_channel2_nr24: u32,
    #[serde(rename = "IOCore.sound.channel2.ShadowFrequency")]
    IOCore_sound_channel2_ShadowFrequency: u32,
    #[serde(rename = "IOCore.sound.channel3.cachedSample")]
    IOCore_sound_channel3_cachedSample: u32,
    #[serde(rename = "IOCore.sound.channel3.canPlay")]
    IOCore_sound_channel3_canPlay: u32,
    #[serde(rename = "IOCore.sound.channel3.counter")]
    IOCore_sound_channel3_counter: u32,
    #[serde(rename = "IOCore.sound.channel3.FrequencyPeriod")]
    IOCore_sound_channel3_FrequencyPeriod: u32,
    #[serde(rename = "IOCore.sound.channel3.lastSampleLookup")]
    IOCore_sound_channel3_lastSampleLookup: u32,
    #[serde(rename = "IOCore.sound.channel3.nr30")]
    IOCore_sound_channel3_nr30: u32,
    #[serde(rename = "IOCore.sound.channel3.nr31")]
    IOCore_sound_channel3_nr31: u32,
    #[serde(rename = "IOCore.sound.channel3.nr32")]
    IOCore_sound_channel3_nr32: u32,
    #[serde(rename = "IOCore.sound.channel3.nr33")]
    IOCore_sound_channel3_nr33: u32,
    #[serde(rename = "IOCore.sound.channel3.nr34")]
    IOCore_sound_channel3_nr34: u32,
    #[serde(rename = "IOCore.sound.channel3.patternType")]
    IOCore_sound_channel3_patternType: u32,
    #[serde(rename = "IOCore.sound.channel3.PCM")]
    IOCore_sound_channel3_PCM: u32,
    #[serde(rename = "IOCore.sound.channel3.PCM16")]
    IOCore_sound_channel3_PCM16: u32,
    #[serde(rename = "IOCore.sound.channel3.PCM32")]
    IOCore_sound_channel3_PCM32: u32,
    #[serde(rename = "IOCore.sound.channel3.WAVERAM16")]
    IOCore_sound_channel3_WAVERAM16: u32,
    #[serde(rename = "IOCore.sound.channel3.WAVERAM32")]
    IOCore_sound_channel3_WAVERAM32: u32,
    #[serde(rename = "IOCore.sound.channel3.WAVERAM8")]
    IOCore_sound_channel3_WAVERAM8: u32,
    #[serde(rename = "IOCore.sound.channel3.WAVERAMBankAccessed")]
    IOCore_sound_channel3_WAVERAMBankAccessed: u32,
    #[serde(rename = "IOCore.sound.channel3.WaveRAMBankSize")]
    IOCore_sound_channel3_WaveRAMBankSize: u32,
    #[serde(rename = "IOCore.sound.channel3.WAVERAMBankSpecified")]
    IOCore_sound_channel3_WAVERAMBankSpecified: u32,
    #[serde(rename = "IOCore.sound.channel4.BitRange")]
    IOCore_sound_channel4_BitRange: u32,
    #[serde(rename = "IOCore.sound.channel4.counter")]
    IOCore_sound_channel4_counter: u32,
    #[serde(rename = "IOCore.sound.channel4.currentVolume")]
    IOCore_sound_channel4_currentVolume: u32,
    #[serde(rename = "IOCore.sound.channel4.FrequencyPeriod")]
    IOCore_sound_channel4_FrequencyPeriod: u32,
    #[serde(rename = "IOCore.sound.channel4.lastSampleLookup")]
    IOCore_sound_channel4_lastSampleLookup: u32,
    #[serde(rename = "IOCore.sound.channel4.LSFR15Table")]
    IOCore_sound_channel4_LSFR15Table: u32,
    #[serde(rename = "IOCore.sound.channel4.LSFR7Table")]
    IOCore_sound_channel4_LSFR7Table: u32,
    #[serde(rename = "IOCore.sound.channel4.noiseSampleTable")]
    IOCore_sound_channel4_noiseSampleTable: u32,
    #[serde(rename = "IOCore.sound.channel4.nr42")]
    IOCore_sound_channel4_nr42: u32,
    #[serde(rename = "IOCore.sound.channel4.nr43")]
    IOCore_sound_channel4_nr43: u32,
    #[serde(rename = "IOCore.sound.channel4.nr44")]
    IOCore_sound_channel4_nr44: u32,
    #[serde(rename = "IOCore.sound.channel4.VolumeShifter")]
    IOCore_sound_channel4_VolumeShifter: u32,
    #[serde(rename = "IOCore.sound.downsampleInputLeft")]
    IOCore_sound_downsampleInputLeft: u32,
    #[serde(rename = "IOCore.sound.downsampleInputRight")]
    IOCore_sound_downsampleInputRight: u32,
    #[serde(rename = "IOCore.sound.FIFOABuffer.buffer")]
    IOCore_sound_FIFOABuffer_buffer: u32,
    #[serde(rename = "IOCore.sound.FIFOABuffer.count")]
    IOCore_sound_FIFOABuffer_count: u32,
    #[serde(rename = "IOCore.sound.FIFOABuffer.position")]
    IOCore_sound_FIFOABuffer_position: u32,
    #[serde(rename = "IOCore.sound.FIFOBBuffer.buffer")]
    IOCore_sound_FIFOBBuffer_buffer: u32,
    #[serde(rename = "IOCore.sound.mixerOutputCacheLeft")]
    IOCore_sound_mixerOutputCacheLeft: u32,
    #[serde(rename = "IOCore.sound.mixerOutputCacheRight")]
    IOCore_sound_mixerOutputCacheRight: u32,
    #[serde(rename = "IOCore.sound.mixerSoundBIAS")]
    IOCore_sound_mixerSoundBIAS: u32,
    #[serde(rename = "IOCore.sound.nr50")]
    IOCore_sound_nr50: u32,
    #[serde(rename = "IOCore.sound.nr51")]
    IOCore_sound_nr51: u32,
    #[serde(rename = "IOCore.sound.nr52")]
    IOCore_sound_nr52: u32,
    #[serde(rename = "IOCore.sound.nr60")]
    IOCore_sound_nr60: u32,
    #[serde(rename = "IOCore.sound.nr61")]
    IOCore_sound_nr61: u32,
    #[serde(rename = "IOCore.sound.nr62")]
    IOCore_sound_nr62: u32,
    #[serde(rename = "IOCore.sound.nr63")]
    IOCore_sound_nr63: u32,
    #[serde(rename = "IOCore.sound.PWMBitDepthMask")]
    IOCore_sound_PWMBitDepthMask: u32,
    #[serde(rename = "IOCore.sound.PWMBitDepthMaskShadow")]
    IOCore_sound_PWMBitDepthMaskShadow: u32,
    #[serde(rename = "IOCore.sound.PWMWidth")]
    IOCore_sound_PWMWidth: u32,
    #[serde(rename = "IOCore.sound.PWMWidthOld")]
    IOCore_sound_PWMWidthOld: u32,
    #[serde(rename = "IOCore.sound.PWMWidthShadow")]
    IOCore_sound_PWMWidthShadow: u32,
    #[serde(rename = "IOCore.sound.sequencePosition")]
    IOCore_sound_sequencePosition: u32,
    #[serde(rename = "IOCore.sound.sequencerClocks")]
    IOCore_sound_sequencerClocks: u32,
    #[serde(rename = "IOCore.sound.soundMasterEnabled")]
    IOCore_sound_soundMasterEnabled: u32,
    #[serde(rename = "IOCore.sound.VinLeftChannelMasterVolume")]
    IOCore_sound_VinLeftChannelMasterVolume: u32,
    #[serde(rename = "IOCore.sound.VinRightChannelMasterVolume")]
    IOCore_sound_VinRightChannelMasterVolume: u32,
    #[serde(rename = "IOCore.systemStatus")]
    IOCore_systemStatus: u32,
    #[serde(rename = "IOCore.THUMB.decode")]
    IOCore_THUMB_decode: u32,
    #[serde(rename = "IOCore.THUMB.execute")]
    IOCore_THUMB_execute: u32,
    #[serde(rename = "IOCore.THUMB.fetch")]
    IOCore_THUMB_fetch: u32,
    #[serde(rename = "IOCore.timer.timer0Control")]
    IOCore_timer_timer0Control: u32,
    #[serde(rename = "IOCore.timer.timer0Counter")]
    IOCore_timer_timer0Counter: u32,
    #[serde(rename = "IOCore.timer.timer0Enabled")]
    IOCore_timer_timer0Enabled: u32,
    #[serde(rename = "IOCore.timer.timer0IRQ")]
    IOCore_timer_timer0IRQ: u32,
    #[serde(rename = "IOCore.timer.timer0Precounter")]
    IOCore_timer_timer0Precounter: u32,
    #[serde(rename = "IOCore.timer.timer0Prescalar")]
    IOCore_timer_timer0Prescalar: u32,
    #[serde(rename = "IOCore.timer.timer0PrescalarShifted")]
    IOCore_timer_timer0PrescalarShifted: u32,
    #[serde(rename = "IOCore.timer.timer0Reload")]
    IOCore_timer_timer0Reload: u32,
    #[serde(rename = "IOCore.timer.timer1Control")]
    IOCore_timer_timer1Control: u32,
    #[serde(rename = "IOCore.timer.timer1Counter")]
    IOCore_timer_timer1Counter: u32,
    #[serde(rename = "IOCore.timer.timer1CountUp")]
    IOCore_timer_timer1CountUp: u32,
    #[serde(rename = "IOCore.timer.timer1Enabled")]
    IOCore_timer_timer1Enabled: u32,
    #[serde(rename = "IOCore.timer.timer1IRQ")]
    IOCore_timer_timer1IRQ: u32,
    #[serde(rename = "IOCore.timer.timer1Precounter")]
    IOCore_timer_timer1Precounter: u32,
    #[serde(rename = "IOCore.timer.timer1Prescalar")]
    IOCore_timer_timer1Prescalar: u32,
    #[serde(rename = "IOCore.timer.timer1PrescalarShifted")]
    IOCore_timer_timer1PrescalarShifted: u32,
    #[serde(rename = "IOCore.timer.timer1Reload")]
    IOCore_timer_timer1Reload: u32,
    #[serde(rename = "IOCore.timer.timer1UseChainedClocks")]
    IOCore_timer_timer1UseChainedClocks: u32,
    #[serde(rename = "IOCore.timer.timer1UseMainClocks")]
    IOCore_timer_timer1UseMainClocks: u32,
    #[serde(rename = "IOCore.timer.timer2Control")]
    IOCore_timer_timer2Control: u32,
    #[serde(rename = "IOCore.timer.timer2Counter")]
    IOCore_timer_timer2Counter: u32,
    #[serde(rename = "IOCore.timer.timer2CountUp")]
    IOCore_timer_timer2CountUp: u32,
    #[serde(rename = "IOCore.timer.timer2Enabled")]
    IOCore_timer_timer2Enabled: u32,
    #[serde(rename = "IOCore.timer.timer2IRQ")]
    IOCore_timer_timer2IRQ: u32,
    #[serde(rename = "IOCore.timer.timer2Precounter")]
    IOCore_timer_timer2Precounter: u32,
    #[serde(rename = "IOCore.timer.timer2Prescalar")]
    IOCore_timer_timer2Prescalar: u32,
    #[serde(rename = "IOCore.timer.timer2PrescalarShifted")]
    IOCore_timer_timer2PrescalarShifted: u32,
    #[serde(rename = "IOCore.timer.timer2Reload")]
    IOCore_timer_timer2Reload: u32,
    #[serde(rename = "IOCore.timer.timer2UseChainedClocks")]
    IOCore_timer_timer2UseChainedClocks: u32,
    #[serde(rename = "IOCore.timer.timer2UseMainClocks")]
    IOCore_timer_timer2UseMainClocks: u32,
    #[serde(rename = "IOCore.timer.timer3Control")]
    IOCore_timer_timer3Control: u32,
    #[serde(rename = "IOCore.timer.timer3Counter")]
    IOCore_timer_timer3Counter: u32,
    #[serde(rename = "IOCore.timer.timer3CountUp")]
    IOCore_timer_timer3CountUp: u32,
    #[serde(rename = "IOCore.timer.timer3Enabled")]
    IOCore_timer_timer3Enabled: u32,
    #[serde(rename = "IOCore.timer.timer3IRQ")]
    IOCore_timer_timer3IRQ: u32,
    #[serde(rename = "IOCore.timer.timer3Precounter")]
    IOCore_timer_timer3Precounter: u32,
    #[serde(rename = "IOCore.timer.timer3Prescalar")]
    IOCore_timer_timer3Prescalar: u32,
    #[serde(rename = "IOCore.timer.timer3PrescalarShifted")]
    IOCore_timer_timer3PrescalarShifted: u32,
    #[serde(rename = "IOCore.timer.timer3Reload")]
    IOCore_timer_timer3Reload: u32,
    #[serde(rename = "IOCore.timer.timer3UseChainedClocks")]
    IOCore_timer_timer3UseChainedClocks: u32,
    #[serde(rename = "IOCore.timer.timer3UseMainClocks")]
    IOCore_timer_timer3UseMainClocks: u32,
    #[serde(rename = "IOCore.timerClocks")]
    IOCore_timerClocks: u32,
    #[serde(rename = "IOCore.wait.buffer")]
    IOCore_wait_buffer: u32,
    #[serde(rename = "IOCore.wait.clocks")]
    IOCore_wait_clocks: u32,
    #[serde(rename = "IOCore.wait.isOAMRendering")]
    IOCore_wait_isOAMRendering: u32,
    #[serde(rename = "IOCore.wait.isRendering")]
    IOCore_wait_isRendering: u32,
    #[serde(rename = "IOCore.wait.nonSequential")]
    IOCore_wait_nonSequential: u32,
    #[serde(rename = "IOCore.wait.POSTBOOT")]
    IOCore_wait_POSTBOOT: u32,
    #[serde(rename = "IOCore.wait.SRAMWaitState")]
    IOCore_wait_SRAMWaitState: u32,
    #[serde(rename = "IOCore.wait.WAITCNT0")]
    IOCore_wait_WAITCNT0: u32,
    #[serde(rename = "IOCore.wait.WAITCNT1")]
    IOCore_wait_WAITCNT1: u32,
    #[serde(rename = "IOCore.wait.waitStateClocks16")]
    IOCore_wait_waitStateClocks16: u32,
    #[serde(rename = "IOCore.wait.waitStateClocks32")]
    IOCore_wait_waitStateClocks32: u32,
    #[serde(rename = "IOCore.wait.WRAMConfiguration")]
    IOCore_wait_WRAMConfiguration: u32,
    #[serde(rename = "IOCore.wait.WRAMWaitState")]
    IOCore_wait_WRAMWaitState: u32,
    #[serde(rename = "lastTimestamp")]
    lastTimestamp: u32,
    #[serde(rename = "metricStart")]
    metricStart: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
