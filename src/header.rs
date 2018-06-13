use nom;
use std::str;
use std::str::FromStr;
use std::collections::HashMap;
use nom::types::CompleteStr;

fn is_vchar(i: char) -> bool {
    i as u8 > 32 && i as u8 <= 126
}

fn not_uri_end(x: char) -> bool {
    is_vchar(x) && x != ';'
}

named!(pub parse_uri_params<CompleteStr, (CompleteStr, CompleteStr)>,
    preceded!(tag!(";"), separated_pair!(take_until_s!("="), char!('='), take_while!(not_uri_end)))
);

#[derive(Debug, PartialEq)]
pub struct Via<'a> {
    pub protocol: CompleteStr<'a>,
    pub host: CompleteStr<'a>,
    pub port: CompleteStr<'a>,
    pub parameters: HashMap<CompleteStr<'a>, CompleteStr<'a>>
}

named!(pub parse_via<CompleteStr, Via >,
    do_parse!(
        protocol: take_until_and_consume_s!(" ") >>
        host: take_until_and_consume_s!(":") >>
        port: take_until_s!(";") >>
        params: fold_many0!(parse_uri_params, HashMap::new(), |mut acc: HashMap<_,_>, item| {
                let (name, value) = item;
                acc.insert(name, value);
                acc
        }) >>
        (Via {
            protocol: protocol,
            host: host,
            port: port,
            parameters: params
        })
    )
);

#[derive(Debug, PartialEq)]
pub struct NameAddr<'a> {
    pub name: Option<CompleteStr<'a>>,
    pub uri: CompleteStr<'a>,
    pub parameters: HashMap<CompleteStr<'a>, CompleteStr<'a>>
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
            name: name,
            uri: uri,
            parameters: params
        })
    )
);

#[derive(Debug, PartialEq)]
pub struct Uri<'a> {
    pub schema: CompleteStr<'a>,
    pub user: Option<CompleteStr<'a>>,
    pub password: Option<CompleteStr<'a>>,
    pub host: CompleteStr<'a>,
    pub port: Option<u16>,
    pub parameters: HashMap<CompleteStr<'a>, CompleteStr<'a>>
    //headers: HashMap<&'a str, &'a str>
}

fn testat(chr: char) -> bool { chr != ':' && chr != '@' }

named!(pub parse_uri_auth<CompleteStr, (Option<CompleteStr>, Option<CompleteStr>)>,
    do_parse!(
        a: take_while!(testat) >>
        b: opt!(preceded!(tag!(":"), take_while!(testat))) >>
        tag!("@") >> (Some(a), b)
    )
);

fn not_address_end(x: char) -> bool { is_vchar(x) && x != ':' && x != '@' }

named!(pub parse_uri_address<CompleteStr, (CompleteStr, Option<u16>)>,
    tuple!(
        take_while!(not_address_end),
        opt!(map_res!(preceded!(tag!(":"), take_while!(|x|{ nom::is_digit(x as u8) })), |s: CompleteStr| -> Result<u16, _> { FromStr::from_str(s.0) } ))
    )
);

named!(pub parse_uri<CompleteStr, Uri>,
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
            schema: schema,
            user: up.0 ,
            password: up.1,
            host: hp.0,
            port: hp.1,
            parameters: params
        })
    )
);
