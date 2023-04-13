use std::collections::HashSet;
use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn create_counter_context() -> *mut c_void {
    let ctx = Box::new(CounterContext {
        set: HashSet::new(),
    });
    Box::into_raw(ctx) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn insert_counter_context(context: *mut c_void, value: i32) {
    let mut counter = Box::from_raw(context as *mut CounterContext);
    counter.set.insert(value);
    Box::into_raw(counter);
}

#[no_mangle]
pub unsafe extern "C" fn delete_counter_context(context: *mut c_void) {
    let counter = Box::from_raw(context as *mut CounterContext);
    for value in counter.set.iter() {
        println!("counter value: {}", value)
    }
}

#[repr(C)]
pub struct CounterContext {
    pub set: HashSet<i32>,
}