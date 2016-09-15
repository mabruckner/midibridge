extern crate alsa;
extern crate byteorder;
extern crate clap;

use alsa as a;
use alsa::sequencer as s;

use byteorder::{LittleEndian, ReadBytesExt};

use clap::{App, Arg};

use std::ffi::CString;

use std::net::UdpSocket;


fn from_bytes(evt: &mut s::Event, buf: &[u8]) {
    let mut buf = buf.clone();
    evt.set_type(buf.read_u8().unwrap());
    let mut vals = [0;3];
    for i in 0..3 {
        vals[i] = buf.read_u32::<LittleEndian>().unwrap()
    }
    evt.set_data(vals);
}

fn main()
{
    let matches = App::new("midibridge server")
                        .about("Creates an alsa sequencer that emits midi events recieved from one or more midibridge clients.")
                        .arg(Arg::with_name("address")
                             .required(true)
                             .value_name("ADDRESS")
                             .help("IP address and port to bind for server"))
                        .get_matches();
    let addr = matches.value_of("address").unwrap();

    let sock = UdpSocket::bind(addr).expect("Unable to bind address.");
    let name = CString::new("default").unwrap();
    let seq = s::Sequencer::open(&name, a::Direction::Playback, false).unwrap();
    let name = CString::new("midibridge server").unwrap();
    seq.set_name(&name).unwrap();
    let name = CString::new("output").unwrap();
    let port = seq.create_port(&name, a::Direction::Playback).unwrap();
    let mut evt = s::Event::new();
    println!("Starting server at {}, sequencer {}:{}", addr, seq.client_id().unwrap(), port);
    loop {
        let mut buf = [0; 16];
        sock.recv_from(&mut buf).unwrap();
        from_bytes(&mut evt, &buf);
        evt.set_source((seq.client_id().unwrap() as u8, port as u8));
        evt.set_dest((254, 0));
        seq.event_output(&mut evt).unwrap();
        seq.drain_output().unwrap();
    }
}
