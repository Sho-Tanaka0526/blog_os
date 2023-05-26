//Color
#[allow(dead_code)] //警告の打ち消し
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] //u8で型を指定する
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

//ColorCode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]    //ColorCodeがu8と同じ構造を持つようにする
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode { //前景色と背景色を持つ
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

//textbuffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]  //デフォルトの構造体におけるフィールドの並べ方が未定義のため
            //Cと同じように並べる
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

//テキストバッファのサイズを指定するグローバル変数
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

//Volatile(揮発性)でバッファへの書き込みを最適化により処理させない
use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//writer
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,    //プログラム中ずっと参照を有効にする
}

//出力
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),   //引数が改行の場合
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();    //改行する
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {  //書き込みの処理
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;  //現在の列の位置を進める
            }
        }
    }

    fn new_line(&mut self) {/* TODO */}
}

//文字列全体の出力
impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                //出力可能なASCIIバイトか、改行コード
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //出力可能なASCIIバイトではない
                _ => self.write_byte(0xfe),
            }
        }
    }
}

//一時的に使う書き出し関数
pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),    //文字色を黄色、背景色を黒色に
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');    //記述する文字列をwriterに書き込み
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}