pub fn begin(ip_address: String, username: String, password: String, rx: std::sync::mpsc::Receiver<bool>) {
    let query_time = std::time::Duration::from_secs(30);
    loop {
        match rx.recv_timeout(query_time) {
            Ok(_) => {
                for _ in 0..60 {
                    query_modem(&ip_address, &username, &password);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            },
            Err(_) => {
                query_modem(&ip_address, &username, &password);
            }
        }
    }
}

fn query_modem(ip_address: &String, username: &String, password: &String) {

}
