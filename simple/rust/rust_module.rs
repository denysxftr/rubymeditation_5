#![crate_type = "dylib"]

extern crate libc;
pub use libc::types::os::arch::c95::c_char;
pub use libc::types::os::arch::c99::uintptr_t;

extern fn test_method(env: libc::uintptr_t) {
  println!("BOOO!!!");
}

#[link(name = "ruby")]
extern {
  // define ruby module
  fn rb_define_module(name: *const libc::c_char) -> libc::uintptr_t;

  // define singleton method for object
  fn rb_define_singleton_method(
    klass: libc::uintptr_t,
    name: *const libc::c_char,
    callback: extern fn(libc::uintptr_t),
    argc: libc::c_int
  );
}

#[no_mangle]
pub extern fn Init_librust_module() {
  unsafe {
    let module = rb_define_module("RustModule".to_c_str().as_ptr()) };
    rb_define_singleton_method(module, "test_method".to_c_str().as_ptr(), test_method, 0);
  };
}