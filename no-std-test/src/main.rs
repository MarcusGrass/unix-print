#![no_std]
#![no_main]
use core::panic::PanicInfo;
use unix_print::{unix_dbg, unix_eprint, unix_eprintln, unix_print, unix_println};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unix_dbg!();
    unix_print!("Print 1 ->");
    unix_print!(" Print 2 ->");
    unix_print!(" Print last.");
    unix_println!();
    unix_println!("Did 3 prints to stdout");
    unix_eprint!("Print 1 ->");
    unix_eprint!(" Print 2 ->");
    unix_eprint!(" Print last.");
    unix_eprintln!();
    unix_eprintln!("Did 3 prints to stderr");
    let my_test_five = 5;
    unix_dbg!(my_test_five);
    unix_println!("Bye!");
    0
}
