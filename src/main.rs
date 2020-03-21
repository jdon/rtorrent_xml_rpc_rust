use std::io::prelude::*;
use std::net::TcpStream;

extern crate rtorrent_xml_rpc;

fn main() -> std::io::Result<()> {
	let mut stream = TcpStream::connect("127.0.0.1:16891").unwrap();

	let mut res: Vec<u8> = Vec::new();
	let mut xml_res: Vec<char> = Vec::new();

	let aa = String::from("<?xml version=\"1.0\"?><methodCall><methodName>d.multicall2</methodName><params><param><value><string/></value></param><param><value><string>main</string></value></param><param><value><string>d.name=</string></value></param><param><value><string>d.hash=</string></value></param><param><value><string>d.message=</string></value></param><param><value><string>d.state=</string></value></param><param><value><string>d.priority=</string></value></param><param><value><string>d.state_changed=</string></value></param><param><value><string>d.base_path=</string></value></param><param><value><string>d.directory_base=</string></value></param><param><value><string>d.base_filename=</string></value></param><param><value><string>d.directory=</string></value></param><param><value><string>d.directory.set=</string></value></param><param><value><string>d.completed_bytes=</string></value></param><param><value><string>d.size_bytes=</string></value></param><param><value><string>d.down.total=</string></value></param><param><value><string>d.up.total=</string></value></param><param><value><string>d.down.rate=</string></value></param><param><value><string>d.up.rate=</string></value></param><param><value><string>d.custom=seedingtime=</string></value></param><param><value><string>d.custom=addtime=</string></value></param><param><value><string>d.creation_date==</string></value></param><param><value><string>d.is_private==</string></value></param><param><value><string>d.ratio=</string></value></param><param><value><string>d.peers_connected=</string></value></param><param><value><string>d.bytes_done=</string></value></param><param><value><string>d.is_active=</string></value></param><param><value><string>d.complete=</string></value></param><param><value><string>d.hashing=</string></value></param><param><value><string>d.is_hash_checking=</string></value></param><param><value><string>d.is_open=</string></value></param></params></methodCall>");
	rtorrent_xml_rpc::make_request(&mut stream, &aa, &mut res);
	for ch in res {
		xml_res.push(ch as char);
	}
	println!("{:?}", xml_res);
	Ok(())
}
