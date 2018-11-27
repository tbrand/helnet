extern crate trust_dns_resolver;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

fn main() {
    let mut resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let mut response = resolver.lookup_ip("www.example.com.").unwrap();
    println!("{:?}", response);
}
