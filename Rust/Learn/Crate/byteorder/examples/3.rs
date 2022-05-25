use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::{fs, io, mem, path};

#[derive(Debug, Copy, Clone)]
struct A<'a> {
    a: [u8; 2],
    s: &'a str,
}

#[derive(Debug, Copy, Clone)]
struct B<'a> {
    a: [u8; 3],
    s: &'a str,
}

#[derive(Debug)]
struct BytesSlice<'a> {
    slice: &'a [u8],
    size: usize,
    slice_len: usize,
    max: usize,
}

trait BytesExt: Sized {
    fn as_bytes(&self) -> &[u8];
    fn from_ref(buf: &[u8]) -> &Self;
}

impl<'a> BytesExt for A<'a>
where
    Self: Sized + Copy,
{
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
    fn from_ref(buf: &[u8]) -> &Self {
        let p: *const Self = buf.as_ptr() as *const Self;
        unsafe { &*p }
    }
}

pub fn read_bytes(file: &str) -> io::Result<Vec<u8>> {
    let mut f = fs::File::open(&file).expect("no file found");
    let metadata = fs::metadata(&file).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];

    f.read_exact(&mut buffer)?;

    Ok(buffer)
}

pub fn append_bytes(file: &str, bytes: &[u8]) -> io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file)
        .expect("");

    f.write_all(bytes)
}

impl<'a> BytesSlice<'a> {
    pub fn read(&mut self) -> &[u8] {
        let s = &(self.slice[0..self.size]);
        self.slice_len -= self.size;
        s
    }

    pub fn to_struct<T>(&mut self) -> &T
    where
        T: Sized + Copy + BytesExt,
    {
        let b = self.read();
        <T as BytesExt>::from_ref(b)
    }

    pub fn to_vec_struct<T>(&mut self, v: &mut Vec<T>)
    where
        T: Sized + Copy + BytesExt,
    {
        for _f in 0..self.max {
            let s = self.to_struct::<T>();
            v.push(*s);
        }
    }
}

fn main() {
    let val_size: usize = mem::size_of::<A>();

    let s: Vec<A> = vec![
        A {
            a: [0, 2],
            s: "ccc",
        },
        A {
            a: [0, 1],
            s: "akasssssssssssssssssasdasdki",
        },
    ];

    let mut bytes: Vec<u8> = Vec::with_capacity(val_size * s.len());

    for f in s.iter() {
        bytes.extend_from_slice(f.as_bytes());
    }

    let file = "test";
    if let Ok(_) = fs::File::create(file) {
        append_bytes(file, bytes.as_slice()).unwrap();
    }

    let bytes = read_bytes("test").expect("");
    let bytes = bytes.as_slice();

    let mut bytes_slice = BytesSlice {
        slice: bytes,
        size: val_size,
        slice_len: bytes.len(),
        max: bytes.len() / val_size,
    };

    if bytes_slice.slice_len > 0 {
        let mut v: Vec<A> = Vec::with_capacity(bytes_slice.slice_len);

        bytes_slice.to_vec_struct(&mut v);

        println!("{:#?}", v);
    }
}

// https://github.com/rust-lang/project-safe-transmute/blob/master/rfcs/0000-ext-byte-transmutation.md
//https://www.reddit.com/r/rust/comments/dw2vb3/convert_from_u8_to_generic_sized_struct/
//transmute from slice [u8] to A: 23.358µs
//transmute_copy from slice [u8] to A: 21.771µs
//convert from slice [u8] to &A: 1.426µs
