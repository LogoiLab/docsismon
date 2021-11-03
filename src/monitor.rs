use super::fastping_rs::PingResult::{Idle, Receive};
use super::fastping_rs::Pinger;

pub fn begin(ping_ip: std::net::IpAddr, tx: std::sync::mpsc::Sender<bool>) {
    let (pinger, results) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    };
    pinger.add_ipaddr(format!("{}", ping_ip).as_str());
    pinger.run_pinger();
    loop {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    println!("Host {} seemingly down. Possible disconnect.", addr);
                    tx.send(true).unwrap();
                }
                Receive { addr, rtt } => {
                    println!("Receive from Address {} in {:?}.", addr, rtt);
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }
    drop(tx);
}
