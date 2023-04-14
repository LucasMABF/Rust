#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::ptr::write_volatile;
    unsafe{
        // turn pin 21 into an output
        write_volatile(0x3F20_0008 as *mut u32, 1<<3);
        loop{
            // turn pin 21 on
            write_volatile(0x3F20_001C as *mut u32, 1<<21);

            // sleep
            for _ in 1..50000{
                asm!("nop");
            }

            // turn pin 21 off
            write_volatile(0x3F20_001C as *mut u32, 1<<21);

            // sleep
            for _ in 1..50000{
                asm!("nop");
            }
        }
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}
