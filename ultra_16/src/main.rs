#![cfg_attr(not(debug_assertions), deny(warnings))]

fn main() {
    let uut = ultra_16_gs::Blinky::default();
    rust_hdl_bsp_alchitry_cu::synth::generate_bitstream(uut, "firmware/blinky")
}
