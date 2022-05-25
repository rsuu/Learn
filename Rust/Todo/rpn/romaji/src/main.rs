use std::{fs, io};

mod ja_map;

pub struct Input {
    romaji_input: String,
    romaji_len: usize,
    hinakara: bool,
}

pub fn romaji_input() -> Input {
    let args: Vec<String> = std::env::args().collect();
    let romaji_input: String;
    let romaji_len: usize;
    let hinakara = true;

    match args[1].as_str() {
        "-" => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            romaji_input = buffer;
            romaji_len = romaji_input.len() - 1;
        }

        "-i" => {
            let file = fs::read_to_string(args[2].as_str()).expect("No such file");
            romaji_input = file;
            romaji_len = romaji_input.len() - 1;
        }

        _ => {
            romaji_input = args[1].to_string();
            romaji_len = args[1].len();
        }
    }

    Input {
        romaji_input,
        romaji_len,
        hinakara,
    }
}

pub fn romaji_convert(romaji_split: Vec<&str>, input: &Input) -> String {
    let mut ptr: usize = 0;
    let mut _ptr1: usize;
    let mut _ptr2: usize;
    let mut kana: String;
    let mut output = String::new();

    for _i in ptr..input.romaji_len {
        if _i >= ptr {
            _ptr1 = ptr + 1;
            _ptr2 = ptr + 2;

            // 处理第一个假名
            match romaji_split[_ptr1] {
                "a" | "i" | "u" | "e" | "o" => {
                    _ptr1 = ptr + 1;
                    ptr = ptr + 1;
                    kana = format!("{}", romaji_split[_ptr1]);
                }

                // 处理第二个假名
                _ => match romaji_split[_ptr2] {
                    "a" | "i" | "u" | "e" | "o" | "n" => {
                        _ptr1 = ptr + 1;
                        _ptr2 = ptr + 2;
                        ptr = ptr + 2;
                        kana = format!("{}{}", romaji_split[_ptr1], romaji_split[_ptr2]);
                    }

                    // 处理第二个假名及双辅音
                    /*
                    "k" | "s" | "h" | "m" | "y" | "r" | "w" => {
                        // no "t"
                        _ptr1 = ptr + 1;
                        _ptr2 = ptr + 2;
                        ptr = ptr + 3;
                        let xtsu: &str = "っ";
                        kana = format!("{}{}", romaji_split[_ptr2], romaji_split[_ptr2 + 1]);

                        output.push_str(xtsu);
                    }
                    */

                    // 处理第三个假名
                    _ => match romaji_split[_ptr2 + 1] {
                        "a" | "i" | "u" | "e" | "o" | "n" => {
                            _ptr1 = ptr + 1;
                            _ptr2 = ptr + 2;
                            ptr = ptr + 3;
                            kana = format!(
                                "{}{}{}",
                                romaji_split[_ptr1],
                                romaji_split[_ptr2],
                                romaji_split[_ptr2 + 1]
                            );
                        }

                        // 处理第三个假名及促音っ{xtsu}
                        "s" => {
                            _ptr1 = ptr + 1;
                            _ptr2 = ptr + 2;
                            ptr = _i + 4;
                            kana = format!(
                                "{}{}{}{}",
                                romaji_split[_ptr2 - 1],
                                romaji_split[_ptr2],
                                romaji_split[_ptr2 + 1],
                                romaji_split[_ptr2 + 2],
                            );
                        }

                        // 其他
                        _ => {
                            kana = "".to_string();
                        }
                    },
                },
            }

            output.push_str(ja_map::hinagana_map(&kana));
        }
    }
    output
}

fn main() {
    let input = romaji_input();

    let romaji_split: Vec<&str> = input.romaji_input.split("").collect();

    println!("{}", romaji_convert(romaji_split, &input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_romaji_1() {
        let input = Input {
            romaji_input: "a".to_string(),
            romaji_len: 1,
            hinakara: true,
        };

        let mut romaji_output: Vec<&str> = vec![];

        romaji_output.extend(&[ja_map::hinagana_map(&input.romaji_input)]);

        assert_eq!(romaji_output, ["あ"]);
    }

    #[test]
    fn test_romaji_2() {
        let input = Input {
            romaji_input: "nokkaxtsuaiueonnrrann".to_string(),
            romaji_len: 21,
            hinakara: true,
        };

        let romaji_split: Vec<&str> = input.romaji_input.split("").collect();

        let c = romaji_convert(romaji_split, &input);

        assert_eq!(c, "のっかっあいうえおんっらん");
    }
}
