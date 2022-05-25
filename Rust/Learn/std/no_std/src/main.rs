#[no_std]
use core::fmt::{self, Write};

struct Stdout;
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ret = unsafe { libc::write(libc::STDOUT_FILENO, s.as_ptr() as *const _, s.len()) };
        if ret == s.len() as isize {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let _ = writeln!(&mut Stdout, "Hello, world!");
    0
}
