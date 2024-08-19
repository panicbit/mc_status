use std::io::Cursor;

use anyhow::{ensure, Result};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub mod status_response;
pub mod varint;

pub use status_response::StatusResponse;

pub async fn write_handshake<W: AsyncWrite + Unpin>(
    w: &mut W,
    protocol_version: i32,
    host: &str,
    port: u16,
    next_state: i32,
) -> Result<()> {
    write_packet(w, 0x00, async |w| {
        varint::write(w, protocol_version).await?;
        write_string(w, host).await?;
        write_unsigned_short(w, port).await?;
        varint::write(w, next_state).await?;
        Ok(())
    })
    .await
}

pub async fn write_status_request<W: AsyncWrite + Unpin>(w: &mut W) -> Result<()> {
    write_packet(w, 0x00, async |_| Ok(())).await
}

pub async fn read_status_response<R: AsyncRead + Unpin>(r: &mut R) -> Result<StatusResponse> {
    read_packet(r, async |id, r| {
        ensure!(id == 0x00);

        let status_response = read_string(r).await?;
        let status_response = serde_json::from_str::<StatusResponse>(&status_response)?;

        Ok(status_response)
    })
    .await
}

pub async fn write_packet<W, F>(w: &mut W, id: i32, f: F) -> Result<()>
where
    W: AsyncWrite + Unpin,
    F: async FnOnce(&mut Vec<u8>) -> Result<()>,
{
    let buf = &mut Vec::new();

    varint::write(buf, id).await?;
    f(buf).await?;

    varint::write(w, buf.len() as i32).await?;
    w.write_all(buf).await?;

    Ok(())
}

pub async fn read_packet<R, F, T>(r: &mut R, f: F) -> Result<T>
where
    R: AsyncRead + Unpin,
    F: async FnOnce(i32, &mut (dyn AsyncRead + Unpin)) -> Result<T>,
{
    let len = varint::read(r).await? as usize;
    let mut data = vec![0; len];
    r.read_exact(&mut data).await?;
    let mut data = Cursor::new(data);

    let packet_id = varint::read(&mut data).await?;
    let value = f(packet_id, &mut data).await?;

    Ok(value)
}

pub async fn write_unsigned_short<W: AsyncWrite + ?Sized + Unpin>(
    w: &mut W,
    value: u16,
) -> Result<()> {
    w.write_all(&value.to_be_bytes()).await?;
    Ok(())
}

pub async fn write_string<W: AsyncWrite + ?Sized + Unpin>(w: &mut W, value: &str) -> Result<()> {
    varint::write(w, value.len() as i32).await?;
    w.write_all(value.as_bytes()).await?;
    Ok(())
}

pub async fn read_string<R: AsyncRead + ?Sized + Unpin>(r: &mut R) -> Result<String> {
    let len = varint::read(r).await? as usize;
    let mut buf = vec![0; len];

    r.read_exact(&mut buf).await?;

    let value = String::from_utf8(buf)?;

    Ok(value)
}

pub async fn read_byte<R: AsyncRead + ?Sized + Unpin>(r: &mut R) -> Result<u8> {
    let buf = &mut [0];
    r.read_exact(buf).await?;
    Ok(buf[0])
}
