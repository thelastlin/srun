use core::panic;
use std::net::Ipv4Addr;
use url::Url;

fn main() {
    let auth_server = match (option_env!("AUTH_SERVER_TLS"), cfg!(feature = "tls")) {
        (Some(tls_server), true) => {
            tls_server.parse::<Url>().expect("AUTH_SERVER_TLS invalid");
            tls_server
        }
        (None, enabled @ true) | (_, enabled @ false) => {
            if enabled {
                println!("cargo::warning=\"TLS features enabled without AUTH_SERVER_TLS.\"");
                println!("cargo::warning=\"Fallbacking to AUTH_SERVER_IP\"");
            };
            match option_env!("AUTH_SERVER_IP") {
                Some(var) if var.parse::<Ipv4Addr>().is_ok() => var,
                Some(var) => panic!("AUTH_SERVER_IP invalid: {}", var),
                None => panic!("Expect env AUTH_SERVER_IP, export AUTH_SERVER_IP=10.0.0.1"),
            }
        }
    };
    println!("cargo::rustc-env=AUTH_SERVER={}", auth_server)
}
