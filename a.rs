#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! strsim = "0.11.0"
//! ```

use strsim::jaro;

fn main() {
    let jaro = jaro("ovo", "ovo");
    println!("{}", jaro);
}