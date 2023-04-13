use std::ffi::c_void;
use shape_core::{Point, Rectangle};
use heatflow::HeatFlow;

/// Returns a new [`HeatFlow`]
#[no_mangle]
pub extern "C" fn create_heat_flow(x: f32, y: f32, w: f32, h: f32, resolution: f32, max_time: usize) -> *mut c_void {
    let object = box HeatFlow::new(Rectangle::new(Point::new(x, y), (w, h)), resolution, max_time);
    Box::into_raw(object) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn delete_heat_flow(heat_flow: *mut c_void) {
    let _ = Box::from_raw(heat_flow as *mut HeatFlow);
}

#[no_mangle]
pub unsafe extern "C" fn heat_flow_time_fly(heat_flow: *mut c_void) {
    let mut cs_object = Box::from_raw(heat_flow as *mut HeatFlow);
    cs_object.time_fly();
    Box::into_raw(cs_object);
}

#[no_mangle]
pub unsafe extern "C" fn heat_flow_sampling(heat_flow: *mut c_void, x: f32, y: f32, weight: f32) {
    let mut cs_object = Box::from_raw(heat_flow as *mut HeatFlow);
    cs_object.sampling(Point::new(x, y), weight);
    Box::into_raw(cs_object);
}

/// Get time from heat flow
#[no_mangle]
pub unsafe extern "C" fn heat_flow_get_t(heat_flow: *mut c_void) -> u64 {
    let cs_object = Box::from_raw(heat_flow as *mut HeatFlow);
    let out = cs_object.get_time() as u64;
    Box::into_raw(cs_object);
    out
}