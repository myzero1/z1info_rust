//! Add extra information to the binary package through `cargo run [...] z1info=extra_information`
//!
//! # Use
//!
//! ### add code
//! 
//! ### add dependencies
//! - z1info_rust = "0.1.3"
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
//! - `cargo run p1 p2 z1info=version:1.2.3,compiler:myzero1`
//! ### build
//! - cargo clean && cargo build
//! - OR cargo clean && cargo build --release
//!
//! # Result
//! ### run command or run `builded binary package`
//! - cargo clean  && cargo run  p1 p2
//! - OR run builded binary package
//!
//! ### The Result
//!
//! <br/>=============== z1info extended data ====================
//! <br/>| Extended data added to binary file through z1info.
//! <br/>|--------------- z1info parameter ----------------------
//! <br/>| z1info=version:1.2.3,compiler:myzero1
//! <br/>|--------------- git info ------------------------------
//! <br/>| commit id: 94896476ea1696f9b8764cd845f225e4af586bc4
//! <br/>|--------------- build time ----------------------------
//! <br/>| 1621770625
//! <br/>=========================================================
//!

use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

/// Add or display additional information
///
/// # Examples
///
/// ```no_run
/// z1info_rust::run("z1template");
/// // z1info_test::run("z1_info:{z1_info},git_info:{git_info},build_time:{build_time}");
/// ```
///
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
        content = str::replace(&content[..],"{build_time}",&format!("{}",get_current_timestamp()));

        // let mut file = std::fs::File::create("z1info_tmp").unwrap();
        // file.write_all(content.as_bytes()).expect("write z1info failed");

        write_to_tmp(&content,)
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

fn get_current_timestamp()->u64{
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    return since_the_epoch.as_secs();
}

fn path_exist(path: &str) -> bool {
    return match std::path::Path::new(path).canonicalize() {
        Ok(_buf) => true,
        Err(_error) => false,
    };
}

fn write_to_tmp(content: &str) {
    let cargo_toml = format!("{:?}", std::fs::read_to_string("Cargo.toml"));
    let mut write_to_crate = true;

    let cargo_toml_info: Vec<&str> = cargo_toml.split("z1info_rust = ").collect();
    if cargo_toml_info.len()>1{
        let cargo_toml_info2: Vec<&str> = cargo_toml_info[1].split("\\\"").collect();

        if cargo_toml_info2.len()>1{
            let cargo_toml_info2: Vec<&str> = cargo_toml_info[1].split("\\\"").collect();
            let mut cargo_registry = String::from("");

            match std::env::var("CARGO_HOME") {
                Ok(val) => cargo_registry = format!("{}/registry/src", val),
                Err(_e) => println!("{:?}",_e),
            }

            let paths = std::fs::read_dir(cargo_registry).unwrap();
            for path in paths {
                let tmp_file = &format!("{}/z1info_rust-{}/z1info_tmp", path.unwrap().path().display(),cargo_toml_info2[1])[..];

                if path_exist(tmp_file){
                    let mut file = std::fs::File::create(tmp_file).unwrap();
                    file.write_all(content.as_bytes()).expect("write z1info failed");

                    write_to_crate = false;
                }
            }
        }
    }

    if write_to_crate{
        let mut file = std::fs::File::create("z1info_tmp").unwrap();
        file.write_all(content.as_bytes()).expect("write z1info failed");
    }
}
