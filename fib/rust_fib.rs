#![crate_type = "dylib"]

extern crate libc;
pub use libc::types::os::arch::c95::c_long;
pub use libc::types::os::arch::c99::uintptr_t;

fn fib(number: uint) -> uint {
  if number <= 1 {
    return number;
  } else {
    return fib(number - 1) + fib(number - 2);
  }
}

extern fn process(context: libc::uintptr_t, input: libc::uintptr_t) -> libc::uintptr_t {
  unsafe {
    let number = rb_num2long(input);

    return rb_int2big(fib(number));
  }
}

#[link(name = "ruby")]
extern {
  fn rb_define_module(name: *const libc::c_char) -> libc::uintptr_t;

  fn rb_num2long(input: libc::uintptr_t) -> uint;

  fn rb_define_singleton_method(
    klass: libc::uintptr_t,
    name: *const libc::c_char,
    callback: extern fn(libc::uintptr_t, libc::uintptr_t) -> libc::uintptr_t,
    argc: libc::c_int
  );

  fn rb_int2big(value: uint) -> libc::uintptr_t;
}

#[no_mangle]
pub extern fn Init_librust_fib() {
  unsafe {
    let module = rb_define_module("RustImplementation".to_c_str().as_ptr());
    rb_define_singleton_method(module, "fib".to_c_str().as_ptr(), process, 1);
  };
}
