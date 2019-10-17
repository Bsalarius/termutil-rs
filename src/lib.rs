extern crate libc;
#[cfg(windows)]
extern crate winapi;

use libc::{c_char, c_uint};
use std::ffi::CStr;
use std::io::Read;

#[no_mangle]
pub fn tu_pause() {
    println!("Press any key to continue...");
    let _ = std::io::stdin().bytes().next();
}

#[no_mangle]
pub fn tu_pause_with_message(msg: *const c_char) {
    let msg = unsafe { CStr::from_ptr(msg) };
    let msg = msg.to_str().unwrap();
    println!("{}", msg);
    let _ = std::io::stdin().bytes().next();
}

#[no_mangle]
#[cfg(windows)]
pub fn tu_clear() {
    if let Ok(_) = std::env::var("ANSICON") {
        print!("\x1B[2J");
    } else {
        unsafe {
            use winapi::um::processenv::GetStdHandle;
            use winapi::um::winbase::STD_OUTPUT_HANDLE;
            use winapi::um::wincon::{GetConsoleScreenBufferInfo, FillConsoleOutputCharacterA,
                SetConsoleCursorPosition, CONSOLE_SCREEN_BUFFER_INFO};
            use winapi::um::wincontypes::{COORD, SMALL_RECT};

            let handle_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
            let home = COORD {
                X: 0,
                Y: 0,
            };
            let mut _chars_written = 0;
            let mut console_info = CONSOLE_SCREEN_BUFFER_INFO {
                dwSize: COORD { X: 0, Y: 0 },
                dwCursorPosition: COORD { X: 0, Y: 0 },
                wAttributes: 0,
                srWindow: SMALL_RECT { Left: 0, Top: 0, Right: 0, Bottom: 0 },
                dwMaximumWindowSize: COORD { X: 0, Y: 0 },
            };
            GetConsoleScreenBufferInfo(handle_stdout, &mut console_info);
            FillConsoleOutputCharacterA(handle_stdout, ' ' as i8,
                (console_info.dwSize.X * console_info.dwSize.Y) as u32,
                home, &mut _chars_written);
            SetConsoleCursorPosition(handle_stdout, home);
        }
    }
}

#[no_mangle]
#[cfg(not(windows))]
pub fn tu_clear() {
    print!("\x1B[2J");
}

#[no_mangle]
pub fn tu_sleep(millis: c_uint) {
    use std::{thread, time};
    thread::sleep(time::Duration::from_millis(millis.into()));
}
