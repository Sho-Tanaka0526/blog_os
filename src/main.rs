#![no_std]  //標準ライブラリのリンクを無効化
#![no_main] //通常のエントリポイントを使用しない

use core::panic::PanicInfo; //panic_handlerのために利用

mod vga_buffer;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]    //OSのエントリポイントを独自の_start関数で上書きしていく
pub extern "C" fn _start() -> !{
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

//この関数はパニック時に呼ばれる
#[cfg(not(test))]   //->https://github.com/rust-lang/rust-analyzer/issues/4490
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}