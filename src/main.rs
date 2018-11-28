extern crate clap;
extern crate reqwest;
extern crate trust_dns_resolver;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::net::IpAddr;
use std::time::Duration;
use reqwest::{Method, Url};

mod cli;

fn http_request(method: &Method, url: &Url) -> Result<reqwest::Response, reqwest::Error>{
    let client = reqwest::Client::builder().timeout(Duration::from_secs(1)).build().unwrap();
    client.request(method.clone(), url.clone()).send()
}

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .arg(cli::uri())
        .arg(cli::method())
        .arg(cli::body())
        .arg(cli::header())
        .get_matches();

    let headers = cli::headers(&matches);
    println!("headers: {:?}", headers);

    let url = cli::url(&matches);
    println!("url: {:?}", url);

    if url == Err(reqwest::UrlError::RelativeUrlWithoutBase) {
        println!("port not specified");
        std::process::exit(-1);
    }

    let url = url.unwrap();

    let method = cli::rmethod(&matches);
    println!("method: {:?}", method);

    let response = http_request(&method, &url);
    println!("{:?}", response);

    // let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    // let response = resolver.lookup_ip("www.example.com.").unwrap();
    //  
    // for ip in response.iter() {
    //     match ip {
    //         IpAddr::V4(ip) => {
    //             println!("It's IPv4 {:?}", ip);
    //         }
    //         IpAddr::V6(ip) => {
    //             println!("It's IPv6 {:?}", ip);
    //         }
    //     }
    // }
}
