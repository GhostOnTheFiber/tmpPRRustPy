extern crate cuckoofilter;
extern crate serde_json;
extern crate libc;

use cuckoofilter::{CuckooFilter, ExportedCuckooFilter};
use std::collections::hash_map::DefaultHasher;
use libc::c_char;
use std::slice;
use std::ffi::CString;


#[no_mangle]
pub extern fn createFilter(ptr: *const i8, len: usize) -> *const u8{
    assert!(!ptr.is_null());
    let array = unsafe { slice::from_raw_parts(ptr, len) };
    //array.iter().sum()
    
    // Just a small filter to test serialization.
    let mut filter = CuckooFilter::<DefaultHasher>::with_capacity(100);
    // Fill a few values.
    for i in array {
        filter.add(&i).unwrap();
    }
    // export data.
    let store: ExportedCuckooFilter = filter.export();
    
    // serialize using json (for example, any serde format can be used).
    let saved_json = serde_json::to_string(&store).unwrap();
    let s = CString::new(saved_json).unwrap();
    //s.into_raw()
} 
// pub extern fn serialization() {
//     // Just a small filter to test serialization.
//     let mut filter = CuckooFilter::<DefaultHasher>::with_capacity(100);

//     // Fill a few values.
//     for i in 0..50 {
//         filter.add(&i).unwrap();
//     }
//     // export data.
//     let store: ExportedCuckooFilter = filter.export();

//     // serialize using json (for example, any serde format can be used).
//     let saved_json = serde_json::to_string(&store).unwrap();

//     // create a new filter from the json string.
//     let restore_json: ExportedCuckooFilter = serde_json::from_str(&saved_json).unwrap();
//     let recovered_filter = CuckooFilter::<DefaultHasher>::from(restore_json);

//     // Check our values exist within the reconstructed filter.
//     for i in 0..50 {
//         assert!(recovered_filter.contains(&i));
//     }
// }