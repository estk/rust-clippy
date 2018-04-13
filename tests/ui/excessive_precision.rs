#![feature(plugin, custom_attribute)]
#![warn(excessive_precision)]

fn main() {
    let too_suf: f32 = 6.283_185_307_f32;
    const TOO_SUF: f32 = 6.283_185_307_f32;

    let too: f32 = 6.283_185_307;
    const TOO: f32 = 6.283_185_307;

    let too64: f64 = 0.123_456_789_012_345_6;
    const TOO64: f64 = 0.123_456_789_012_345_6;
}
