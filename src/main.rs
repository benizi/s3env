extern crate getopts;
extern crate ini;

use getopts::Options;
use std::env;
use ini::Ini;
use ini::ini::Properties;

// TODO: I feel like there's a way to combine this with the `use` above...
type IniErr = ini::ini::Error;

#[derive(Debug)]
struct S3cfg {
    access: String,
    secret: String,
}

fn ini_err(msg: &str) -> IniErr {
    IniErr{line: 0, col: 0, msg: msg.to_string()}
}

fn get_opt(section: &Properties, key: &str) -> Result<String, IniErr> {
    match section.get(key) {
        Some(s) => Ok(s.clone()),
        None => Err(ini_err(&*format!("no {} key", key))),
    }
}

fn get_s3cfg(filename: &str, section: &str) -> Result<S3cfg, IniErr> {
    let ini = try!(Ini::load_from_file(filename));
    let section = match ini.section(Some(section)) {
        Some(s) => s,
        None => return Err(ini_err("no such section")),
    };
    let access = try!(get_opt(section, "access_key"));
    let secret = try!(get_opt(section, "secret_key"));
    Ok(S3cfg{access: access.to_owned(), secret: secret.to_owned()})
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut options = Options::new();
    options.optflag("", "s3cmd", "Use s3cmd locations");
    options.optopt("i", "identity", "Identity to choose", "NAME");
    options.optopt("", "section", "Identity to choose", "NAME");

    options.optopt("a", "access", "Name of ENV var for Access Key", "NAME");
    options.optopt("s", "secret", "Name of ENV var for Secret Key", "NAME");

    options.optflag("", "dockerenv", "Print as docker run -e args");

    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    let section = match matches.opts_str(&["i".to_string(),"identity".to_string(),"section".to_string()]) {
        Some(s) => s,
        None => "default".to_owned()
    };
    let cfg = match get_s3cfg("/home/bhaskell/.s3cfg", &section[..]) {
        Ok(cfg) => cfg,
        Err(e) => { println!("Failed: {}", e.msg); std::process::exit(1) }
    };

    let accessname = match matches.opts_str(&["a".to_string(), "access".to_string()]) {
        Some(k) => k,
        None => String::from("AWS_KEY")
    };

    let secretname = match matches.opts_str(&["s".to_string(), "secret".to_string()]) {
        Some(k) => k,
        None => String::from("AWS_SECRET")
    };

    if matches.opt_present("dockerenv") {
        println!("-e {}={}", accessname, cfg.access);
        println!("-e {}={}", secretname, cfg.secret);
    } else {
        println!("{:?}", cfg);
    }
}
