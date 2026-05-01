use super::*;

pub(crate) fn load_bytes(path: &path) -> IoResult<Vec<u8>>
{
    Err(IoError::new(path, FileError::NotSupported).when_reading())
}

pub(crate) fn save_bytes(path: &path, bytes: &[u8]) -> IoResult
{
    Err(IoError::new(path, FileError::NotSupported).when_writing())
}

/*
use std::{cell::RefCell, collections::HashMap, ffi::CString};


pub(crate) fn load_bytes(path: &path) -> IoResult<Vec<u8>>
{
    Err(IoError::new(path, FileError::NotSupported).when_reading())
}




thread_local! {
    #[allow(clippy::type_complexity)]
    static FILES: RefCell<HashMap<u32, (String, Box<dyn FnOnce(IoResult<Vec<u8>>)>)>> = RefCell::new(HashMap::new());
}

pub(crate) fn load_bytes_async<F>(path: &path, on_loaded: F)
    where F: FnOnce(IoResult<Vec<u8>>) + 'static
{
    let url = CString::new(path).unwrap();
    let file_id = unsafe { fs_load_bytes(url.as_ptr(), url.as_bytes().len() as u32) };
    FILES.with(|files| {
        let mut files = files.borrow_mut();
        files.insert(file_id, (path.to_owned(), Box::new(on_loaded)));
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn file_loaded(file_id: u32) {

    FILES.with(|files| {
        let mut files = files.borrow_mut();
        let (path, callback) = files
            .remove(&file_id)
            .unwrap_or_else(|| panic!("Unknown file loaded!"));
        let file_len = unsafe { fs_get_buffer_size(file_id) };
        if file_len == -1 {
            callback(Err(IoError::new(path, FileError::DownloadFailed).when_reading()));
        } else {
            let mut buffer = vec![0; file_len as usize];
            unsafe { fs::fs_take_buffer(file_id, buffer.as_mut_ptr(), file_len as u32) };
            callback(Ok(buffer));
        }
    })
}

unsafe extern "C" {
    pub fn fs_load_bytes(ptr: *const i8, len: u32) -> u32;
    pub fn fs_get_buffer_size(file_id: u32) -> i32;
    pub fn fs_take_buffer(file_id: u32, ptr: *mut u8, max_size: u32);
}




pub(crate) fn save_bytes(path: &path, bytes: &[u8]) -> IoResult
{
    Err(IoError::new(path, FileError::NotSupported).when_writing())
}

pub(crate) fn save_bytes_async<F>(path: &path, bytes: Vec<u8>, on_saved: F)
    where F: FnOnce(IoResult) + 'static
{
    on_saved(Err(IoError::new(path, FileError::NotSupported).when_writing()))
}
*/
