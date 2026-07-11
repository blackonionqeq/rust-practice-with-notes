use std::ffi::{CStr, CString};

fn main() {
    let cstring = CString::new("Hello, world!").expect("CString::new failed");
    // CString to CStr: use as_c_str
    let cstr: &CStr = cstring.as_c_str();

    // cstr to str: use to_str
    match cstr.to_str() {
        Ok(s) => println!("{s}"),
        Err(e) => println!("Error: {}", e),
    }

    // cstring字符串内不能带\0，否则会报错
    match CString::new("hel\0lo") {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}
