#![no_std]  //標準ライブラリのリンクを無効化
#![no_main] //通常のエントリポイントを使用しない

use core::panic::PanicInfo; //panic_handlerのために利用

mod vga_buffer;

<<<<<<< HEAD
//static HELLO: &[u8] = b"Hello World!";
=======
// static HELLO: &[u8] = b"Hello World!";
>>>>>>> 88ba26de62fb5e0597212cdec0e7354894d2072e

#[no_mangle]    //OSのエントリポイントを独自の_start関数で上書きしていく
pub extern "C" fn _start() -> !{
    // //`_start`という名前のエントリポイント関数
    // let vga_buffer = 0xb8000 as *mut u8;    //整数を生ポインタにキャスト

    // for (i, &byte) in HELLO.iter().enumerate() {    //HELLOというバイト列変数の要素に対してイテレート
    //     unsafe {    //メモリへの書き込み処理コード
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    vga_buffer::print_something();

    loop {}
}

//この関数はパニック時に呼ばれる
#[cfg(not(test))]   //->https://github.com/rust-lang/rust-analyzer/issues/4490
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}