use std::ffi::c_void;
use heatflow::{HeatFlow, HeatMap};

#[no_mangle]
pub unsafe extern "C" fn heat_flow_as_heat_map(heat_flow: *mut c_void) -> *mut c_void {
    let cs_object = Box::from_raw(heat_flow as *mut HeatFlow);
    let object = box cs_object.as_heatmap();
    Box::into_raw(object) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn delete_heat_map(heat_map: *mut c_void) {
    let _ = Box::from_raw(heat_map as *mut HeatMap);
}

#[no_mangle]
pub unsafe extern "C" fn heat_map_get_t(heat_map: *mut c_void) -> usize {
    let mut cs_object = Box::from_raw(heat_map as *mut HeatMap);
    cs_object.get_time()
}

#[no_mangle]
pub unsafe extern "C" fn heat_map_get_w(heat_map: *mut c_void) -> usize {
    let mut cs_object = Box::from_raw(heat_map as *mut HeatMap);
    cs_object.get_w()
}

#[no_mangle]
pub unsafe extern "C" fn heat_map_get_h(heat_map: *mut c_void) -> usize {
    let mut cs_object = Box::from_raw(heat_map as *mut HeatMap);
    cs_object.get_h()
}