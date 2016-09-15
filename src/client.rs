extern crate alsa;
extern crate byteorder;
extern crate clap;

use alsa as a;
use alsa::sequencer as s;

use clap::{App, Arg};

use std::ffi::CString;

use std::net::UdpSocket;

use byteorder::{LittleEndian, WriteBytesExt};

fn to_bytes(evt: &s::Event) -> Vec<u8> {
    let mut wtr = vec![];
    wtr.write_u8(evt.get_type()).unwrap();
    for &x in &evt.get_data() {
        wtr.write_u32::<LittleEndian>(x).unwrap();
    }
    wtr
}

fn main()
{
    let matches = App::new("midibridge client")
                        .about("Creats an alsa sequencer client that connects to a corresponding midibridge server.")
                        .arg(Arg::with_name("server")
                             .required(true)
                             .value_name("SERVER")
                             .help("IP address and port of the server"))
                        .arg(Arg::with_name("client")
                             .short("c")
                             .long("client")
                             .value_name("CLIENT")
                             .help("IP address and port to bind for this client"))
                        .get_matches();
    let addr = matches.value_of("server").unwrap();
    let client = match matches.value_of("client") {
        Some(val) => val,
        None => "0.0.0.0:1234"
    };

    let sock = UdpSocket::bind(client).expect("Unable to bind client address.");
    let name = CString::new("default").unwrap();
    let seq = s::Sequencer::open(&name, a::Direction::Capture, false).unwrap();
    let name = CString::new("midibridge client").unwrap();
    seq.set_name(&name).unwrap();
    let name = CString::new("input").unwrap();
    let port = seq.create_port(&name, a::Direction::Capture).unwrap();
    println!("Starting client on {} (server at {}), sequencer {}:{}", client, addr, seq.client_id().unwrap(), port);
    let mut evt = s::Event::new();
    loop {
        seq.event_input(&mut evt).unwrap();
        let v = to_bytes(&evt);
        sock.send_to(v.as_slice(), addr).unwrap();
    }
}
