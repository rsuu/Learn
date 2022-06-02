use speedy::{Endianness, Readable, Writable};
use std::mem::size_of;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Writable, Readable, Clone)]
struct A<'a> {
    number: u64,     // 8
    float: f32,      // 4
    string: String,  // 24
    vector: Vec<u8>, // 24
    strs: &'a str,   // 16
    uuid: Uuid,      // 16
                     //t_su8: &'a [u8], // 16
                     //strs2: &'a str,  // 16
}
fn main() {
    println!(
        "\
String: {}
&str: {}
u64: {}
vec<u8>: {}
f32: {}
&[u8]: {}
",
        size_of::<String>(),
        size_of::<&str>(),
        size_of::<u64>(),
        size_of::<Vec<u8>>(),
        size_of::<f32>(),
        size_of::<&[u8]>(),
    );

    let uuid = Uuid::new_v4();
    let v = vec![
        A {
            strs: "aaa",
            float: 0.0,
            number: 0x12345678ABCDEF00,
            string: "A totally pointless string".to_owned(),
            vector: vec![1, 2, 3],
            //t_su8: &[1, 2, 3],
            //strs2: "aaa",
            uuid,
        },
        A {
            strs: "aaa",
            float: 0.0,
            number: 0x12345678ABCDEF00,
            string: "A totally pointless string".to_owned(),
            vector: vec![1, 2, 3],
            //t_su8: &[1, 2, 3],
            //strs2: "aaa",
            uuid,
        },
    ];
    let bytes = v.write_to_vec().unwrap();
    println!("{:?}", bytes);
    println!("{:?}", bytes.len());
    let b = A::read_from_buffer(&bytes).unwrap();

    let time = OffsetDateTime::now_utc();

    println!("{:#?}", time);
}
