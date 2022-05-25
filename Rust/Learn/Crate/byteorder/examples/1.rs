use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use std::{fs, fs::File, io, io::Cursor, io::Read, path::Path};

#[derive(Debug)]
pub struct Foo {
    a: String,
    b: u32,
}

fn main() -> io::Result<()> {
    let f = "./w.txt";
    let buffer = read_to_bytes(f);
    let mut rdr = Cursor::new(&buffer);
    println!("{}", rdr.read_u8().unwrap());
    println!("{}", rdr.read_uint::<BigEndian>(2).unwrap());
    read_parse_file(f).unwrap();

    Ok(())
}

fn read_to_bytes(filepath: &str) -> Vec<u8> {
    let mut f = File::open(&filepath).expect("no file found");
    let metadata = fs::metadata(&filepath).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn read_parse_file(filepath: &str) -> io::Result<()> {
    let path = Path::new(&filepath);
    let display = path.display();
    println!("path is : {}", &display);

    let _f = File::open(&path)?;
    let buffer = read_to_bytes(filepath);
    println!("{:?}", buffer);

    let mut rdr: Cursor<&Vec<u8>> = Cursor::new(&buffer);
    println!("{}", read_string(&mut rdr));
    println!("{}", rdr.read_u16::<LittleEndian>().unwrap());

    let mut rdr: Cursor<&Vec<u8>> = Cursor::new(&buffer);
    println!("{}", read_string(&mut rdr));
    let a = read_usize::<BigEndian, Cursor<&Vec<u8>>>(&mut rdr);
    println!("{:?}", a.unwrap());

    Ok(())
}

pub fn read_string(rdr: &mut Cursor<&Vec<u8>>) -> String {
    let str_len = rdr.read_u8().unwrap();
    let mut str_bytes = vec![0_u8; str_len as usize];

    rdr.read_exact(&mut str_bytes).unwrap();

    String::from_utf8(str_bytes).unwrap()
}

fn read_usize<B, R>(b: &mut R) -> Result<usize, std::io::Error>
where
    B: ByteOrder,
    R: ReadBytesExt,
{
    if cfg!(target_pointer_width = "64") {
        b.read_u64::<B>().map(|v| v as usize)
    } else if cfg!(target_pointer_width = "32") {
        b.read_u32::<B>().map(|v| v as usize)
    } else {
        b.read_u16::<B>().map(|v| v as usize)
    }
}
