use std::ffi::{CStr, CString};

use sdl2::{
    libc::c_int,
    sys::{SDL_GetClipboardText, SDL_HasClipboardText, SDL_SetClipboardText, SDL_bool},
};

pub fn set_clipboard_text(text: &str) -> Result<(), String> {
    let text = CString::new(text).map_err(|e| e.to_string())?;
    if unsafe { SDL_SetClipboardText(text.as_ptr()) != c_int::from(0) } {
        return Err("Error set clipboard text_board".to_owned());
    }
    Ok(())
}

pub fn has_clipboard_text() -> bool {
    unsafe { SDL_HasClipboardText() == SDL_bool::SDL_TRUE }
}

pub fn get_clipboard_text() -> Option<Result<String, String>> {
    if has_clipboard_text() {
        match unsafe { CStr::from_ptr(SDL_GetClipboardText()) }
            .to_str()
            .map_err(|e| e.to_string())
        {
            Ok(text) => Some(Ok(text.to_owned())),
            Err(err) => Some(Err(err)),
        }
    } else {
        None
    }
}
