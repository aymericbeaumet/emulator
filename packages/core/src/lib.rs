mod emulators;
mod screen;

use std::boxed::Box;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GameBoyColor_new() -> *mut emulators::gameboycolor::GameBoyColor {
    Box::into_raw(Box::new(emulators::gameboycolor::GameBoyColor::new()))
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GameBoyColor_input(
    engine_ptr: *mut emulators::gameboycolor::GameBoyColor,
    inputs: u8,
) {
    (unsafe { &mut *engine_ptr }).input(inputs);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GameBoyColor_render(
    engine_ptr: *mut emulators::gameboycolor::GameBoyColor,
    callback: fn(*mut u32, usize, usize),
) {
    (unsafe { &mut *engine_ptr }).render(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GameBoyColor_delete(engine_ptr: *mut emulators::gameboycolor::GameBoyColor) {
    unsafe { Box::from_raw(engine_ptr) };
}
