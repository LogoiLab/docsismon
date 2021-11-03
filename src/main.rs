#[macro_use]
extern crate clap;
extern crate fastping_rs;
use std::thread;
use std::sync::mpsc::channel;

use clap::{App, Arg};

pub mod monitor;
pub mod parser;

arg_enum! {
    #[derive(Debug)]
    pub enum Model {
        CM1000,
        CM1100,
        SB8200
    }
}

fn launch_netgear_cm1000(ip_address: String, username: String, password: String, ping_ip: Option<std::net::IpAddr>) {

    match ping_ip {
        Some(s) => {
            let (tx, rx) = channel();
            let monitor_thread = thread::spawn(move || {
                monitor::begin(s, tx);
            });

            let parser_thread = thread::spawn(move || {
                parser::begin(ip_address, username, password, rx);
            });
            parser_thread.join().unwrap();
            monitor_thread.join().unwrap();
        },
        None => {
        }
    }
}

fn launch_arris(ip_address: String, ping_ip: Option<std::net::IpAddr>) {

}

fn main() {
    let m = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("model")
             .help("The model of the modem")
             .takes_value(true)
             .short("m")
             .long("model")
             .multiple(false)
             .required(true)
             .possible_values(&Model::variants()))
        .arg(Arg::with_name("ip_address")
             .help("The IP address of the modem (usually 192.168.100.1)")
             .takes_value(true)
             .short("i")
             .long("ip")
             .multiple(false)
             .required(true))
        .arg(Arg::with_name("username")
             .help("The username to access the modem, if not specified defaults will be used")
             .takes_value(true)
             .short("u")
             .long("user")
             .multiple(false)
             .required(false)
             .requires("password")
             .default_value("admin"))
        .arg(Arg::with_name("password")
             .help("The password to access the modem, if not specified defaults will be used")
             .takes_value(true)
             .short("p")
             .long("pass")
             .multiple(false)
             .required(false)
             .requires("username")
             .default_value("password"))
        .arg(Arg::with_name("down_detection")
             .help("Enable packet-loss based down detection by specifying an IP address. If your modem is using DHCP this should be your ISP's DHCP server. This option defaults to CloudFlare (1.1.1.1)")
             .takes_value(true)
             .short("d")
             .long("down-detect")
             .multiple(false)
             .required(false)
             .default_value("1.1.1.1"))
        .get_matches();


    let ping_ip: Option<std::net::IpAddr> = match m.value_of("down_detection") {
        Some(s) => {
            if m.is_present("down_detection") {
                Some (
                    match s.parse::<std::net::IpAddr>() {
                        Ok(o) => o,
                        Err(_) => {
                            eprintln!("Not a valid IP address for down detection!");
                            std::process::exit(1);
                        }
                    }
                )
            } else {
                None
            }
        },
        None => {
            None
        }
    };

    let ip_address: String = match m.value_of("ip_address") {
        Some(s) => {
            match s.parse::<std::net::IpAddr>() {
                Ok(_) => s.to_string(),
                Err(_) => {
                    eprintln!("Not a valid IP address!");
                    std::process::exit(1);
                }
            }
        },
        None => {
            eprintln!("IP address must be defined!");
            std::process::exit(1);
        }
    };

    let username: String = match m.value_of("username") {
        Some(s) => s.to_string(),
        None => "admin".to_string()
    };

    let password: String = match m.value_of("password") {
        Some(s) => s.to_string(),
        None => "password".to_string()
    };

    let model = value_t!(m.value_of("model"), Model).unwrap_or_else(|e| e.exit());
    match model {
        Model::CM1000 => launch_netgear_cm1000(ip_address, username, password, ping_ip),
        Model::CM1100 => println!("Found CM1100"),
        Model::SB8200 => println!("Found SB8200")//launch_arris(ip_address, ping_ip)
    }
}
