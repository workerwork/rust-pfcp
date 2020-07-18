use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;
use std::net::UdpSocket;
use yaml_rust::yaml;

pub fn get_args() -> (UdpSocket,) {
    let matches = App::new("UPF PFCP Controller")
        .version("1.0")
        .author("wow d. <workerwork@qq.com>")
        .about("UPF PFCP Controller")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        //.arg(Arg::with_name("INPUT")
        //     .help("Sets the input file to use")
        //     .required(true)
        //     .index(1))
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    // println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => {
            println!("No verbose info");
        }
        1 => {
            println!("Some verbose info");
            //exit(0);
        }
        2 => {
            println!("Tons of verbose info");
            //exit(0);
        }
        3 | _ => {
            println!("Don't be crazy");
            //exit(0);
        }
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
    let mut f = File::open(config).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
    let ip = &docs[0]["servers"][0]["server"]["ip"].as_str().unwrap();
    let port = &docs[0]["servers"][0]["server"]["port"].as_str().unwrap();
    let addr = format!("{}:{}", ip, port);
    let socket = UdpSocket::bind(addr).unwrap();
    (socket,)
    // more program logic goes here...
}
