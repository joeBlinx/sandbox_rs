use std::ffi::CString;
use std::os::raw::c_void;

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
pub fn draw_elements(number_indices: usize) {
    unsafe {
        gl::DrawElements(
            gl::TRIANGLES,         // mode
            number_indices as i32, // count
            gl::UNSIGNED_SHORT,    // type
            0 as *const c_void,    // element array buffer offset
        );
    };
}
