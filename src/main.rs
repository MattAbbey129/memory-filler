// SPDX-License-Identifier: MIT
// Copyright © 2025 Matt Abbey

use anyhow::{Context, Result};

fn main() -> Result<()> {

    let mut buffer: Vec<u8> = Vec::new();

    /*
        I avoid setting magic numbers by making
        the intent of this specific number clear.
    */
    const CLUSTER_SIZE: usize = {
        const ONE_MEGABYTE: usize = 1_048_576;
        ONE_MEGABYTE
    };

    loop {
        if buffer.len() % CLUSTER_SIZE == 0 {
            println!("Buffer: {} bytes", buffer.len());
        }

        buffer.push(0);
    }

}
