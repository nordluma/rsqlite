use std::borrow::Cow;

use crate::{
    page::Page,
    pager::{read_varint_at, Pager},
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub enum RecordFieldType {
    Null,
    I8,
    I16,
    I24,
    I32,
    I48,
    I64,
    Float,
    Zero,
    One,
    String(usize),
    Blob(usize),
}

#[derive(Debug, Clone)]
pub struct RecordField {
    pub offset: usize,
    pub field_type: RecordFieldType,
}

#[derive(Debug, Clone)]
pub struct RecordHeader {
    pub fields: Vec<RecordField>,
}

#[derive(Debug)]
pub struct Cursor<'p> {
    header: RecordHeader,
    pager: &'p mut Pager,
    page_index: usize,
    page_cell: usize,
}

impl<'p> Cursor<'p> {
    pub fn field(&mut self, n: usize) -> Option<Value> {
        let record_field = self.header.fields.get(n)?;

        let payload = match self.pager.read_page(self.page_index) {
            Ok(Page::TableLeaf(leaf)) => &leaf.cells[self.page_cell].payload,
            _ => return None,
        };

        match record_field.field_type {
            RecordFieldType::Null => Some(Value::Null),
            RecordFieldType::I8 => Some(Value::Int(read_i8_at(payload, record_field.offset))),
            RecordFieldType::I16 => Some(Value::Int(read_i16_at(payload, record_field.offset))),
            RecordFieldType::I24 => Some(Value::Int(read_i24_at(payload, record_field.offset))),
            RecordFieldType::I32 => Some(Value::Int(read_i32_at(payload, record_field.offset))),
            RecordFieldType::I48 => Some(Value::Int(read_i48_at(payload, record_field.offset))),
            RecordFieldType::I64 => Some(Value::Int(read_i64_at(payload, record_field.offset))),
            RecordFieldType::Float => Some(Value::Float(read_f64_at(payload, record_field.offset))),
            RecordFieldType::String(length) => {
                let value = std::str::from_utf8(
                    &payload[record_field.offset..record_field.offset + length],
                )
                .expect("invalid utf8");

                Some(Value::String(Cow::Borrowed(value)))
            }
            RecordFieldType::Blob(length) => {
                let value = &payload[record_field.offset..record_field.offset + length];

                Some(Value::Blob(Cow::Borrowed(value)))
            }
            _ => unimplemented!(),
        }
    }
}

fn parse_record_header(mut buffer: &[u8]) -> Result<RecordHeader, anyhow::Error> {
    let (varint_size, header_length) = read_varint_at(buffer, 0);
    buffer = &buffer[varint_size as usize..header_length as usize];

    let mut fields = Vec::new();
    let mut current_offset = header_length as usize;

    while !buffer.is_empty() {
        let (discriminant_size, discriminant) = read_varint_at(buffer, 0);
        buffer = &buffer[discriminant_size as usize..];

        let (field_type, field_size) = match discriminant {
            0 => (RecordFieldType::Null, 0),
            1 => (RecordFieldType::I8, 1),
            2 => (RecordFieldType::I16, 2),
            3 => (RecordFieldType::I24, 3),
            4 => (RecordFieldType::I32, 4),
            5 => (RecordFieldType::I48, 6),
            6 => (RecordFieldType::I64, 8),
            7 => (RecordFieldType::Float, 8),
            8 => (RecordFieldType::Zero, 0),
            9 => (RecordFieldType::One, 0),
            n if n >= 12 && n % 2 == 0 => {
                let size = ((n - 12) / 2) as usize;
                (RecordFieldType::Blob(size), size)
            }
            n if n >= 13 && n % 2 == 1 => {
                let size = ((n - 13) / 2) as usize;
                (RecordFieldType::String(size), size)
            }
            n => anyhow::bail!("unsupported field type: {n}"),
        };

        fields.push(RecordField {
            offset: current_offset,
            field_type,
        });

        current_offset += field_size;
    }

    Ok(RecordHeader { fields })
}

fn read_i8_at(input: &[u8], offset: usize) -> i64 {
    input[offset] as i64
}

fn read_i16_at(input: &[u8], offset: usize) -> i64 {
    i16::from_be_bytes(input[offset..offset + 2].try_into().unwrap()) as i64
}

fn read_i24_at(input: &[u8], offset: usize) -> i64 {
    (i32::from_be_bytes(input[offset..offset + 3].try_into().unwrap()) & 0x00FFFFFF) as i64
}

fn read_i32_at(input: &[u8], offset: usize) -> i64 {
    i32::from_be_bytes(input[offset..offset + 4].try_into().unwrap()) as i64
}

fn read_i48_at(input: &[u8], offset: usize) -> i64 {
    i64::from_be_bytes(input[offset..offset + 6].try_into().unwrap()) & 0x0000FFFFFFFFFFFF
}

fn read_i64_at(input: &[u8], offset: usize) -> i64 {
    i64::from_be_bytes(input[offset..offset + 8].try_into().unwrap())
}

fn read_f64_at(input: &[u8], offset: usize) -> f64 {
    f64::from_be_bytes(input[offset..offset + 8].try_into().unwrap())
}
