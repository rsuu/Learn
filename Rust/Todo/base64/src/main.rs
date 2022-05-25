fn main() {
    println!("{}", base64_en("🍉🍇🍑🍓🥝"))
}

fn base64_en(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut binary = String::with_capacity(bytes.len() * 8);

    for f in bytes.iter() {
        binary.push_str(format!("{:08b}", f).as_str());
    }

    let binary_code = binary
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 6 == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>();

    let base64 = binary_code
        .split(' ')
        .map(|f| {
            if f.len() == 6 {
                table(f.parse::<u32>().expect(""))
            } else {
                // f.len() == 4
                table(format!("{}00", f).parse::<u32>().expect(""))
            }
        })
        .collect::<Vec<char>>();

    let mut res = String::with_capacity(base64.len() + 8);

    for f in base64.iter() {
        res.push(*f);
    }
    match base64.len() % 4 {
        0 => {}
        2 => {
            res.push('=');
            res.push('=');
        }
        3 => {
            res.push('=');
        }
        _ => panic!(),
    }

    res
}

fn table(n: u32) -> char {
    match n {
        000000 => 'A',
        000001 => 'B',
        000010 => 'C',
        000011 => 'D',
        000100 => 'E',
        000101 => 'F',
        000110 => 'G',
        000111 => 'H',
        001000 => 'I',
        001001 => 'J',
        001010 => 'K',
        001011 => 'L',
        001100 => 'M',
        001101 => 'N',
        001110 => 'O',
        001111 => 'P',
        010000 => 'Q',
        010001 => 'R',
        010010 => 'S',
        010011 => 'T',
        010100 => 'U',
        010101 => 'V',
        010110 => 'W',
        010111 => 'X',
        011000 => 'Y',
        011001 => 'Z',
        011010 => 'a',
        011011 => 'b',
        011100 => 'c',
        011101 => 'd',
        011110 => 'e',
        011111 => 'f',
        100000 => 'g',
        100001 => 'h',
        100010 => 'i',
        100011 => 'j',
        100100 => 'k',
        100101 => 'l',
        100110 => 'm',
        100111 => 'n',
        101000 => 'o',
        101001 => 'p',
        101010 => 'q',
        101011 => 'r',
        101100 => 's',
        101101 => 't',
        101110 => 'u',
        101111 => 'v',
        110000 => 'w',
        110001 => 'x',
        110010 => 'y',
        110011 => 'z',
        110100 => '0',
        110101 => '1',
        110110 => '2',
        110111 => '3',
        111000 => '4',
        111001 => '5',
        111010 => '6',
        111011 => '7',
        111100 => '8',
        111101 => '9',
        111110 => '+',
        111111 => '/',
        _ => panic!(),
    }
}

mod test {
    use crate::base64_en;

    #[test]
    fn test() {
        assert_eq!(
            "6YGT5Y+v6YGT6Z2e5bi46YGT",
            base64_en("道可道非常道").as_str()
        );
        assert_eq!(
            "8J+NifCfjYfwn42R8J+Nk/CfpZ0=",
            base64_en("🍉🍇🍑🍓🥝").as_str()
        );

        assert_eq!("aA==", base64_en("h").as_str());

        assert_eq!("zrHOss6zzrQ=", base64_en("αβγδ").as_str());
    }
}
