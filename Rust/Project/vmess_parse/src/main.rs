// curl xxx | vmess_parse -
// curl http | base64 -d | cut vemss.* | base64 -d | jq | json_parse

use base64::{decode, encode};
use std::{
    fs::File,
    io::{self, prelude::*, BufReader, Write},
    path::Path,
    time::SystemTime,
};
use tinyjson::JsonValue;

#[derive(Debug)]
pub struct Vmess<'a> {
    pub add: &'a str,
    pub host: &'a str,
    pub id: &'a str,
    pub net: &'a str,
    pub port: u32,
    pub aid: u32,
    pub protocol: &'a str,
    pub path: &'a str,
}

fn main() {
    let f01 = include_bytes!("./01_log.json");
    let f02 = include_bytes!("./02_dns.json");
    let f03 = include_bytes!("./03_inbounds.json");
    let f04 = include_bytes!("./04_outbounds.json");
    let f05 = include_bytes!("./05_routing.json");
    let args: Vec<_> = std::env::args().collect();
    let mut link = String::new();

    if args.len() >= 2 {
        if &args[1] == "-" {
            io::stdin().read_to_string(&mut link).unwrap();
            link = link.replace('\n', "");
        } else {
            link = args[1].clone();
        }

        let link = base64_de(&link);
        let links = remove_empty_lines(&link);
        let null = "".to_string();
        //println!("{:?}", links);

        for f in links.into_iter() {
            //println!("{}", f);
            if let Some(body) = parse_vmess(f) {
                // dorp "vmess://" && decode base64

                let json: JsonValue = body.parse().expect("1"); // parse json

                let mut port: u32 = 443;
                if let Some(p) = json["port"].get::<f64>() {
                    port = *p as u32;
                } else if let Some(p) = json["port"].get::<String>() {
                    // convert String to u32
                    // remove "
                    // e.g.
                    // port: "443" -> port: 443
                    port = p.replace('"', "").parse::<u32>().expect("");
                }

                let mut aid: u32 = 0;
                if let Some(p) = json["aid"].get::<f64>() {
                    aid = *p as u32;
                } else {
                }

                let mut protocol = "vmess";
                if body.find("protocol").is_some() {
                    if let Some(p) = json["protocol"].get::<String>() {
                        protocol = p.as_str();
                    } else {
                    }
                } else {
                }

                let mut path = "/";
                if body.find("path").is_some() {
                    if let Some(p) = json["path"].get::<String>() {
                        path = p.as_str();
                    } else {
                    }
                } else {
                }

                let vmess = Vmess {
                    add: (json["add"].get::<String>().expect("")),
                    host: (json["host"].get::<String>().expect("")),
                    id: (json["id"].get::<String>().expect("")),
                    net: (json["net"].get::<String>().expect("")),
                    protocol,
                    port,
                    aid,
                    path,
                };

                let mut outbounds = String::from_utf8_lossy(f04).to_string();
                outbounds = outbounds
                    .replace("$ip", format!("\"{}\"", vmess.host).as_str())
                    .replace("$port", format!("{}", vmess.port).as_str())
                    .replace("$aid", format!("{}", vmess.aid).as_str())
                    .replace("$address", format!("\"{}\"", vmess.add).as_str())
                    .replace("$users_id", format!("\"{}\"", vmess.id).as_str())
                    .replace("$host", format!("\"{}\"", vmess.host).as_str())
                    .replace("$network", format!("\"{}\"", vmess.net).as_str())
                    .replace("$protocol", format!("\"{}\"", vmess.protocol).as_str())
                    .replace("$path", format!("\"{}\"", vmess.path).as_str());

                let output = format!(
                    "{{ {s1} {s2} {s3} {s4} {s5} }}",
                    s1 = String::from_utf8_lossy(f01),
                    s2 = String::from_utf8_lossy(f02),
                    s3 = String::from_utf8_lossy(f03),
                    s4 = outbounds,
                    s5 = String::from_utf8_lossy(f05)
                );

                if args.len() >= 3 {
                    save_to_file(
                        &args[2],
                        &format!("{}_{}", vmess.add, get_time_ns()),
                        &output,
                    )
                    .unwrap()
                } else {
                    io::stdout().write_all(output.as_bytes()).expect("stdout");
                }
            }
        }
    }
}

pub fn remove_first_and_last(value: &str) -> Vec<char> {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.collect::<Vec<char>>()
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>()
}

fn lines_from_text(s: &str) -> Vec<String> {
    s.split(',').map(|s| s.to_string()).collect()
}

fn base64_de(s: &str) -> String {
    String::from_utf8(decode(s).expect("base64_de")).expect("base64_de -> utf8")
}

fn base64_en(s: &str) -> String {
    encode(s.as_bytes())
}

fn parse_vmess(s: &str) -> Option<String> {
    if s.starts_with("vmess://") {
        Some(base64_de(s.split("vmess://").collect::<Vec<_>>()[1]))
    } else {
        None
    }
}

fn remove_empty_lines(s: &str) -> Vec<&str> {
    s.lines().filter(|l| !l.is_empty()).collect()
}

fn save_to_file(filepath: &str, filename: &str, text: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}/{}.json", filepath, filename.replace('"', "")))?; // remove double quote
    file.write_all(text.as_bytes())
}

fn split_and_return(s: &str) -> String {
    s.split(':').collect::<Vec<_>>()[1].to_string()
}

fn get_time_ns() -> u128 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error");
    now.as_nanos()
}
