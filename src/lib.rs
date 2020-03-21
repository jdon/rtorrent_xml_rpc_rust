use std::io::prelude::*;
use std::net::TcpStream;

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
