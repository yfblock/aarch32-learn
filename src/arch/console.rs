use arm_pl011::Pl011Uart;
use spin::Mutex;

static PL011: Mutex<Pl011Uart> = Mutex::new(Pl011Uart::new(0x09000000 as *mut u8));

#[inline]
pub fn putchar(c: u8) {
    PL011.lock().putchar(c);
}

#[inline]
pub fn write(s: &str) {
    s.as_bytes().iter().for_each(|x| PL011.lock().putchar(*x));
}

pub fn init() {
    PL011.lock().init();
}

// pub fn getchar() -> Option<u8> {
//
// }
