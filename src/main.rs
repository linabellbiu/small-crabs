use std::{env, process};

use small_crabs::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("输入的参数错误: {}", err);
        process::exit(1);
    });

    println!("搜索:{}", config.query);
    println!("在这个文件:{}", config.filename);

    if let Err(err) = run(config) {
        println!("发生错误: {}", err);
        process::exit(1);
    }
}

