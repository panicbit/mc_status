use std::io::Cursor;

use anyhow::{ensure, Result};
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub mod status_response;
pub mod varint;

pub use status_response::StatusResponse;

pub async fn write_handshake<W: AsyncWrite + ?Sized + Unpin>(
    w: &mut W,
    protocol_version: i32,
    host: &str,
    port: u16,
    next_state: i32,
) -> Result<()> {
    write_packet(w, 0x00, |w| {
        varint::write(w, protocol_version)?;
        write_string(w, host)?;
        write_unsigned_short(w, port)?;
        varint::write(w, next_state)?;
        Ok(())
    })
    .await
}

pub async fn write_status_request<W: AsyncWrite + ?Sized + Unpin>(w: &mut W) -> Result<()> {
    write_packet(w, 0x00, |_| Ok(())).await
}

pub async fn read_status_response<R: AsyncRead + ?Sized + Unpin>(
    r: &mut R,
) -> Result<StatusResponse> {
    read_packet(r, |id, r| {
        ensure!(id == 0x00);

        let status_response = read_string(r)?;
        let status_response = serde_json::from_str::<StatusResponse>(&status_response)?;

        Ok(status_response)
    })
    .await
}

pub async fn write_packet<W, F>(w: &mut W, id: i32, f: F) -> Result<()>
where
    W: AsyncWrite + ?Sized + Unpin,
    F: FnOnce(&mut dyn Write) -> Result<()>,
{
    let buf = &mut Vec::new();

    varint::write(buf, id)?;
    f(buf)?;

    varint::write_async(w, buf.len() as i32).await?;
    w.write_all(buf).await?;

    Ok(())
}

pub async fn read_packet<R, F, T>(r: &mut R, f: F) -> Result<T>
where
    R: AsyncRead + ?Sized + Unpin,
    F: FnOnce(i32, &mut dyn Read) -> Result<T>,
{
    let len = varint::read_async(r).await? as usize;

    let mut data = vec![0; len];
    r.read_exact(&mut data).await?;
    let mut data = Cursor::new(data);

    let packet_id = varint::read(&mut data)?;
    let value = f(packet_id, &mut data)?;

    Ok(value)
}

pub fn write_unsigned_short<W: Write + ?Sized>(w: &mut W, value: u16) -> Result<()> {
    w.write_all(&value.to_be_bytes())?;
    Ok(())
}

pub fn write_string<W: Write + ?Sized>(w: &mut W, value: &str) -> Result<()> {
    varint::write(w, value.len() as i32)?;
    w.write_all(value.as_bytes())?;
    Ok(())
}

pub fn read_string<R: Read + ?Sized>(r: &mut R) -> Result<String> {
    let len = varint::read(r)? as usize;
    let mut buf = vec![0; len];

    r.read_exact(&mut buf)?;

    let value = String::from_utf8(buf)?;

    Ok(value)
}

pub fn read_byte<R: Read + ?Sized>(r: &mut R) -> Result<u8> {
    let buf = &mut [0];
    r.read_exact(buf)?;
    Ok(buf[0])
}

pub async fn read_byte_async<R: AsyncRead + ?Sized + Unpin>(r: &mut R) -> Result<u8> {
    let buf = &mut [0];
    r.read_exact(buf).await?;
    Ok(buf[0])
}
