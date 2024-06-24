use std::io::{Cursor, Read, Write};

use anyhow::{bail, ensure, Result};
use clap::Parser;
use cli::Cli;
use status_response::StatusResponse;

mod cli;
mod status_response;

fn main() -> Result<()> {
    Cli::parse().run()
}

fn write_handshake<W: Write>(
    w: &mut W,
    protocol_version: i32,
    host: &str,
    port: u16,
    next_state: i32,
) -> Result<()> {
    write_packet(w, 0x00, |w| {
        write_var_int(w, protocol_version)?;
        write_string(w, host)?;
        write_unsigned_short(w, port)?;
        write_var_int(w, next_state)?;
        Ok(())
    })
}

fn write_status_request<W: Write>(w: &mut W) -> Result<()> {
    write_packet(w, 0x00, |_| Ok(()))
}

fn read_status_response<R: Read>(r: &mut R) -> Result<StatusResponse> {
    read_packet(r, |id, r| {
        ensure!(id == 0x00);

        let status_response = read_string(r)?;
        let status_response = serde_json::from_str::<StatusResponse>(&status_response)?;

        Ok(status_response)
    })
}

fn write_packet<W, F>(w: &mut W, id: i32, f: F) -> Result<()>
where
    W: Write,
    F: FnOnce(&mut dyn Write) -> Result<()>,
{
    let buf = &mut Vec::new();

    write_var_int(buf, id)?;
    f(buf)?;

    write_var_int(w, buf.len() as i32)?;
    w.write_all(buf)?;

    Ok(())
}

fn read_packet<R, F, T>(r: &mut R, f: F) -> Result<T>
where
    R: Read,
    F: FnOnce(i32, &mut dyn Read) -> Result<T>,
{
    let len = read_var_int(r)? as usize;
    let mut data = vec![0; len];
    r.read_exact(&mut data)?;
    let mut data = Cursor::new(data);

    let packet_id = read_var_int(&mut data)?;
    let value = f(packet_id, &mut data)?;

    Ok(value)
}

fn write_unsigned_short<W: Write + ?Sized>(w: &mut W, value: u16) -> Result<()> {
    w.write_all(&value.to_be_bytes())?;
    Ok(())
}

fn write_string<W: Write + ?Sized>(w: &mut W, value: &str) -> Result<()> {
    write_var_int(w, value.len() as i32)?;
    w.write_all(value.as_bytes())?;
    Ok(())
}

fn read_string<R: Read + ?Sized>(r: &mut R) -> Result<String> {
    let len = read_var_int(r)? as usize;
    let mut buf = vec![0; len];

    r.read_exact(&mut buf)?;

    let value = String::from_utf8(buf)?;

    Ok(value)
}

const SEGMENT_BITS: u32 = 0x7F;
const CONTINUE_BIT: u32 = 0x80;

fn write_var_int<W: Write + ?Sized>(w: &mut W, value: i32) -> Result<()> {
    let mut value = value as u32;

    loop {
        if (value & !SEGMENT_BITS) == 0 {
            w.write_all(&[value as u8])?;
            return Ok(());
        }

        let segment = (value & SEGMENT_BITS) | CONTINUE_BIT;

        w.write_all(&[segment as u8])?;

        value >>= 7;
    }
}

fn read_byte<R: Read + ?Sized>(r: &mut R) -> Result<u8> {
    let buf = &mut [0];
    r.read_exact(buf)?;
    Ok(buf[0])
}

fn read_var_int<R: Read + ?Sized>(r: &mut R) -> Result<i32> {
    let mut value: u32 = 0;
    let mut position = 0;

    loop {
        let current_byte = read_byte(r)? as u32;

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
