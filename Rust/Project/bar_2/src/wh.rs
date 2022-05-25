pub struct UnixSize(pub u16, pub u16);

use libc::{ioctl, isatty, TIOCGWINSZ};
use std::os::unix::prelude::RawFd;

pub fn terminal_size() -> Option<UnixSize> {
    terminal_size_using_fd(libc::STDOUT_FILENO)
}

pub fn terminal_size_using_fd(fd: RawFd) -> Option<UnixSize> {
    let is_tty: bool = unsafe { isatty(fd) == 1 };

    if !is_tty {
        return None;
    }

    let mut winsize = UnixSize(0, 0);

    if unsafe { ioctl(fd, TIOCGWINSZ, &mut winsize) } == -1 {
        return None;
    }

    let rows = winsize.0;
    let cols = winsize.1;

    if rows > 0 && cols > 0 {
        Some(UnixSize(cols, rows))
    } else {
        None
    }
}
