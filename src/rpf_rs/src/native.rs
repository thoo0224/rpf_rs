use std::sync::Arc;

use crate::archive::Archive;

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