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
        /*
            Allocate more memory in the vector when we've written the
            amount of bytes in CLUSTER_SIZE since the last allocation.
        */
        if buffer.len() % CLUSTER_SIZE == 0 {
            buffer
                .try_reserve_exact(CLUSTER_SIZE)
                .with_context(|| format!("Unable to allocate {CLUSTER_SIZE} bytes in memory! Did we ran out of memory?"))?;

            println!("Buffer: {} bytes", buffer.len());
        }

        buffer.push(0);
    }

}
