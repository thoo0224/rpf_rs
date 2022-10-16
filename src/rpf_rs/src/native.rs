use std::sync::Arc;

use crate::archive::{Archive, Header};

#[repr(C, packed)]
pub struct NativeHeader {
    magic: u32,
    entry_count: u32,
    names_length: u32,
    encryption_type: u32
}

impl From<Header> for NativeHeader {
    fn from(header: Header) -> Self {
        Self {
            magic: header.magic,
            entry_count: header.entry_count,
            names_length: header.names_length,
            encryption_type: header.encryption_type
        }
    }
}

#[repr(C)]
pub enum NativeError {
    None = 0
}

#[no_mangle]
pub unsafe extern "C" fn create_archive<'a>(ptr: *const u8, len: usize) -> *const Archive<&'a [u8]> {
    let data = std::slice::from_raw_parts(ptr, len);
    let archive = Archive::from(data);

    Arc::into_raw(Arc::new(archive)) as *const Archive<&'a [u8]>
}

#[no_mangle]
pub unsafe extern "C" fn free_archive<'a>(archive: *const Archive<&'a [u8]>) {
    let _archive = Arc::from_raw(archive);
}

// todo: erorr handling
#[no_mangle]
pub unsafe extern "C" fn read_header<'a>(archive: *mut Archive<&'a [u8]>, header: &mut NativeHeader) -> NativeError {
    let archive = &mut *archive;
    let parsed_header = archive.read_header().unwrap();

    *header = NativeHeader::from(parsed_header);

    NativeError::None
}