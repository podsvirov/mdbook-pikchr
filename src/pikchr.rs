use libc::{c_char, c_int, c_uint, c_void, free};
use std::ffi::{CStr, CString};
use std::fmt;
use std::ops::Deref;

extern "C" {
    #[allow(non_snake_case)]
    pub fn pikchr(
        zText: *const c_char,
        zClass: *const c_char,
        mFlags: c_uint,
        pnWidth: *mut c_int,
        pnHeight: *mut c_int,
    ) -> *mut c_char;
}

pub struct Pikchr {
    rendered: *const c_char,
    pub width: c_int,
    pub height: c_int,
}

impl Drop for Pikchr {
    fn drop(&mut self) {
        if self.rendered.is_null() {
            unsafe {
                free(self.rendered as *mut c_void);
            }
            self.rendered = std::ptr::null();
        }
    }
}

impl Deref for Pikchr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe {
            let cstr = CStr::from_ptr(self.rendered);
            std::str::from_utf8_unchecked(cstr.to_bytes())
        }
    }
}

impl fmt::Display for Pikchr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self)
    }
}

impl Pikchr {
    pub fn render(source: &str, class: Option<&str>) -> Result<Pikchr, String> {
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let flags: c_uint = 0;
        let source = CString::new(source).map_err(|e| format!("{:?}", e))?;
        let res: *mut c_char = unsafe {
            pikchr(
                source.as_ptr() as *const c_char,
                class
                    .map(|s| s.as_ptr() as *const c_char)
                    .unwrap_or(std::ptr::null()),
                flags,
                &mut width as *mut c_int,
                &mut height as *mut c_int,
            )
        };
        if width < 0 {
            let err = unsafe { CStr::from_ptr(res) };
            let err = err.to_bytes();
            let err = String::from_utf8_lossy(err).into_owned();
            unsafe {
                free(res as *mut c_void);
            }
            Err(err)
        } else {
            Ok(Pikchr {
                rendered: res,
                width,
                height,
            })
        }
    }
}
