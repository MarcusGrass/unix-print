# Unix print
A `no_std`, `no_libc` library that allows using Rust's standard syscall macros 
to output to stdout or stderr.  
*Should work on posix compliant OSes but is only tested on linux

## Examples
Used just like the regular `print!`, `println!`, `eprint!`, `eprintln!`, and `dbg!` macros.

See the [example no_std binary for usage](no-std-test/src/main.rs), run it with `cargo run -p no-std-test`.

## Credits
Heavily inspired by [rust-libc-print](https://github.com/mmastrac/rust-libc-print).

## License
Licensed under [MIT](LICENSE).