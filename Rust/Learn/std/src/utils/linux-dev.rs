pub fn dev_urandom() -> io::Result<u8> {
    use std::{fs::File, io, io::Read};
    let mut f = File::open("/dev/urandom").unwrap();
    let mut buf = [0_u8; 1];

    f.read_exact(&mut buf)?;

    Ok(buf[0].into())
}
