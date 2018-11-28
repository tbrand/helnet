use clap::{Arg, ArgMatches};
use reqwest::{Url, Method};

pub fn uri<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("uri")
        .takes_value(true)
        .long("--uri")
        .short("-U")
        .required(true)
        .multiple(false)
}

pub fn url(matches: &ArgMatches) -> Result<reqwest::Url, reqwest::UrlError>{
    Url::parse(matches.value_of("uri").unwrap())
}

pub fn method<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("method")
        .takes_value(true)
        .long("--method")
        .short("-M")
        .required(false)
        .multiple(false)
}

pub fn rmethod(matches: &ArgMatches) -> Method {
    if let Some(m) = matches.value_of("method") {
        match m.to_uppercase().as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PATCH" => Method::PATCH,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "OPTION" => Method::OPTIONS,
            "HEAD" => Method::HEAD,
            _ => Method::GET, // should error
        }
    } else {
        Method::GET
    }
}

pub fn header<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("header")
        .takes_value(true)
        .long("--header")
        .short("-H")
        .required(false)
        .multiple(true)
}

pub fn headers(matches: &ArgMatches) -> Vec<(String, String)> {
    let mut res = Vec::<(String, String)>::new();

    if let Some(headers) = matches.values_of("header") {
        for header in headers {
            let h: (String, String) = {
                let split = header.split(":").collect::<Vec<&str>>();
                (
                    split[0].trim_start().trim_end().to_owned(),
                    split[1..].join(":").trim_start().trim_end().to_owned()
                )
            };

            res.push(h)
        }
    }

    res
}

pub fn body<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("body")
        .takes_value(true)
        .long("--body")
        .short("-B")
        .required(false)
        .multiple(false)
}
