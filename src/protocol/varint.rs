use super::read_byte;
use anyhow::{bail, Result};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

const SEGMENT_BITS: u32 = 0x7F;
const CONTINUE_BIT: u32 = 0x80;

pub async fn read<R: AsyncRead + ?Sized + Unpin>(r: &mut R) -> Result<i32> {
    let mut value: u32 = 0;
    let mut position = 0;

    loop {
        let current_byte = read_byte(r).await? as u32;

        value |= (current_byte & SEGMENT_BITS) << position;

        if (current_byte & CONTINUE_BIT) == 0 {
            break;
        }

        position += 7;

        if position >= 32 {
            bail!("VarInt is too big");
        }
    }

    Ok(value as i32)
}

pub async fn write<W: AsyncWrite + ?Sized + Unpin>(w: &mut W, value: i32) -> Result<()> {
    let mut value = value as u32;

    loop {
        if (value & !SEGMENT_BITS) == 0 {
            w.write_all(&[value as u8]).await?;
            return Ok(());
        }

        let segment = (value & SEGMENT_BITS) | CONTINUE_BIT;

        w.write_all(&[segment as u8]).await?;

        value >>= 7;
    }
}
