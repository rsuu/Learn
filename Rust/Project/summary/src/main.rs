use std::io::{self, Read};

fn main() {
    static t: char = '/';
    static space: &str = "    ";
    static summary: &str = "- [SUMMARY.md](SUMMARY.md)";

    // Read from the buffer and convert the input to Vec<&str>
    // 读取缓冲区的内容并将其转换成 Vec<&str>
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer = buffer.replace("./", "");
    let s = buffer.split('\n').collect::<Vec<&str>>();

    let mut body: String = "".to_string();

    // Discard the last line(s.len() - 1)
    // 移除最后一行
    for num in 0..s.len() - 1 {
        let depth = s[num].matches(t).count(); // /a/b/c/d = 4  /a/b/c = 3
        let mut text = String::from("-    ");

        // Append 4 spaces at the end of p
        // 在 p 的末尾追加4个空格
        for _ in 0..depth {
            text.push_str(space);
        }

        // Reverse the string after concatenating the string
        // 拼接字符串后反转字符串
        let aa = text.chars().rev().collect::<String>();
        let bb = s[num].rsplit(t).collect::<Vec<_>>()[0].to_string();

        let mut can_push = false;

        // Treat it as a file if the file name contains the character $
        // 如果文件名当中包含 . 这个字符  则视其为文件
        if s[num].contains('.') {
            if s[num].contains(".md") || s[num].contains(".txt") {
                can_push = true;
            }

            if can_push {
                body.push_str(&format!("{} [{}](./{})\n", aa, bb, s[num]));
            }
        }
        // Else, treat it as a directory
        // 否则  视其为目录
        else {
            body.push_str(&format!("{} [{}]()\n", aa, bb));
        }
    }

    let output = format!("{}\n{}", summary, body);
    println!("{}", output);
}
