#![no_std]  //標準ライブラリのリンクを無効化
#![no_main] //通常のエントリポイントを使用しない

use core::panic::PanicInfo; //panic_handlerのために利用

//この関数はパニック時に呼ばれる
#[cfg(not(test))]   //->https://github.com/rust-lang/rust-analyzer/issues/4490
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]    //OSのエントリポイントを独自の_start関数で上書きしていく
pub extern "C" fn _start() -> !{
    //リンカは`_start`という名前の関数を探すのでこの関数がエントリポイントとなる
    //デフォルトでは_startという名前を探すため
    loop {}
}