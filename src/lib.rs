#![warn(clippy::pedantic)]
use std::io::prelude::*;
use std::net::TcpStream;

/// Sends XML to an rTorrent XML RPC server
/// # Arguments
/// * `TcpStream` - The TCP stream used to make a request
/// * `xml` - The XML to send to rTorrent
/// * `buf` - The vector to read the response from rTorrent into
///
/// # Example
/// ```
/// let mut stream = TcpStream::connect("127.0.0.1:16891").unwrap();
///
/// let mut res: Vec<u8> = Vec::new();
/// let mut xml_res: Vec<char> = Vec::new();
///
/// let xml = String::from("<?xml version=\"1.0\"?><methodCall><methodName>d.multicall2</methodName><params><param><value><string/></value></param><param><value><string>main</string></value></param><param><value><string>d.name=</string></value></param><param><value><string>d.hash=</string></value></param><param><value><string>d.message=</string></value></param><param><value><string>d.state=</string></value></param><param><value><string>d.priority=</string></value></param><param><value><string>d.state_changed=</string></value></param><param><value><string>d.base_path=</string></value></param><param><value><string>d.directory_base=</string></value></param><param><value><string>d.base_filename=</string></value></param><param><value><string>d.directory=</string></value></param><param><value><string>d.directory.set=</string></value></param><param><value><string>d.completed_bytes=</string></value></param><param><value><string>d.size_bytes=</string></value></param><param><value><string>d.down.total=</string></value></param><param><value><string>d.up.total=</string></value></param><param><value><string>d.down.rate=</string></value></param><param><value><string>d.up.rate=</string></value></param><param><value><string>d.custom=seedingtime=</string></value></param><param><value><string>d.custom=addtime=</string></value></param><param><value><string>d.creation_date==</string></value></param><param><value><string>d.is_private==</string></value></param><param><value><string>d.ratio=</string></value></param><param><value><string>d.peers_connected=</string></value></param><param><value><string>d.bytes_done=</string></value></param><param><value><string>d.is_active=</string></value></param><param><value><string>d.complete=</string></value></param><param><value><string>d.hashing=</string></value></param><param><value><string>d.is_hash_checking=</string></value></param><param><value><string>d.is_open=</string></value></param></params></methodCall>");
/// rtorrent_xml_rpc::make_request(&mut stream, &xml, &mut res);
/// for ch in res {
///  xml_res.push(ch as char);
/// }
/// println!("{:?}", xml_res);
/// ```
///
/// # Errors
/// When it fails to write or read from the `TcpStream`
pub fn make_request(
	stream: &mut TcpStream,
	xml: &str,
	buf: &mut Vec<u8>,
) -> std::io::Result<usize> {
	let headers = generate_headers(xml);
	let header_length = generate_header_length(&headers);
	let request = generate_request(header_length, &headers, xml).into_bytes();
	let request_array: &[u8] = &request;
	stream.write_all(request_array)?;
	stream.read_to_end(buf)
}

fn generate_headers(xml: &str) -> Vec<String> {
	const NULL_CHAR: char = '\0';
	let xml_length = xml.len();
	let content_length = format!(
		"CONTENT_LENGTH{null_char}{xml_length}{null_char}",
		null_char = NULL_CHAR,
		xml_length = xml_length
	);
	let scgi = format!("SCGI{null_char}1{null_char}", null_char = NULL_CHAR);
	let array: Vec<String> = vec![content_length, scgi];

	array
}

fn generate_header_length(headers: &[String]) -> usize {
	let mut length = 0;
	for header in headers {
		length += header.len();
	}
	length
}

fn generate_request(header_length: usize, headers: &[String], xml: &str) -> String {
	let b = headers.join(",");
	format!("{}:{}{}", header_length, b, xml)
}
