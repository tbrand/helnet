extern crate trust_dns_resolver;
extern crate clap;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::net::IpAddr;

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .get_matches();

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = resolver.lookup_ip("www.example.com.").unwrap();

    for ip in response.iter() {
        match ip {
            IpAddr::V4(ip) => {
                println!("It's IPv4 {:?}", ip);
            }
            IpAddr::V6(ip) => {
                println!("It's IPv6 {:?}", ip);
            }
        }
    }
}
