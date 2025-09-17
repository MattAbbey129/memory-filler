// SPDX-License-Identifier: MIT
// Copyright © 2025 Matt Abbey

use std::io::{stdout, Stdout, Write};

use anyhow::{Context, Result};
use byte_unit::{AdjustedByte, Byte, UnitType};
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

    const NULL_BYTE: u8 = 0;

    let mut stdout: Stdout = stdout();

    init_cursor(&mut stdout)
        .with_context(|| "Unable to initialize terminal cursor")?;

    loop {
        /*
            Avoid needlessly computing 'buffer.len()' multiple
            times here when we know it will be the same number.
        */
        let buffer_length: usize = buffer.len();

        /*
            Allocate more memory to the buffer and print buffer statistics
            when we have written the amount of bytes in CLUSTER_SIZE since
            the last allocation or if there is nothing in the buffer yet.
        */
        if buffer_length % CLUSTER_SIZE == 0 {
            /*
                We manually allocate an exact additional amount of memory to the buffer
                instead of letting Rust speculatively allocate additional memory for us.
                Since we know what we need, we will take care of it instead, since Rust
                may not pick up the pattern of allocation for the buffer.
            */
            buffer
                .try_reserve_exact(CLUSTER_SIZE)
                .with_context(|| format!("Unable to allocate {CLUSTER_SIZE} bytes in memory! Did we run out of memory?"))?;

            let buffer_length_approximate_unit: AdjustedByte = Byte::from_u64(buffer_length as u64)
                .get_appropriate_unit(UnitType::Binary);

            print_buffer_statistics(
                &mut stdout,
                buffer_length,
                buffer_length_approximate_unit,
            )
                .with_context(|| "Unable to print buffer statistics")?;
        }

        buffer.push(NULL_BYTE);
    }

}

/// Initialize terminal cursor position so we
/// know where to print the buffer statistics.
fn init_cursor(stdout: &mut Stdout) -> Result<()> {
    stdout
        .queue(cursor::MoveToColumn(0))
        .with_context(|| "Unable to move cursor to first column")?;
    stdout
        .queue(cursor::SavePosition)
        .with_context(|| "Unable to save cursor position")?;
    stdout
        .flush()
        .with_context(|| "Unable to execute terminal instructions for initializing the terminal cursor")?;

    Ok(())
}

fn print_buffer_statistics(
    stdout: &mut Stdout,
    buffer_length: usize,
    buffer_length_approximate_unit: AdjustedByte,
) -> Result<()> {
    stdout
        .queue(cursor::RestorePosition)
        .with_context(|| "Unable to restore cursor position")?;
    stdout
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
        .with_context(|| "Unable to clear terminal from cursor down")?;
    stdout
        .queue(Print(format!("Buffer: {:.2} ({} bytes)",
            buffer_length_approximate_unit,
            buffer_length,
        )))
        .with_context(|| "Unable to display buffer information to stdout")?;
    stdout
        .flush()
        .with_context(|| "Unable to execute terminal instructions for displaying buffer information")?;

    Ok(())
}
