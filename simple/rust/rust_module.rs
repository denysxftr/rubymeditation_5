#![crate_type = "dylib"]

extern crate libc;
pub use libc::types::os::arch::c95::c_char;
pub use libc::types::common::c95::c_void;

extern fn test_method(env: *mut libc::c_void) {
  println!("BOOO!!!");
}

#[link(name = "ruby")]
extern {
  // define ruby module
  fn rb_define_module(name: *const libc::c_char) -> *mut libc::c_void;

  // define singleton method for object
  fn rb_define_singleton_method(
    klass: *mut libc::c_void,
    name: *const libc::c_char,
    callback: extern fn(*mut libc::c_void),
    argc: libc::c_int
  );
}

#[no_mangle]
pub extern fn Init_librust_module() {
  let module = unsafe { rb_define_module("RustModule".to_c_str().as_ptr()) };
  unsafe { rb_define_singleton_method(module, "test_method".to_c_str().as_ptr(), test_method, 0) };
}