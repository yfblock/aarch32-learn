use core::fmt::{Arguments, Write};

use crate::arch::console;

pub struct WriteImpl;

impl Write for WriteImpl {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        console::write(s);
        Ok(())
    }
}

#[doc(hidden)]
#[inline]
pub fn _print(args: Arguments) {
    let _ = WriteImpl.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::subsystem::console::_print(format_args!($fmt $(, $($arg)+)?))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::subsystem::console::_print(format_args!("{}\n", format_args!($fmt $(, $($arg)+)?)))
    };
}
