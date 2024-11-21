use crate::page;

pub const HEADER_SIZE: usize = 100;
const HEADER_PREFIX: &[u8] = b"SQLITE format 3\0";
const HEADER_PAGE_SIZE_OFFSET: usize = 16;

const PAGE_MAX_SIZE: u32 = 65536;

pub fn parse_header(buffer: &[u8]) -> Result<page::DbHeader, anyhow::Error> {
    if !buffer.starts_with(HEADER_PREFIX) {
        let prefix = String::from_utf8_lossy(&buffer[..HEADER_PREFIX.len()]);
        anyhow::bail!("invalid header prefix: {prefix}");
    }

    let page_size_raw = read_be_word_at(buffer, HEADER_PAGE_SIZE_OFFSET);
    let page_size = match page_size_raw {
        1 => PAGE_MAX_SIZE,
        n if n.is_power_of_two() => n as u32,
        _ => anyhow::bail!("page size is not a power of 2: {page_size_raw}"),
    };

    Ok(page::DbHeader { page_size })
}

fn read_be_word_at(input: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes(input[offset..offset + 2].try_into().unwrap())
}
