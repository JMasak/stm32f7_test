#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate stm32f7;

use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use cortex_m::asm;

fn main() {
    
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn rust_oom(_: AllocLayout) -> ! {
    panic!("out of memory");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    asm::bkpt();
    loop {}
}
