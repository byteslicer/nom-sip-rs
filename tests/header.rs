extern crate sip;
extern crate nom;

use nom::types::CompleteStr;

#[test]
fn parse_uri_params() {
    let (rest, parsed) = sip::parse_uri_params(CompleteStr(";tag=jf73350;test=abc")).unwrap();
    assert_eq!(parsed, (CompleteStr("tag"), CompleteStr("jf73350")));
}

#[test]
fn via() {
    let via = "SIP/2.0/TCP ss2.biloxi.example.com:5061;branch=z9hG4bK123";
    let (rest, parsed) = sip::parse_via(CompleteStr(via)).unwrap();

    assert_eq!(parsed.protocol, CompleteStr("SIP/2.0/TCP"));
    assert_eq!(parsed.host, CompleteStr("ss2.biloxi.example.com"));
    assert_eq!(parsed.port, CompleteStr("5061"));

    assert_eq!(parsed.parameters.get(&CompleteStr("branch")).unwrap(), &CompleteStr("z9hG4bK123"));
}

#[test]
fn name_addr_with_name_pass() {
    let to_hdr = "\"test\" <sip:foo@ss2.biloxi.example.com:5061;user=phone>;tag=jf73350";
    let (rest, parsed) = sip::parse_name_addr(CompleteStr(to_hdr)).unwrap();

    assert_eq!(parsed.name, Some(CompleteStr("test")));
    assert_eq!(parsed.uri, CompleteStr("sip:foo@ss2.biloxi.example.com:5061;user=phone"));
    assert_eq!(parsed.parameters.get(&CompleteStr("tag")).unwrap(), &CompleteStr("jf73350"));
}

#[test]
fn name_addr_without_name_pass() {
    let to_hdr = "<sip:foo@ss2.biloxi.example.com:5061;user=phone>;tag=jf73350";
    let (rest, parsed) = sip::parse_name_addr(CompleteStr(to_hdr)).unwrap();

    assert_eq!(parsed.name, None);
    assert_eq!(parsed.uri, CompleteStr("sip:foo@ss2.biloxi.example.com:5061;user=phone"));
    assert_eq!(parsed.parameters.get(&CompleteStr("tag")).unwrap(), &CompleteStr("jf73350"));
}

#[test]
fn uri_auth_address() {
    let uri = "ss2.biloxi.example.com:5061";
    let (rest, parsed) = sip::parse_uri_address(CompleteStr(uri)).unwrap();

    assert_eq!(parsed.0, CompleteStr("ss2.biloxi.example.com"));
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

    assert_eq!(parsed.0, Some(CompleteStr("foo")));
    assert_eq!(parsed.1.unwrap(), CompleteStr("bar"));
}

#[test]
fn uri_auth_user_only() {
    let uri = "foo@ss2.biloxi.example.com";

    let (rest, parsed) = sip::parse_uri_auth(CompleteStr(uri)).unwrap();

    assert_eq!(parsed.0, Some(CompleteStr("foo")));
    //println!("{:?}", parsed.1);
    assert_eq!(parsed.1.is_some(), false);
}

#[test]
fn uri_2() {
    let uri = "sip:1002@192.168.1.243";
    let (rest, parsed) = sip::parse_uri(CompleteStr(uri)).unwrap();

    assert_eq!(parsed.schema, CompleteStr("sip"));
    assert_eq!(parsed.user, Some(CompleteStr("1002")));
    assert_eq!(parsed.password, None);
    assert_eq!(parsed.host, CompleteStr("192.168.1.243"));
    assert_eq!(parsed.port, None);
}

#[test]
fn uri() {
    let uri = "sip:foo@ss2.biloxi.example.com:5061;user=phone";
    let (rest, parsed) = sip::parse_uri(CompleteStr(uri)).unwrap();

    assert_eq!(parsed.schema, CompleteStr("sip"));
    assert_eq!(parsed.user, Some(CompleteStr("foo")));
    assert_eq!(parsed.password, None);
    assert_eq!(parsed.host, CompleteStr("ss2.biloxi.example.com"));
    assert_eq!(parsed.port, Some(5061));

    assert_eq!(parsed.parameters.get(&CompleteStr("user")).unwrap(), &CompleteStr("phone"));
}
