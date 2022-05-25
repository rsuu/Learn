use async_recursion::async_recursion;
use futures::future::join_all;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;

use std::{
    env,
    error::Error,
    fs::{self, File},
    io,
    path::PathBuf,
    process,
    time::SystemTime,
};

#[tokio::main]
async fn main() {
    let config: Config = Config::read_config().await.unwrap();
    if let Err(e) = run(config).await {
        eprintln!("运行异常 {}", e);
        process::exit(1)
    }
}

#[derive(Debug)]
pub struct Config {
    pub thread_num: u64,
    pub uri: String,
    pub path: String,
    pub file_name: String,
}

// https://stackoverflow.com/questions/66864010/future-created-by-async-block-is-not-send
unsafe impl Send for Config {}
unsafe impl Sync for Config {}

pub type Result<T, E = Box<dyn Error>> = core::result::Result<T, E>;

impl Config {
    pub async fn read_config() -> Result<Self, &'static str> {
        let mut args = env::args();
        if args.len() < 5 {
            return Err("参数小于 4 位");
        }
        args.next();

        let thread_num = match args.next() {
            Some(arg) => arg.parse::<u64>().unwrap(),
            None => return Err("参数 block_size 不存在"),
        };

        let uri = match args.next() {
            Some(arg) => arg,
            None => return Err("参数 uri 不存在"),
        };
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("参数 path 不存在"),
        };
        let file_name = match args.next() {
            Some(arg) => {
                if PathBuf::from(&arg).is_file() {
                    eprintln!("has file");
                    process::exit(-1);
                } else {
                    arg
                }
            }
            None => return Err("参数 file_name 不存在"),
        };

        let uri = redirected_url(&uri).await.unwrap_or(uri);

        Ok(Self {
            thread_num,
            uri,
            path,
            file_name,
        })
    }
    pub fn set_uri(&mut self, url: &String) {
        self.uri = url.to_string();
    }
}

pub async fn redirected_url(url: &str) -> Option<String> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::builder()
        .method(Method::HEAD)
        .uri(url)
        .body(Body::empty())
        .unwrap();
    let response = client.request(request).await.unwrap();
    let headers = response.headers();
    let content_length = headers.get("content-length");
    println!("{:#?}", content_length.unwrap());
    let redirected_url = headers.get("Location");

    if let Some(uri) = redirected_url {
        println!("{:?}", uri);
        Some(uri.to_str().unwrap().to_string())
        // accept_ranges = Some(v);
        // std::process::exit(0)
    } else {
        None
    }
}

/// 根据数据开始、结束索引分段下载文件。
/// 文件名称后缀为 index。如：test.zip1。
async fn download_block(
    index: u64,
    start: u64,
    end: u64,
    config: &Config,
    client: &Client<HttpsConnector<HttpConnector>, Body>,
    to: &str,
) -> Result<()> {
    let request = Request::builder()
        .method(Method::GET)
        .header("range", format!("bytes={}-{}", start, end))
        .uri(config.uri.clone())
        .body(Body::empty())?;

    let response = client.request(request).await?;
    let bytes = hyper::body::to_bytes(response).await?;

    write_file(&bytes, format!("{}/{}", to, index).as_str()).expect("");

    Ok(())
}

fn get_time_ns() -> u128 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error");
    now.as_nanos()
}

/// 写入文件
fn write_file(from: &hyper::body::Bytes, to: &str) -> io::Result<()> {
    fs::write(to, from.iter())
}

/// 合并文件
fn merge(
    size: usize,
    numbers: usize,
    config: &Config,
    tmp_path: &str,
    file_path: &str,
) -> Result<()> {
    let mut to = File::create(&file_path)?;
    let mut from: Vec<String> = Vec::with_capacity(numbers);

    for f in 0..=numbers {
        from.push(format!("{}/{}", tmp_path, f));
    }

    concat_files(from.as_slice(), &mut to)?;
    fs::remove_dir_all(tmp_path)?; // 删除临时文件
    eprintln!("Done");
    Ok(())
}

fn concat_files(from: &[String], to: &mut File) -> io::Result<()> {
    for i in from {
        let mut f = File::open(i)?;
        io::copy(&mut f, to)?;
    }
    //eprintln!("Done");
    Ok(())
}

// async_recursion用来做异步递归
//#[async_recursion]
pub async fn run(config: Config) -> Result<()> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::builder()
        .method(Method::HEAD)
        .uri(&config.uri)
        .body(Body::empty())?;
    let response = client.request(request).await?;
    let headers = response.headers();
    let accept_ranges = headers.get("accept-ranges");
    // println!("{:#?}", accept_ranges.unwrap());
    let content_length = headers.get("content-length");
    println!("{:#?}", content_length.unwrap());
    // let v = &HeaderValue::from_str("bytes").unwrap();
    let accept_ranges_flag = match accept_ranges {
        None => false,
        Some(v) => v.to_str()?.eq("bytes"),
    };

    if accept_ranges_flag & &content_length.clone().is_some() {
        // true & tre = true
        let size = content_length.unwrap().to_str()?.parse::<u64>()?;
        if size == 0 {
            println!("数据为空");
            return Ok(());
        }

        let dir = fs::canonicalize(&config.path.as_str())?
            .display()
            .to_string();
        let tmp = format!(".tmp_{}", get_time_ns());
        let tmp_path = format!("{}/{}", &dir, &tmp);
        let file_path = format!("{}/{}", &dir, &config.file_name.as_str());

        let thread_num = config.thread_num;
        let block_size = size / config.thread_num;
        let head_block = size % block_size + block_size - 1;

        std::fs::create_dir(&tmp_path).expect("");

        // if size < 1MB
        if thread_num == 1 || size < (102400) {
            download_block(0, 0, size - 1, &config, &client, &tmp_path)
                .await
                .expect("");
            fs::rename(format!("{}/0", &tmp_path), &file_path).expect("");
            fs::remove_dir(&tmp_path);

            eprintln!("Done");

            return Ok(());
            // Done
        } else {
            // 多线程
            //println!("支持并发下载\n{}", size);
            //println!("数据块长度 {}", block_size);
            //println!("启用 {} 个线程下载", thread_num);

            let mut run = Vec::new();

            download_block(0, 0, head_block, &config, &client, &tmp_path)
                .await
                .expect(""); // download head_block first
            for i in 1..=thread_num {
                run.push(new_thread(
                    i, head_block, block_size, &config, &client, &tmp_path,
                ));
            }

            join_all(run).await; // run

            //println!("下载完成，开始合并文件");

            merge(
                size as usize,
                ((size - head_block) / block_size) as usize,
                &config,
                &tmp_path,
                &file_path,
            )?;
            // Done
        }
    } else {
        eprintln!("不支持下载");
    }
    Ok(())
}

async fn new_thread(
    i: u64,
    first_attach: u64,
    block_size: u64,
    config: &Config,
    client: &Client<HttpsConnector<HttpConnector>, Body>,
    to: &str,
) {
    let start = i * block_size + first_attach;
    println!("线程 {} 启动", i);
    download_block(i, start, start + block_size - 1, config, client, to)
        .await
        .expect("");
}

// REF
// https:stackoverflow.com/questions/65573245/how-can-i-delete-all-the-files-in-a-directory-but-not-the-directory-itself-in-ru
