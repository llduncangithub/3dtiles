extern crate clap;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;

mod osgb;
mod shape;
pub mod fun_c;

use std::io::Write;
use clap::{Arg, App};
use chrono::prelude::*;
use log::{Level, LevelFilter};

fn main() {
    //     use std::io;
    //     let mut msg = String::new();
    //     io::stdin().read_line(&mut msg).unwrap();
    use std::env;
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    env::set_var("RUST_BACKTRACE", "1");

    let mut builder = env_logger::Builder::from_default_env();
    builder
        .format(|buf, record| {
            let dt = Local::now();
            let mut style = buf.style();
            if record.level() <= Level::Error {
                style.set_color(env_logger::Color::Red);
            } else {
                style.set_color(env_logger::Color::Green);
            }
            writeln!(
                buf,
                "{}: {} - {}",
                style.value(record.level()),
                dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    //env_logger::init();
    let matches = App::new("Make 3dtile program")
        .version("1.0")
        .author("fanvanzh <fanvanzh@sina.com>")
        .about("a very fast 3dtile tool")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Set the input file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Set the out file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("osgb,shape")
                .help("Set input format")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("tile config json")
                .help(
                    "Set the tile config:
{
	\"x\": x,
	\"y\": y,
	\"offset\": 0 (模型最低面地面距离),
	\"max_lvl\" : 20 (处理切片模型到20级停止)
}",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Set output verbose ")
                .takes_value(false),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let format = matches.value_of("format").unwrap();
    let tile_config = matches.value_of("config").unwrap_or("");

    if matches.is_present("verbose") {
        info!("set program versose on");
    }
    let in_path = std::path::Path::new(input);
    if !in_path.exists() {
        error!("{} does not exists.", input);
        return;
    }
    match format {
        "osgb" => {
            convert_osgb(input, output, tile_config);
        }
        _ => {
            error!("not support now.");
        }
    }
}

fn convert_osgb(src: &str, dest: &str, config: &str) {
    use std::time;
    use serde_json::Value;
    use std::fs::File;
    use std::io::prelude::*;

    let dir = std::path::Path::new(src);
    let dir_dest = std::path::Path::new(dest);

    let mut center_x = 0f64;
    let mut center_y = 0f64;
    let mut max_lvl = None;
    let mut trans_region = None;

    // try parse metadata.xml
    let metadata_file = dir.join("metadata.xml");
    if metadata_file.exists() {
        // read and parse
        if let Ok(mut f) = File::open(metadata_file) {
            let mut buffer = String::new();
            if let Ok(_) = f.read_to_string(&mut buffer) {
                let pos0 = buffer.find("<SRS>");
                let pos1 = buffer.find("</SRS>");
                if pos0.is_some() && pos1.is_some() {
                    let vec = (&buffer).as_bytes()[(pos0.unwrap() + 5)..(pos1.unwrap())].to_vec();
                    let str1 = String::from_utf8(vec).unwrap();
                    info!("center point --> {}.", str1);
                    let v: Vec<&str> = str1.split(":").collect();
                    if v.len() > 1 {
                        let v1: Vec<&str> = v[1].split(",").collect();
                        if v1.len() > 1 {
                            let v1_num = (*v1[0]).parse::<f64>();
                            let v2_num = v1[1].parse::<f64>();
                            if v1_num.is_ok() && v2_num.is_ok() {
                                center_y = v1_num.unwrap();
                                center_x = v2_num.unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
    if let Ok(v) = serde_json::from_str::<Value>(config) {
        if let Some(x) = v["x"].as_f64() {
            center_x = x;
        }
        if let Some(y) = v["y"].as_f64() {
            center_y = y;
        }
        if let Some(h) = v["offset"].as_f64() {
            trans_region = Some(h);
        }
        if let Some(lvl) = v["max_lvl"].as_i64() {
            max_lvl = Some(lvl as i32);
        }
    } else if config.len() > 0 {
        error!("config error --> {}", config);
    }
    let tick = time::SystemTime::now();
    if let Err(e) = osgb::osgb_batch_convert(
        &dir,
        &dir_dest,
        max_lvl,
        center_x,
        center_y,
        trans_region,
    )
    {
        error!("{}", e.description());
        return;
    }
    let elap_sec = tick.elapsed().unwrap();
    let tick_num = elap_sec.as_secs() as f64 + elap_sec.subsec_nanos() as f64 * 1e-9;
    info!("task over, cost {:.2} s.", tick_num);
}
