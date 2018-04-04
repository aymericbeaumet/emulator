mod game;

use std::boxed::Box;

#[no_mangle]
pub extern "C" fn game_engine_engine_new() -> *mut game::engine::Engine {
    Box::into_raw(Box::new(game::engine::Engine::new()))
}

#[no_mangle]
pub extern "C" fn game_engine_engine_input(engine_ptr: *mut game::engine::Engine, keys: u8) {
    (unsafe { &mut *engine_ptr }).input(keys);
}

#[no_mangle]
pub extern "C" fn game_engine_engine_update(engine_ptr: *mut game::engine::Engine) {
    (unsafe { &*engine_ptr }).update();
}

#[no_mangle]
pub extern "C" fn game_engine_engine_render(
    engine_ptr: *mut game::engine::Engine,
    callback: fn(*mut u8, usize, usize),
) {
    (unsafe { &mut *engine_ptr }).render(callback);
}

#[no_mangle]
pub extern "C" fn game_engine_engine_delete(engine_ptr: *mut game::engine::Engine) {
    unsafe { Box::from_raw(engine_ptr) };
}
