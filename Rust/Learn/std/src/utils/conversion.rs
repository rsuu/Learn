use std::str;

pub struct Conversion {}

impl Conversion {
    pub fn vec_of_chars() {
        // -- FROM: Vec<char> --
        let src1: Vec<char> = vec!['j', '{', '"', 'i', 'm', 'm', 'y', '"', '}'];

        // to String
        let string1: String = src1.iter().collect::<String>();

        // to &str
        let str1: &str = &src1.iter().collect::<String>();

        // to Vec<u8>
        let byte1: Vec<u8> = src1.iter().map(|c| *c as u8).collect::<Vec<_>>();
        println!(
            "Vec<char>:{:?},
        String:{:#?},
        str:{:#?},
        Vec<u8>:{:?}",
            src1, string1, str1, byte1
        );
    }

    pub fn vec_of_bytes() {
        // -- FROM: Vec<u8> --
        // b - byte
        // r - raw string
        // br - byte of raw string
        let src2: Vec<u8> = br#"e{"ddie"}"#.to_vec();

        // to String
        // from_utf8 consume the vector of bytes
        let string2: String = String::from_utf8(src2.clone()).unwrap();

        // to &str
        let str2: &str = str::from_utf8(&src2).unwrap();

        // to Vec<u8>
        let char2: Vec<char> = src2.iter().map(|b| *b as char).collect::<Vec<_>>();
        println!(
            "Vec<u8>:{:?},
        String:{:#?},
        str:{:#?},
        Vec<char>:{:?}",
            src2, string2, str2, char2
        );
    }

    pub fn from_string() {
        // -- FROM: String --
        let src3: String = String::from(r#"o{"livia"}"#);
        let str3: &str = &src3;
        let char3: Vec<char> = src3.chars().collect::<Vec<_>>();
        let byte3: Vec<u8> = src3.as_bytes().to_vec();
        println!(
            "String:{:#?},
        str:{:#?},
        Vec<char>:{:?},
        Vec<u8>:{:?}",
            src3, str3, char3, byte3
        );
    }

    pub fn from_str() {
        // -- FROM: &str --
        let src4: &str = r#"g{'race'}"#;
        let string4 = String::from(src4);
        let char4: Vec<char> = src4.chars().collect();
        let byte4: Vec<u8> = src4.as_bytes().to_vec();
        println!(
            "str:{:#?}
        String:{:#?},
        Vec<char>:{:?},
        Vec<u8>:{:?}",
            src4, string4, char4, byte4
        );
    }

    pub fn str_to_number() {
        let str_to_number_u8: u8 = "42".parse::<u8>().unwrap();
        let str_to_number_i32 = "-42".parse::<i32>().unwrap();
        println!(
            "str_to_number_u8: {:?}
        str_to_number_i32: {:?}",
            str_to_number_u8, str_to_number_i32
        );
    }

    pub fn string_to_number() {
        let string_to_number_f32 = "11.9".to_string().parse::<f32>().unwrap();
        println!("string_to_number_f32: {:?}", string_to_number_f32);
    }

    pub fn str_string_to_number() {
        let str_string_to_number_i32: i32 = "-11".trim().parse().unwrap();
        let str_string_to_number_u32: u32 = "11".to_string().trim().parse().unwrap();
        println!(
            "str_string_to_number_i32: {:?}
        str_string_to_number_u32: {:?}",
            str_string_to_number_i32, str_string_to_number_u32
        );
    }
}
