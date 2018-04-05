// #![feature(plugin)]
// #![plugin(clippy)]
#![allow(unused_must_use)]
// #![warn(write_literal)]
use std::io::Write;

fn main() {
    let mut v = Vec::new();
    // // these should be fine
    write!(&mut v, "Hello");
    writeln!(&mut v, "Hello");
    let world = "world";
    writeln!(&mut v, "Hello {}", world);
    writeln!(&mut v, "3 in hex is {:X}", 3);

    // these should throw warnings
    write!(&mut v, "Hello {}", "world");
    writeln!(&mut v, "Hello {} {}", world, "world");
    writeln!(&mut v, "Hello {}", "world");
    writeln!(&mut v, "10 / 4 is {}", 2.5);
    writeln!(&mut v, "2 + 1 = {}", 3);
    // writeln!(&mut v, "2 + 1 = {:.4}", 3);
    // writeln!(&mut v, "2 + 1 = {:5.4}", 3);
    // writeln!(&mut v, "Debug test {:?}", "hello, world");

    // positional args don't change the fact
    // that we're using a literal -- this should
    // throw a warning
    // writeln!("{0} {1}", "hello", "world");
    // writeln!("{1} {0}", "hello", "world");

    // named args shouldn't change anything either
    // writeln!("{foo} {bar}", foo = "hello", bar = "world");
    // writeln!("{bar} {foo}", foo = "hello", bar = "world");
}
