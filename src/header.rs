use nom;
use std::str;
use std::str::FromStr;
use std::collections::HashMap;
use nom::types::CompleteStr;
use nom::IResult;

fn is_vchar(i: char) -> bool {
    i as u8 > 32 && i as u8 <= 126
}

fn not_uri_end(x: char) -> bool {
    is_vchar(x) && x != ';'
}

/*
named!(pub parse_uri_params<CompleteStr, (&str, Option<&str>)>,
    preceded!(tag!(";"), map!(separated_pair!(take_until_s!("="), char!('='), take_while!(not_uri_end)), |(a, b)| (a.0, b.0)))
);
*/

named!(pub parse_uri_params<CompleteStr, (&str, Option<&str>)>,
    preceded!(tag!(";"), do_parse!(
        key: take_while!(|c| c != '=' && c != ';') >>
        value: opt!(preceded!(tag!("="), take_while!(not_uri_end))) >>
        ((key.0, value.map(|s| s.0)))
    ))
);

#[derive(Debug, PartialEq)]
pub struct Via<'a> {
    pub protocol: &'a str,
    pub host: &'a str,
    pub port: &'a str,
    pub parameters: HashMap<&'a str, Option<&'a str>>
}

impl<'a> Via<'a> {
    pub fn parse(data: &'a str) -> IResult<CompleteStr, Via> {
        parse_via(CompleteStr(data))
    }
}

named!(pub parse_via<CompleteStr, Via >,
    do_parse!(
        protocol: take_until_and_consume_s!(" ") >>
        host: take_until_and_consume_s!(":") >>
        port: take_while!(not_uri_end) >>
        params: fold_many0!(parse_uri_params, HashMap::new(), |mut acc: HashMap<_,_>, item| {
                let (name, value) = item;
                acc.insert(name, value);
                acc
        }) >>
        (Via {
            protocol: protocol.0,
            host: host.0,
            port: port.0,
            parameters: params
        })
    )
);


#[derive(Debug, PartialEq)]
pub struct NameAddr<'a> {
    pub name: Option<&'a str>,
    pub uri: &'a str,
    pub parameters: HashMap<&'a str, Option<&'a str>>
}

named!(pub parse_name_addr< CompleteStr, NameAddr >,
    do_parse!(
        name: opt!(delimited!(tag!("\""), take_until_s!("\""), tag!("\" "))) >> //opt!(terminated!(preceded!(tag!("\""), take_until_s!("\"")), tag!("\" "))) >>
        tag!("<") >>
        uri: take_until_and_consume_s!(">") >>
        params: fold_many0!(parse_uri_params, HashMap::new(), |mut acc: HashMap<_,_>, item| {
                let (name, value) = item;
                acc.insert(name, value);
                acc
        }) >>
        (NameAddr {
            name: name.map(|s| s.0),
            uri: uri.0,
            parameters: params
        })
    )
);

#[derive(Debug, PartialEq)]
pub struct Uri<'a> {
    pub schema: &'a str,
    pub user: Option<&'a str>,
    pub password: Option<&'a str>,
    pub host: &'a str,
    pub port: Option<u16>,
    pub parameters: HashMap<&'a str, Option<&'a str>>
    //headers: HashMap<&'a str, &'a str>
}

fn testat(chr: char) -> bool { chr != ':' && chr != '@' }

named!(pub parse_uri_auth<CompleteStr, (Option<&str>, Option<&str>)>,
    do_parse!(
        a: take_while!(testat) >>
        b: opt!(preceded!(tag!(":"), take_while!(testat))) >>
        tag!("@") >> (Some(a.0), b.map(|s| s.0))
    )
);

fn not_address_end(x: char) -> bool { is_vchar(x) && x != ':' && x != '@' }

named!(pub parse_uri_address<CompleteStr, (&str, Option<u16>)>,
    do_parse!(
        a: take_while!(not_address_end) >>
        b: opt!(map_res!(preceded!(tag!(":"), take_while!(|x|{ nom::is_digit(x as u8) })), |s: CompleteStr| -> Result<u16, _> { FromStr::from_str(s.0) } )) >>
        ((a.0, b))
    )
);

named!(parse_uri<CompleteStr, Uri>,
    do_parse!(
        schema: take_until_and_consume_s!(":") >>
        up: alt!(call!(parse_uri_auth) | do_parse!((None, None))) >>
        hp: call!(parse_uri_address) >>
        params: fold_many0!(parse_uri_params, HashMap::new(), |mut acc: HashMap<_,_>, item| {
                let (name, value) = item;
                acc.insert(name, value);
                acc
        }) >>
        (Uri {
            schema: schema.0,
            user: up.0 ,
            password: up.1,
            host: hp.0,
            port: hp.1,
            parameters: params
        })
    )
);
