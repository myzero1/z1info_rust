//! Add extra information to the binary package through `cargo run [...] z1info=extra_information`
//! 
//! # Use
//! 
//! ### add code
//! 
//! ```no_run
//! fn main() {
//!     z1info_rust::run("z1template");
//!     // z1info_test::run("z1_info:{z1_info},git_info:{git_info},build_time:{build_time}");
//! }
//! ```
//! ### run command
//! 
//! `z1info=` must be placed at the end of the command line,`cargo run`will write extra information to tmp file.
//! ```
//! cargo run p1 p2 z1info=version:1.2.3,compiler:myzero1
//! ```
//! ### build
//! ```
//! cargo build
//! cargo build --release
//! ```
//! 
//! # Result
//! ### run command or run `builded binary package`
//! 
//! ```
//! cargo run  p1 p2
//! or run builded binary package
//! ```
//! 
//! ```no_run
//! 
//! =============== z1info extended data ====================
//! | Extended data added to binary file through z1info.
//! |--------------- z1info parameter ----------------------
//! | z1info=version:1.2.3,compiler:myzero1
//! |--------------- git info ------------------------------
//! | commit id: 94896476ea1696f9b8764cd845f225e4af586bc4
//! |--------------- build time ----------------------------
//! | 2021-05-22 18:33:57
//! =========================================================
//! 
//! ```

use std::io::Write;
use chrono::{Local};
use std::process::{Command, Stdio};

pub fn run(template: &str){
    let args :Vec<String> = std::env::args().collect();
    
    if !is_runtime(&args){
        let mut content_template = "
            =============== z1info extended data ====================
            | Extended data added to binary file through z1info.
            |--------------- z1info parameter ----------------------
            | {z1_info}
            |--------------- git info ------------------------------
            | commit id: {git_info}
            |--------------- build time ----------------------------
            | {build_time}
            =========================================================
        ";

        if "z1template"!=template {
            content_template = template;
        }

        let mut content = str::replace(content_template,"{z1_info}",&args[args.len()-1]);
        content = str::replace(&content[..],"{git_info}",&get_commit_id());
        content = str::replace(&content[..],"{build_time}",&format!("{}",Local::now().naive_local().format("%Y-%m-%d %H:%M:%S").to_string()));

        let mut file = std::fs::File::create("z1info_tmp").unwrap();
        file.write_all(content.as_bytes()).expect("write z1info failed");
    } else {
        println!("{}",include_str!("../z1info_tmp"));
    }
}

fn is_runtime(args: &Vec<String>)->bool{
    let lenght = args.len();

    if 0 < lenght {
        let flag = "z1info=";
        let flag_length = flag.len();
        let last_length = args[lenght-1].len();
        if last_length >= flag_length {
            return !(flag == &args[lenght-1][0..flag_length]);
        }
    }

    return true;
}

fn get_commit_id()->String{
    // cmd_str可以是从输入流读取或从文件里读取
    let cmd_str = "git rev-parse HEAD";
    let out_str: String;

    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
        .arg("/c")
        .arg(&cmd_str)
        .stdout(Stdio::piped())
        .output()
        .expect(&format!("cmd exec error!"));

        out_str = format!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
       let output= Command::new("sh")
        .arg("/c")
        .arg(&cmd_str)
        .stdout(Stdio::piped())
        .output()
        .expect(&format!("sh exec error!"));

        out_str = format!("{}", String::from_utf8_lossy(&output.stdout));
    };

    let ret_end = str::replace(&out_str[..],"\n","");

    return ret_end;
}