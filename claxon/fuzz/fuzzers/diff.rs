// Claxon -- A FLAC decoding library in Rust
// Copyright 2018 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

extern crate afl;
extern crate claxon;

use std::io;
use std::io::Read;
use std::io::Seek;
use std::iter;

fn main(){
let mut input = vec![];
let data = io::stdin().read_to_end(&mut input);

    // We want two distinct marker bytes.
    if input.len() < 2 || input[0] == input[1] {
        return
    }

    let mut cursor = io::Cursor::new(&input[2..]);

    // Allocate two large buffers filled with a marker byte. We will decode one
    // block twice, once into each buffer. If both decodes are successful, then
    // the two outputs should be identical. If we don't overwrite parts of the
    // buffer, then we would see a difference in the marker byte. The buffer
    // allocated up front should be large enough that Claxon does not need to
    // allocate a new one, but small enough to keep the fuzzer fast.
    let buffer0: Vec<i32> = iter::repeat(input[0] as i32).take(1024 * 8).collect();
    let buffer1: Vec<i32> = iter::repeat(input[1] as i32).take(1024 * 8).collect();

    // NOTE: Somewhat surprisingly, the above runs at about 4x the execs per
    // second of this version below. I should file a performance bug against Rust.
    // let mut buffer0: Vec<i32> = Vec::with_capacity(1024 * 8);
    // let mut buffer1: Vec<i32> = Vec::with_capacity(1024 * 8);
    // buffer0.resize(1024 * 16, data[0] as i32);
    // buffer1.resize(1024 * 16, data[1] as i32);

    let result0 = {
        let mut reader = match claxon::FlacReader::new(&mut cursor) {
            Ok(r) => r,
            Err(..) => return,
        };

        match reader.blocks().read_next_or_eof(buffer0) {
            Ok(Some(block)) => Some(block.into_buffer()),
            _ => None,
        }
    };

    if let Some(ref res) = result0 {
        if res.capacity() != 1024 * 8 {
            // If a resize was needed, there is no point in attempting the
            // second decode, because Claxon allocated a new buffer that was not
            // filled with the marker byte.
            return
        }
    }

    cursor.seek(io::SeekFrom::Start(0)).unwrap();

    let result1 = {
        let mut reader = match claxon::FlacReader::new(&mut cursor) {
            Ok(r) => r,
            Err(..) => panic!("First time decoded fine, second time should too."),
        };

        match reader.blocks().read_next_or_eof(buffer1) {
            Ok(Some(block)) => Some(block.into_buffer()),
            _ => None,
        }
    };

    assert_eq!(result0, result1);
}
