use regex::Regex;
use std::env;
use std::process;
mod func;
use colored::*;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

fn main() {
    //使用 tracing 库打日志
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    info!("start");
    // error!("only a test for tracing::error");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        error!("{}", "参数缺失,程序退出".red());
        error!("Usage: ./myfind <path> <regex>");
        process::exit(1);
    }
    let pattern = &args[2];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            // println!("Invalid regex: {}", err);
            error!("Invalid regex: {}", err);
            process::exit(1);
        }
    };
    // println!("{:?}", regex);
    // println!("searching...\n");
    info!("searching...");
    match func::search_and_find::find(&args[1], &regex) {
        Ok(matches) => {
            if matches.len() == 0 {
                info!("{}", "No matches found.".yellow());
            } else {
                println!("{}", "some matches found.".green());
                for mat in matches {
                    //将匹配到的文件路径打印出
                    //并使用colored库将与正则表达式匹配的部分用红色字体显示
                    let mat_string = mat.to_string();
                    let result = mat_string.replace(pattern, &pattern.blue().to_string());

                    println!("{}", result);
                }
            }
        }
        Err(err) => {
            // println!("Error: {}", err);
            error!("Error: {}", err);
            process::exit(1);
        }
    }
    info!("end");
}
