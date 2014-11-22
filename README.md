# Ruby extensions examples written in C and Rust for RubyMeditation

## Run C example
```bash
ruby extconf.rb
make
ruby test.rb
```

## Run Rust example
```bash
rustc -L /path/to/libruby/ rust_module.rs
ruby test.rb
```