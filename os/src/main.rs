#![no_std]
#![no_main]

mod lang_items;
mod sbi;
mod console;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    sbi::shutdown();
}