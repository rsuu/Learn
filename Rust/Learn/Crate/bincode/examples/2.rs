use byteorder::{ByteOrder, ReadBytesExt};
use std::mem::{self, align_of_val, size_of, size_of_val};
use std::{
    fs,
    fs::File,
    io::{Cursor, Read, Write},
};

fn main() {
    parse_mg();
}

fn parse_mg() {
    #[repr(C)]
    #[repr(align(2))]
    struct Mg {
        len: usize,
        a1: [u8; 24],
    }

    unsafe {
        let mut a1 = [115; 24];

        let my_struct = Mg {
            len: a1.len(),
            a1: a1.clone(),
        };
        let a: String = "".to_string();

        println!("{}", size_of::<Mg>());
        println!("{}", std::mem::size_of_val(&my_struct));
        println!("u64: {}", size_of::<u64>());
        println!("String: {}", size_of::<String>());
        println!("&str: {}", size_of::<&str>());
        println!("{}", size_of_val(&my_struct.a1));
        println!("size {}", size_of_val(&a1));

        let bytes = any_as_u8_slice(&my_struct);
        let mut f = fs::File::create("./k").unwrap();
        f.write_all(bytes);

        let buffer = read_as_bytes("./k");
        let mut f: Cursor<&[u8]> = Cursor::new(&buffer);
        println!("{:?}", f);

        println!("{}", read_string(&mut f, my_struct.len).unwrap());
    }
}
pub fn write_bytes(file: &str, bytes: &[u8]) -> std::io::Result<()> {
    let mut f = fs::File::create(file)?;
    f.write_all(bytes)
}

pub fn read_string<T>(rdr: &mut T, len: usize) -> Result<String, std::string::FromUtf8Error>
where
    T: Read,
{
    let mut str_bytes = vec![0_u8; len];

    rdr.read_exact(&mut str_bytes).expect("E: read_exact");

    String::from_utf8(str_bytes)
}

pub fn read_string2<T>(rdr: &mut T, len: usize) -> Result<String, std::string::FromUtf8Error>
where
    T: Read,
{
    let mut str_bytes = vec![0_u8; len];
    rdr.read_exact(&mut str_bytes).expect("E: read_exact");

    String::from_utf8(str_bytes)
}

pub fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }
}

fn read_as_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");

    buffer
}
