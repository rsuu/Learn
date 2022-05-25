use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fs,
    fs::File,
    io::{self, Read, Write},
};

struct Configuration {
    item1: u8,
    item2: u16,
    item3: i32,
}

impl Configuration {
    fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let item1 = rdr.read_u8()?;
        let item2 = rdr.read_u16::<LittleEndian>()?;
        let item3 = rdr.read_i32::<LittleEndian>()?;

        Ok(Configuration {
            item1,
            item2,
            item3,
        })
    }
}


use std::io::Cursor;
fn main() {
    let c = "a啊我打我打我打我打啊萨打算打算大".as_bytes();

    // println!("{}", read_string(&mut rdr));

    // ... later in code
    let mut f = File::create("./w").unwrap();

    f.write_all(c).unwrap();

    println!("{:?}", &c);
    let b = read_bytes_from("./w");
    let mut rdr: Cursor<&Vec<u8>> = Cursor::new(&b);

    println!("{}", read_string(&mut rdr));
    //println!("{}", String::from_utf8(wtr).unwrap());
}
pub fn read_string(rdr: &mut Cursor<&Vec<u8>>) -> String {
    let str_len = rdr.read_u8().unwrap();
    let str_bytes = vec![0_u8; str_len as usize];
    String::from_utf8(str_bytes).unwrap()
}

fn read_bytes_from(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

//     https://gist.github.com/porglezomp/9bd99d69926f96501682d5681511cbd4
//
