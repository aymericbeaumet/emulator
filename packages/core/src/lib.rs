mod emulators;
mod screen;

use std::boxed::Box;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GameBoyColor_new() -> *mut emulators::gameboycolor::GameBoyColor {
    Box::into_raw(Box::new(emulators::gameboycolor::GameBoyColor::new()))
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GameBoyColor_load(
    emulator_ptr: *mut emulators::gameboycolor::GameBoyColor,
    filepath: *const c_char,
) {
    let filepath = (unsafe { CStr::from_ptr(filepath) }).to_str().unwrap();
    (unsafe { &*emulator_ptr }).load(filepath);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GameBoyColor_input(
    emulator_ptr: *mut emulators::gameboycolor::GameBoyColor,
    inputs: u8,
) {
    (unsafe { &mut *emulator_ptr }).input(inputs);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GameBoyColor_render(
    emulator_ptr: *mut emulators::gameboycolor::GameBoyColor,
    callback: fn(*mut u32, usize, usize),
) {
    (unsafe { &mut *emulator_ptr }).render(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GameBoyColor_delete(emulator_ptr: *mut emulators::gameboycolor::GameBoyColor) {
    unsafe { Box::from_raw(emulator_ptr) };
}
