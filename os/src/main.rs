#![no_std]
#![no_main]
#![feature(llvm_asm)]
//#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

global_asm!(include_str!("entry.asm"));

#[macro_use]
mod console;
mod lang_item;
mod sbi;
mod interrupt;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, sp: usize) -> ! {
    println!("Hello world #{}! sp = 0x{:x}", hartid, sp);

    interrupt::init();

    /*
    let _hart0_m_int_enable: *mut u32 = 0x0c00_2000 as *mut u32;
    let _hart0_s_int_enable: *mut u32 = 0x0c00_2080 as *mut u32;
    let _hart1_m_int_enable: *mut u32 = 0x0c00_2100 as *mut u32;
    let _hart1_s_int_enable: *mut u32 = 0x0c00_2180 as *mut u32;
    unsafe {
        for i in 0xa..=0xd {
            /*
            _hart0_m_int_enable.write_volatile(1 << i);
            _hart0_s_int_enable.write_volatile(1 << i);
             */
            /*
            _hart1_m_int_enable.write_volatile(1 << i);
            _hart1_s_int_enable.write_volatile(1 << i);
             */
        }
        _hart0_m_int_enable.write_volatile(0);
        _hart0_s_int_enable.write_volatile(0);
        _hart1_m_int_enable.write_volatile(0);
        _hart1_s_int_enable.write_volatile(0);
    }
     */

    /*
    loop {
        let getc = sbi::console_getchar() as i32;
        if getc != -1 {
            /*
            match getc as u8 {
                LF | CR => {
                    print!("{}", LF as char);
                    print!("{}", CR as char);
                }
                _ => {
                    print!("{}", getc as u8 as char);
                }
            }
            */
            //print!("{}", getc as u8 as char);
            sbi::console_putchar(getc as usize);
        }
    }
     */

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }

    println!("Hello world again!");

    /*
    let somewhere_you_cannot_write = 0x12345678 as *mut usize;
    unsafe {
        somewhere_you_cannot_write.write_volatile(0usize);
    }
    */

    /*
    loop {
    //for _ in 0..10 {
        let mtime = 0x200bff8 as *const usize;
        println!("mtime = {}", unsafe { mtime.read_volatile() });
    }
    */

    loop {}
    //panic!("end of rust_main!")
}

