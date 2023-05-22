#![no_std]  //標準ライブラリのリンクを無効化
#![no_main] //通常のエントリポイントを使用しない

use core::panic::PanicInfo;

//OSのエントリポイントを独自の_start関数で上書きしていく
#[no_mangle]
pub extern "C" fn _start() -> !{
    loop {}
}

//この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}