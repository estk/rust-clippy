#![feature(plugin, custom_attribute)]
#![warn(excessive_precision)]

fn main() {
    // Consts
    const GOOD32_SUF: f32 = 0.123_456_f32;
    const GOOD32: f32 = 0.123_456;
    const GOOD64: f64 = 0.123_456_789_012;

    const BAD32_SUF: f32 = 1.123_456_789_f32;
    const BAD32: f32 = 1.123_456_789;
    const BAD64: f64 = 0.123_456_789_012_345_6;
    const BAD64_SUF: f64 = 0.123_456_789_012_345_6f64;

    // const BAD64_32_SUF: f32 = 0.123_456_789_012_345_6f64;
    // Literal
    // println!(9.999_999_999_999_999_999_999_999_999);

    // let too_suf: f32 = 6.283_185_307_f32;

    // let too: f32 = 6.283_185_307;

    // let too64: f64 = 0.123_456_789_012_345_6;
}
