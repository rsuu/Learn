use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::{fs, io, mem};

fn main() {
    //test_a();
    test_b();
}

fn test_a() {
    let s: Vec<A> = vec![
        A {
            a: [0, 2],
            s: "cac",
        },
        A {
            a: [0, 1],
            s: "akasssssssssssssssssasdasdki",
        },
    ];

    let type_size: usize = mem::size_of::<A>();
    // rustc -Z print-type-sizes 3.rs

    let mut bytes: Vec<u8> = Vec::with_capacity(type_size * s.len());

    for f in s.iter() {
        bytes.extend_from_slice(f.as_bytes());
        //println!("{:?}", bytes);
    }

    // write
    let file = "test";
    if let Ok(_) = fs::File::create(file) {
        append_bytes(file, bytes.as_slice()).unwrap();
    }

    let bytes = read_bytes("test").expect("");
    let bytes = bytes.as_slice();
    //println!("{:?}", bytes);

    // read
    let mut bytes_slice = BytesSlice::new(bytes, type_size);
    //println!("{}", bytes_slice.slice_len);
    let v = bytes_slice.to_vec_struct::<A>().expect("");
    println!("{:#?}", v);
}

fn test_b() {
    let s: Vec<B> = vec![
        B {
            a: [0, 2, 0],
            s: "cac",
            d: vec![1],
            c: 0,
        },
        B {
            a: [0, 1, 0],
            d: vec![1],
            c: 1,
            s: "akasssssssssssssssssasdasdki",
        },
    ];

    let type_size: usize = mem::size_of::<B>();
    let mut bytes: Vec<u8> = Vec::with_capacity(type_size * s.len());

    for f in s.iter() {
        bytes.extend_from_slice(f.as_bytes());
    }

    let file = "test";
    if let Ok(_) = fs::File::create(file) {
        append_bytes(file, bytes.as_slice()).unwrap();
    }

    let bytes = read_bytes("test").expect("");
    let bytes = bytes.as_slice();

    let mut bytes_slice = BytesSlice::new(bytes, type_size);
    let v = bytes_slice.to_vec_struct_clone::<B>().expect("");
    println!("{:#?}", v);
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(packed)]
struct A<'a> {
    a: [u8; 2],
    s: &'a str,
}

#[derive(Debug, Clone)]
struct B<'a> {
    a: [u8; 3],
    c: u8,
    d: Vec<u8>,
    s: &'a str,
}

#[derive(Debug, Copy, Clone)]
struct BytesSlice<'a> {
    slice: &'a [u8],
    size: usize,
    slice_len: usize,
    max: usize,
    start: usize,
}

impl<'a> BytesSlice<'a> {
    pub fn new(slice: &'a [u8], size: usize) -> Self {
        Self {
            slice,
            size,
            slice_len: slice.len(),
            max: slice.len() / size,
            start: 0,
        }
    }
    pub fn read(&mut self) -> &[u8] {
        let s = &(self.slice[self.start..self.start + self.size]);
        //println!("{}", self.slice_len);
        (*self).start += self.size;
        //println!("{:?}", s);

        s
    }

    pub fn as_struct<T>(&mut self) -> &T
    where
        T: Sized + Copy + BytesExt,
    {
        <T as BytesExt>::from_ref(self.read())
    }

    pub fn to_vec_struct<T>(&mut self) -> Result<Vec<T>, ()>
    where
        T: Sized + Copy + BytesExt,
    {
        if self.max > 0 {
            let mut v: Vec<T> = Vec::with_capacity(self.slice_len);

            for _f in 0..self.max {
                // println!("{}", _f);

                let s = self.as_struct::<T>();
                v.push(*s);
            }

            Ok(v)
        } else {
            Err(())
        }
    }

    pub fn to_struct<T>(&mut self) -> &T
    where
        T: Sized + Clone + BytesExt,
    {
        <T as BytesExt>::from_ref(self.read())
    }

    pub fn to_vec_struct_clone<T>(&mut self) -> Result<Vec<T>, ()>
    where
        T: Sized + Clone + BytesExt,
    {
        if self.max > 0 {
            let mut v: Vec<T> = Vec::with_capacity(self.slice_len);

            for _f in 0..self.max {
                // println!("{}", _f);

                let s = self.to_struct::<T>();
                v.push(s.clone());
            }

            Ok(v)
        } else {
            Err(())
        }
    }
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

impl<'a> BytesExt for B<'a>
where
    Self: Sized + Clone,
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
    let mut f = fs::File::open(&file)?;
    let metadata = fs::metadata(&file)?;
    let mut buffer = vec![0; metadata.len() as usize];

    f.read_exact(&mut buffer)?;

    Ok(buffer)
}

pub fn append_bytes(file: &str, bytes: &[u8]) -> io::Result<()> {
    let mut f = OpenOptions::new().write(true).append(true).open(file)?;

    f.write_all(bytes)
}

// https://github.com/rust-lang/project-safe-transmute/blob/master/rfcs/0000-ext-byte-transmutation.md
//https://www.reddit.com/r/rust/comments/dw2vb3/convert_from_u8_to_generic_sized_struct/
//transmute from slice [u8] to A: 23.358µs
//transmute_copy from slice [u8] to A: 21.771µs
//convert from slice [u8] to &A: 1.426µs
