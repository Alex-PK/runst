#!/usr/bin/env runst

use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if !args.is_empty() {
        println!("Hello {}", args.join(" "))
    } else {
        println!("Hello world");
    }
}
