use core::arch::global_asm;

use crate::arch::utils::get_pl;

global_asm!(include_str!("head.S"));

#[unsafe(no_mangle)]
extern "C" fn boot_rust_stage1() {
    super::console::init();
    let pl = get_pl();
    println!("{:?}", pl);
    crate::main();
    panic!("shutdown manually")
}
