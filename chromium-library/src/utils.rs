use cef;
use std::os::raw::{c_char};

pub fn subp_path(cwd: &::std::path::Path) -> String {
    let subp_path = if cfg!(target_os = "windows") { 
        cwd.join("chromium_subp.exe")
    } else if cfg!(target_os = "macos") {
        cwd.join("chromium_subp.app/Contents/MacOS/chromium_subp")
    } else { 
        cwd.join("chromium_subp") 
    };
    let subp = subp_path.to_str().unwrap();
    println!("subp: {:?}", subp);
    String::from(subp)
}

#[cfg(unix)]
pub fn prepare_args() -> cef::_cef_main_args_t {
    use std::ffi;
    let mut args: Vec<*mut c_char> = ::std::env::args().map(|arg| {
        // println!("arg: {:?}", arg);
        let carg_rslt = ffi::CString::new(arg);
        let carg = carg_rslt.expect("cant create arg");
        let mp = carg.into_raw();
        mp
    }).collect();
    if cfg!(target_os = "macos") {
        let carg_rslt = ffi::CString::new("--disable-gpu-compositing");
        let carg = carg_rslt.expect("cant create arg");
        let mp = carg.into_raw();
        args.push(mp);
        let carg_rslt = ffi::CString::new("--disable-accelerated-2d-canvas");
        let carg = carg_rslt.expect("cant create arg");
        let mp = carg.into_raw();
        args.push(mp);
        // println!("Force --disable-gpu-compositing");
    }

    let args_size = args.len() as i32;
    let args_ptr = args.as_mut_ptr();
    ::std::mem::forget(args);

    let main_args = cef::_cef_main_args_t {
        argc : args_size,
        argv : args_ptr
    };
    // println!("Hello CEF, ARGS: {:?}", main_args.argc);

    main_args
}

#[cfg(windows)]
pub fn prepare_args() -> cef::_cef_main_args_t {
    let h_instance = unsafe { winapi::um::libloaderapi::GetModuleHandleA(0 as winapi::um::winnt::LPCSTR) };
    let main_args = cef::_cef_main_args_t {
        instance: unsafe { std::mem::transmute(h_instance) }
        //instance: unsafe { std::mem::transmute(0 as i64) }
    };
    // println!("Hello CEF, hinstance: {:?}", main_args.instance);
    main_args
}

pub fn cef_string(value: &str) -> cef::cef_string_t {
    let mut str_cef = cef::cef_string_t {str: ::std::ptr::null_mut(), length: 0, dtor: Option::Some(dtr)};
    //unsafe { cef::cef_string_utf16_set(value.as_ptr() as *mut cef::char16, value.len(), &mut str_cef, 1) };
    unsafe {cef::cef_string_utf8_to_utf16(value.as_ptr() as *mut c_char, value.len(), &mut str_cef);}
    str_cef
}

pub fn cef_string_empty() -> cef::cef_string_t {
    let mut empty_str = cef::cef_string_t {
        str: ::std::ptr::null_mut(), 
        length: 0, 
        dtor: Option::Some(dtr)
    };
    
    let emp = "";
    //unsafe { cef::cef_string_utf16_set(emp.as_ptr() as *mut cef::char16, 0, &mut empty_str, 1) };
    unsafe { cef::cef_string_utf8_to_utf16(emp.as_ptr() as *mut c_char, 0, &mut empty_str);}

    empty_str
}

unsafe extern "C" fn dtr(_: *mut cef::char16) {
    // println!("DESTROY CEF_STRING");
}

pub fn str_from_c(cstr: *const c_char) -> &'static str {
    let slice = unsafe { ::std::ffi::CStr::from_ptr(cstr) };
    let url = ::std::str::from_utf8(slice.to_bytes()).unwrap();
    url
}