extern crate sip;
extern crate nom;

use nom::types::CompleteStr;
use sip::{Uri, Via};

#[test]
fn parse_uri_params() {
    let (rest, parsed) = sip::parse_uri_params(CompleteStr(";tag=jf73350;test=abc")).unwrap();
    assert_eq!(parsed, ("tag", Some("jf73350")));
}

#[test]
fn via() {
    let via = "SIP/2.0/TCP ss2.biloxi.example.com:5061;branch=z9hG4bK123;rport";
    let (rest, parsed) = Via::parse(via).unwrap();

    assert_eq!(parsed.protocol, "SIP/2.0/TCP");
    assert_eq!(parsed.host, "ss2.biloxi.example.com");
    assert_eq!(parsed.port, "5061");

    assert_eq!(parsed.parameters.get(&"branch").unwrap(), &Some("z9hG4bK123"));
    assert_eq!(parsed.parameters.get(&"rport").unwrap(), &None);
}

#[test]
fn via2() {
    let via = "SIP/2.0/UDP 192.168.1.194:5061";
    let (rest, parsed) = Via::parse(via).unwrap();

    assert_eq!(parsed.protocol, "SIP/2.0/UDP");
    assert_eq!(parsed.host, "192.168.1.194");
    assert_eq!(parsed.port, "5061");
}

#[test]
fn name_addr_with_name_pass() {
    let to_hdr = "\"test\" <sip:foo@ss2.biloxi.example.com:5061;user=phone>;tag=jf73350";
    let (rest, parsed) = sip::parse_name_addr(CompleteStr(to_hdr)).unwrap();

    assert_eq!(parsed.name, Some("test"));
    assert_eq!(parsed.uri, "sip:foo@ss2.biloxi.example.com:5061;user=phone");
    assert_eq!(parsed.parameters.get(&"tag").unwrap().unwrap(), "jf73350");
}

#[test]
fn name_addr_without_name_pass() {
    let to_hdr = "<sip:foo@ss2.biloxi.example.com:5061;user=phone>;tag=jf73350";
    let (rest, parsed) = sip::parse_name_addr(CompleteStr(to_hdr)).unwrap();

    assert_eq!(parsed.name, None);
    assert_eq!(parsed.uri, "sip:foo@ss2.biloxi.example.com:5061;user=phone");
    assert_eq!(parsed.parameters.get(&"tag").unwrap().unwrap(), "jf73350");
}

#[test]
fn uri_auth_address() {
    let uri = "ss2.biloxi.example.com:5061";
    let (rest, parsed) = sip::parse_uri_address(CompleteStr(uri)).unwrap();

    assert_eq!(parsed.0, "ss2.biloxi.example.com");
    assert_eq!(parsed.1, Some(5061));
}

#[test]
fn uri_auth() {
    let uri = "ss2.biloxi.example.com";

    let res = sip::parse_uri_auth(CompleteStr(uri));
    assert_eq!(res.is_ok(), false);
}

#[test]
fn uri_auth_user_pass() {
    let uri = "foo:bar@ss2.biloxi.example.com";

    let (rest, parsed) = sip::parse_uri_auth(CompleteStr(uri)).unwrap();

    assert_eq!(parsed.0, Some("foo"));
    assert_eq!(parsed.1.unwrap(), "bar");
}

#[test]
fn uri_auth_user_only() {
    let uri = "foo@ss2.biloxi.example.com";

    let (rest, parsed) = sip::parse_uri_auth(CompleteStr(uri)).unwrap();

    assert_eq!(parsed.0, Some("foo"));
    //println!("{:?}", parsed.1);
    assert_eq!(parsed.1.is_some(), false);
}

#[test]
fn uri_2() {
    let uri = "sip:1002@192.168.1.243";
    let (rest, parsed) = Uri::parse(uri).unwrap();

    assert_eq!(parsed.schema, "sip");
    assert_eq!(parsed.user, Some("1002"));
    assert_eq!(parsed.password, None);
    assert_eq!(parsed.host, "192.168.1.243");
    assert_eq!(parsed.port, None);
}

#[test]
fn uri() {
    let uri = "sip:foo@ss2.biloxi.example.com:5061;user=phone";
    let (rest, parsed) = Uri::parse(uri).unwrap();

    assert_eq!(parsed.schema, "sip");
    assert_eq!(parsed.user, Some("foo"));
    assert_eq!(parsed.password, None);
    assert_eq!(parsed.host, "ss2.biloxi.example.com");
    assert_eq!(parsed.port, Some(5061));

    assert_eq!(parsed.parameters.get(&"user").unwrap().unwrap(), "phone");
}
