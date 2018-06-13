use nom;
use std::str;
use std::str::FromStr;
use std::collections::HashMap;
use indexmap::IndexMap;
use nom::types::CompleteStr;

fn is_vchar(i: char) -> bool {
    i as u8 > 32 && i as u8 <= 126
}

fn valid_header_name_char(c: u8) -> bool {
    c != ':' as u8 && c > 32 && c <= 126
}

named!(pub parse_header<&[u8], (&str, CompleteStr)>,
    do_parse!(
        a: map_res!(take_while!(valid_header_name_char), str::from_utf8) >>
        tag!(":") >>
        take_while!(nom::is_space) >>
        b: map_res!(take_until_and_consume!("\r\n"), str::from_utf8) >>
        (a, CompleteStr(b))
    )
);

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub startline: CompleteStr<'a>,
    pub headers: IndexMap<&'a str, Vec<CompleteStr<'a>>>
}

named!(pub parse< &[u8], Message >,
    do_parse!(
        opt!(tag!("\r\n")) >>
        sl: map_res!(take_until_and_consume!("\r\n"), str::from_utf8) >>
        hdrs: fold_many0!(parse_header, IndexMap::new(), |mut acc: IndexMap<_,Vec<_>>, item| {
            let (name, value) = item;
            let res = {
                if let Some(list) = acc.get_mut(&name) {
                    list.push(value);
                    true
                } else {
                    false
                }
            };

            if !res {
                acc.insert(name, vec![value]);
            }

            acc
        }) >>
        tag!("\r\n") >>
        (Message { startline: CompleteStr(sl), headers: hdrs })
    )
);

#[derive(Debug, PartialEq)]
pub struct RequestLine<'a> {
    pub method: CompleteStr<'a>,
    pub uri: CompleteStr<'a>,
    pub version: CompleteStr<'a>
}

named!(pub parse_request_line<CompleteStr, RequestLine >,
    do_parse!(
        m: take_until_and_consume_s!(" ") >>
        u: take_until_and_consume_s!(" ") >>
        v: take_while!(is_vchar) >>
        (RequestLine { method: m, uri: u, version: v })
    )
);

#[derive(Debug, PartialEq)]
pub struct StatusLine<'a> {
    pub code: u32,
    pub message: CompleteStr<'a>,
    pub version: CompleteStr<'a>
}

named!(pub parse_status_line< CompleteStr, StatusLine >,
    do_parse!(
        v: take_until_and_consume_s!(" ") >>
        c: map_res!(take_until_and_consume_s!(" "), |s: CompleteStr| -> Result<u32, _> { FromStr::from_str(s.0) }) >>
        m: take_while!(is_vchar) >>
        (StatusLine { code: c, message: m, version: v })
    )
);
