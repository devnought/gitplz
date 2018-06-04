extern crate termcolor;

use std::io::{self, Write};
use termcolor::{StandardStreamLock, WriteColor};

pub use termcolor::{Color, ColorChoice, ColorSpec, StandardStream};

pub struct ColorPrinter<'a> {
    is_terminal: bool,
    handle: StandardStreamLock<'a>,
}

impl<'a> ColorPrinter<'a> {
    pub fn new(is_terminal: bool, stream: &'a StandardStream) -> Self {
        Self {
            is_terminal,
            handle: stream.lock(),
        }
    }

    pub fn color_context<F>(&mut self, color_spec: &ColorSpec, func: F)
    where
        F: Fn(&mut Write) -> (),
    {
        if !self.is_terminal {
            func(&mut self.handle);
            return;
        }

        self.handle.set_color(color_spec).expect("color set fail");
        func(&mut self.handle);
        self.handle.reset().expect("Color reset fail");
    }
}

impl<'a> Write for ColorPrinter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.handle.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.handle.flush()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
