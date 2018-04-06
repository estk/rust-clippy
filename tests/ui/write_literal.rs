#![allow(unused_must_use)]
#![warn(write_literal)]

use std::io::Write;

fn main() {
    let mut v = Vec::new();

    // These should be fine
    write!(&mut v, "Hello");
    writeln!(&mut v, "Hello");
    let world = "world";
    writeln!(&mut v, "Hello {}", world);
    writeln!(&mut v, "3 in hex is {:X}", 3);

    // These should throw warnings
    write!(&mut v, "Hello {}", "world");
    writeln!(&mut v, "Hello {} {}", world, "world");
    writeln!(&mut v, "Hello {}", "world");
    writeln!(&mut v, "10 / 4 is {}", 2.5);
    writeln!(&mut v, "2 + 1 = {}", 3);
}
