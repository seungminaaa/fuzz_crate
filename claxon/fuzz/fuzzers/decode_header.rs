// Claxon -- A FLAC decoding library in Rust
// Copyright 2017 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.



extern crate afl;
extern crate claxon;

use std::io;
use std::io::Read;
use std::io::Cursor;

fn main() {
    let mut input = vec![];
    let data = io::stdin().read_to_end(&mut input);

    let cursor = Cursor::new(&input);
    match claxon::FlacReader::new(cursor) {
        Ok(..) => {}
        Err(..) => {}
    };
}

