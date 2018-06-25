use nom;
use std::str;
use std::str::FromStr;
use std::collections::{VecDeque, HashMap};
use indexmap::IndexMap;
use nom::types::CompleteStr;
use nom::IResult;

fn is_vchar(i: char) -> bool {
    i as u8 > 32 && i as u8 <= 126
}

fn valid_header_name_char(c: u8) -> bool {
    c != ':' as u8 && c > 32 && c <= 126
}

named!(pub parse_header<&[u8], (&str, &str)>,
    do_parse!(
        a: map_res!(take_while!(valid_header_name_char), str::from_utf8) >>
        tag!(":") >>
        take_while!(nom::is_space) >>
        b: map_res!(take_until_and_consume!("\r\n"), str::from_utf8) >>
        (a, b)
    )
);

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub startline: &'a str,
    pub headers: IndexMap<&'a str, VecDeque<&'a str>>
}

named!(pub parse< &[u8], Message >,
    do_parse!(
        opt!(tag!("\r\n")) >>
        sl: map_res!(take_until_and_consume!("\r\n"), str::from_utf8) >>
        hdrs: fold_many0!(parse_header, IndexMap::new(), |mut acc: IndexMap<_,VecDeque<_>>, item| {
            let (name, value) = item;
            let res = {
                if let Some(list) = acc.get_mut(&name) {
                    list.push_back(value);
                    true
                } else {
                    false
                }
            };

            if !res {
                let mut new = VecDeque::new();
                new.push_back(value);
                acc.insert(name, new);
            }

            acc
        }) >>
        tag!("\r\n") >>
        (Message { startline: sl, headers: hdrs })
    )
);

#[derive(Debug, PartialEq)]
pub struct RequestLine<'a> {
    pub method: &'a str,
    pub uri: &'a str,
    pub version: &'a str
}

impl<'a> RequestLine<'a> {
    pub fn parse(data: &'a str) -> IResult<CompleteStr, RequestLine> {
        parse_request_line(CompleteStr(data))
    }
}

named!(pub parse_request_line<CompleteStr, RequestLine >,
    do_parse!(
        m: take_until_and_consume_s!(" ") >>
        u: take_until_and_consume_s!(" ") >>
        v: take_while!(is_vchar) >>
        (RequestLine { method: m.0, uri: u.0, version: v.0 })
    )
);

#[derive(Debug, PartialEq)]
pub struct StatusLine<'a> {
    pub code: u32,
    pub message: &'a str,
    pub version: &'a str
}

impl<'a> StatusLine<'a> {
    pub fn parse(data: &'a str) -> IResult<CompleteStr, StatusLine> {
        parse_status_line(CompleteStr(data))
    }
}

named!(pub parse_status_line< CompleteStr, StatusLine >,
    do_parse!(
        v: take_until_and_consume_s!(" ") >>
        c: map_res!(take_until_and_consume_s!(" "), |s: CompleteStr| -> Result<u32, _> { FromStr::from_str(s.0) }) >>
        m: take_while!(is_vchar) >>
        (StatusLine { code: c, message: m.0, version: v.0 })
    )
);
