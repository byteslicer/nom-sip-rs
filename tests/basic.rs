extern crate sip;
extern crate nom;

use nom::types::CompleteStr;

#[test]
fn request_line() {
    let message = "REGISTER sips:ss2.biloxi.example.com SIP/2.0";
    let (rest, parsed) = sip::parse_request_line(CompleteStr(message)).unwrap();

    assert_eq!(parsed.method, "REGISTER");
    assert_eq!(parsed.uri, "sips:ss2.biloxi.example.com");
    assert_eq!(parsed.version, "SIP/2.0");
}

#[test]
fn status_line() {
    let message = "SIP/2.0 200 OK";
    let (rest, parsed) = sip::parse_status_line(CompleteStr(message)).unwrap();

    assert_eq!(parsed.code, 200);
    assert_eq!(parsed.message, "OK");
    assert_eq!(parsed.version, "SIP/2.0");
}

#[test]
fn parse_header() {
    let header = "Via: SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7\r\n";
    let (rest, parsed) = sip::parse_header(header.as_bytes()).unwrap();
    assert_eq!(parsed, ("Via", "SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7"));
}

#[test]
fn basic_sip() {
    let mut message = "REGISTER sips:ss2.biloxi.example.com SIP/2.0\r\n".to_string();
    message.push_str("Via: SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7\r\n");
    message.push_str("Max-Forwards: 70\r\n");
    message.push_str("From: Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl\r\n");
    message.push_str("To: Bob <sips:bob@biloxi.example.com>\r\n");
    message.push_str("Call-ID: 1j9FpLxk3uxtm8tn@biloxi.example.com\r\n");
    message.push_str("CSeq: 1 REGISTER\r\n");
    message.push_str("Contact: <sips:bob@client.biloxi.example.com>\r\n");
    message.push_str("Content-Length: 0\r\n\r\n");

    let (rest, parsed) = sip::parse(message.as_bytes()).unwrap();

    assert_eq!(parsed.startline, "REGISTER sips:ss2.biloxi.example.com SIP/2.0");

    let via_list = parsed.headers.get("Via").unwrap();
    assert_eq!(via_list.len(), 1);
    assert_eq!(via_list[0], "SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7");

    let content_length_list = parsed.headers.get("Content-Length").unwrap();
    assert_eq!(content_length_list.len(), 1);
    assert_eq!(content_length_list[0], "0");

    assert_eq!(rest.len(), 0);
}
