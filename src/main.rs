// SPDX-License-Identifier: MIT
// Copyright © 2025 Matt Abbey

use std::io::{stdout, Stdout, Write};

use anyhow::{Context, Result};
use crossterm::{cursor, style::Print, terminal, QueueableCommand};

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

    let mut stdout: Stdout = stdout();

    stdout
        .queue(cursor::MoveToColumn(0))
        .with_context(|| "Unable to move cursor to first column")?;
    stdout
        .queue(cursor::SavePosition)
        .with_context(|| "Unable to save cursor position")?;
    stdout
        .flush()
        .with_context(|| "Unable to execute terminal instructions for initializing the terminal cursor")?;

    loop {
        /*
            Avoid needlessly computing 'buffer.len()' multiple
            times here when we know it will be the same number.
        */
        let buffer_length: usize = buffer.len();

        /*
            Allocate more memory in the vector when we've written the
            amount of bytes in CLUSTER_SIZE since the last allocation.
        */
        if buffer_length % CLUSTER_SIZE == 0 {
            stdout
                .queue(cursor::RestorePosition)
                .with_context(|| "Unable to restore cursor position")?;
            stdout
                .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
                .with_context(|| "Unable to clear terminal from cursor down")?;

            buffer
                .try_reserve_exact(CLUSTER_SIZE)
                .with_context(|| format!("Unable to allocate {CLUSTER_SIZE} bytes in memory! Did we ran out of memory?"))?;

            stdout
                .queue(Print(format!("Buffer: {} bytes", buffer_length)))
                .with_context(|| "Unable to display buffer information to stdout")?;
            stdout
                .flush()
                .with_context(|| "Unable to execute terminal instructions for displaying buffer information")?;
        }

        buffer.push(0);
    }

}
