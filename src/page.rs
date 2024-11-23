#[derive(Debug, Clone, Copy)]
pub struct DbHeader {
    pub page_size: u32,
}

#[derive(Debug, Clone)]
pub enum Page {
    TableLeaf(TableLeafPage),
}

#[derive(Debug, Clone)]
pub struct TableLeafPage {
    pub header: PageHeader,
    pub cell_pointers: Vec<u16>,
    pub cells: Vec<TableLeafCell>,
}

#[derive(Debug, Clone, Copy)]
pub struct PageHeader {
    pub page_type: PageType,
    pub first_freeblock: u16,
    pub cell_count: u16,
    pub cell_content_offset: u32,
    pub fragmented_bytes_count: u8,
    pub rightmost_pointer: Option<u32>,
}

impl PageHeader {
    pub fn byte_size(&self) -> usize {
        if self.rightmost_pointer.is_some() {
            12
        } else {
            8
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PageType {
    TableLeaf,
    TableInterior,
}

#[derive(Debug, Clone)]
pub struct TableLeafCell {
    pub size: i64,
    pub row_id: i64,
    pub payload: Vec<u8>,
}
