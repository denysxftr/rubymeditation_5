#include "ruby.h"

VALUE rb_module;

VALUE test_method() {
  printf("BOOO!!!\n");

  return Qnil;
};

void Init_c_module() {
  rb_module = rb_define_module("CModule");
  rb_define_singleton_method(rb_module, "test_method", test_method, 0);
}