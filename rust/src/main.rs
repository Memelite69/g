#![no_std]  // No implicit linking to the std lib.
#![no_main] // There is no runtime, we define our own entry point
use core::panic::PanicInfo;

#[link(wasm_import_module = "host")]
extern "C" {
    fn putchar(c: i32);
}

fn puts(str: &str) {
    for c in str.chars() {
        unsafe {putchar(c as i32)};
    }
    unsafe {putchar('\n' as i32)};
}

fn int_to_ascii(_i: i32) -> &'static str{
    //TODO
    return "TODO itoa"
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    puts("PANIC!");
    loop{};
}

fn fizzbuzz(n: i32){
    if n % 15 == 0 {
        puts("fizzbuzz");
    } else if n % 5 == 0 {
        puts("buzz");
    } else if n % 3 == 0 {
        puts("fizz");
    } else {
        puts(int_to_ascii(n));
    }
}

#[no_mangle]
pub fn main(){
    for n in 0..100 {
        fizzbuzz(n);
    }
}
