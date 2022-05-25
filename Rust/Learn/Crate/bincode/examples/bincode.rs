use bincode::{config::*, BorrowDecode};
use byteorder::{ByteOrder, ReadBytesExt};
use std::mem;
use std::{
    fs,
    fs::File,
    io::{Cursor, Read, Write},
};

type BinConfig = bincode::config::Configuration<LittleEndian, Varint, SkipFixedArrayLength>;

#[derive(BorrowDecode, bincode::Encode, PartialEq, Debug, Eq)]
//#[repr(C)]
pub struct Foo<'a> {
    a: String, // 24
    //v: Vec<u8>, // 24 OR ...
    s: &'a str, // 16
    b: u64,     // 8
    b1: u64,    // 8
    b2: u64,    // 8
    b3: u64,    // 8
                // 24*2+16+8*4=96-24=72
}

const BIGCONFIG: BinConfig = bincode::config::Configuration::standard();

fn main() {
    let f = Foo {
        a: "oadawdawdawd啊我打我打为wdadadAWdaw大我打我打我打我打我打按大王".to_string(),
        //v: vec![1, 0, 0, 0],
        s: "aa啊我打我打",
        b: 18446744073709551611,
        b1: 18446744073709551611,
        b2: 18446744073709551611,
        b3: 18446744073709551611,
    };
    println!("{}", mem::size_of::<Foo>());

    let en = bincode::encode_to_vec(&f, BIGCONFIG).unwrap();

    write_bytes("./w", &en).unwrap();

    let buffer = read_as_bytes("./w");

    let mut rdr: Cursor<&[u8]> = Cursor::new(&buffer);
    println!("{}", read_string(&mut rdr).unwrap());
    //println!("{}", rdr.read_u8().unwrap());
    //println!("{}", rdr.read_u16::<byteorder::LittleEndian>().unwrap());
    println!("{}", read_string(&mut rdr).unwrap());
    println!("{}", rdr.read_u64::<byteorder::LittleEndian>().unwrap());
    println!("{}", rdr.read_u64::<byteorder::LittleEndian>().unwrap());
    println!("{}", rdr.read_u64::<byteorder::LittleEndian>().unwrap());
    println!("{}", rdr.read_u64::<byteorder::LittleEndian>().unwrap());
}
pub fn write_bytes(file: &str, bytes: &[u8]) -> std::io::Result<()> {
    let mut f = fs::File::create(file)?;
    f.write_all(bytes)
}

pub fn read_string<T>(rdr: &mut T) -> Result<String, std::string::FromUtf8Error>
where
    T: Read,
{
    let str_len = rdr.read_u8().expect("E: str_len");
    let mut str_bytes = vec![0_u8; str_len as usize];

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

fn test_w1() {
    // https://users.rust-lang.org/t/format-string-to-buffer-and-get-length/37679/8
    let mut buf = [0u8; 65536];

    let mut cursor = Cursor::new(&mut buf[..]);
    let s = "string";
    write!(cursor, "test {} 123", s).unwrap();
    let len = cursor.position() as usize;

    println!("{}", std::str::from_utf8(&buf[..len]).unwrap());
}

fn parse_mg() {
    #[repr(C)]
    #[repr(align(4))]
    struct Mg {
        a: String, // 24
        b: u64,    // 8
        b1: u64,   // 8
    }

    unsafe {
        let my_struct = Mg {
            a: "sss".to_string(),
            b: 1,
            b1: 0,
        };

        let bytes = any_as_u8_slice(&my_struct);
        println!("{:?}", bytes);

        let mut f = fs::File::create("./k").unwrap();
        f.write_all(bytes);

        let buffer = read_as_bytes("./k");

        let mut f: Cursor<&[u8]> = Cursor::new(&buffer);

        let a = f.read_u8().unwrap();
        let b = f.read_u8().unwrap();
        let c = f.read_u64::<byteorder::LittleEndian>().unwrap();
        let d = f.read_u64::<byteorder::LittleEndian>().unwrap();
        println!(
            "{}
              {}
              {}
              {}",
            a, b, c, d
        );
    }
}
