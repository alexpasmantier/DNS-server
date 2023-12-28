use std::net::SocketAddr;

pub fn parse_arguments_for_resolver(arguments: &Vec<String>) -> Option<SocketAddr> {
    let mut external_resolver = false;
    for arg in arguments {
        if external_resolver {
            let address: SocketAddr = arg.parse().unwrap();
            return Some(address);
        }
        if arg == "--resolver" {
            external_resolver = true;
        }
    }
    return None;
}
