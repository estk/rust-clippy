#![feature(plugin, custom_attribute)]
#![warn(excessive_precision)]

fn main() {
    // TODO add prefix tests
    // Consts
    const GOOD32_SUF: f32 = 0.123_456_f32;
    const GOOD32: f32 = 0.123_456;
    const GOOD64: f64 = 0.123_456_789_012;

    const BAD32_SUF: f32 = 0.123_456_789_f32;
    const BAD32: f32 = 0.123_456_789;
    const BAD64: f64 = 0.123_456_789_012_345_67;
    const BAD64_SUF: f64 = 0.123_456_789_012_345_67f64;

    // const BAD64_32_SUF: f32 = 0.123_456_789_012_345_6f64;
    // Literal
    println!(9.999_999_999_999_999_999_999_999_999);

    // TODO add inferred type tests
    // Locals
    let good32_suf: f32 = 0.123_456_f32;
    let good32: f32 = 0.123_456;
    let good64: f64 = 0.123_456_789_012;

    let bad32_suf: f32 = 1.123_456_789_f32;
    let bad32: f32 = 1.123_456_789;
    let bad64: f64 = 0.123_456_789_012_345_67;
    let bad64_suf: f64 = 0.123_456_789_012_345_67f64;

    // TODO Vectors / nested vectors
    let recurse: Vec<f32> = vec![0.123_456_789];

    // Exponential float notation
    let good_exp32: f32 = 1e-10;
    let bad_exp32: f32 = 1.123456789e-10;
}
