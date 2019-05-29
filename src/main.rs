#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate stm32f7;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}    
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
