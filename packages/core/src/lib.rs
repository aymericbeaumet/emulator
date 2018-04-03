use std::boxed::Box;

pub struct Core {}

impl Core {
    fn new() -> Self {
        Core {}
    }

    fn input(&self) {}

    fn update(&self) {}

    fn render(&self, callback: fn()) {
        callback();
    }
}

impl Drop for Core {
    fn drop(&mut self) {}
}

/*
 * C bindings
 */

#[no_mangle]
pub extern "C" fn new() -> *mut Core {
    Box::into_raw(Box::new(Core::new()))
}

#[no_mangle]
pub extern "C" fn input(core_ptr: *mut Core) {
    (unsafe { &*core_ptr }).input();
}

#[no_mangle]
pub extern "C" fn update(core_ptr: *mut Core) {
    (unsafe { &*core_ptr }).update();
}

#[no_mangle]
pub extern "C" fn render(core_ptr: *mut Core, callback: fn()) {
    (unsafe { &*core_ptr }).render(callback);
}

#[no_mangle]
pub extern "C" fn delete(core_ptr: *mut Core) {
    unsafe { Box::from_raw(core_ptr) };
}
