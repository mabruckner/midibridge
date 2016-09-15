use alsa;
use std::ffi::CStr;
use std::{fmt, ptr};
use super::error::*;
use super::{Direction};


pub struct Sequencer(*mut alsa::snd_seq_t);

impl Drop for Sequencer {
    fn drop(&mut self) { unsafe { alsa::snd_seq_close(self.0) }; }
}

impl Sequencer {
    pub fn open(name: &CStr, dir: Direction, nonblock: bool) -> Result<Sequencer> {
        let mut r = ptr::null_mut();
        let direction = match dir {
            Direction::Capture => 2,
            Direction::Playback => 1
        };
        let mode = match nonblock {
            true => 1,
            false => 0
        };
        acheck!(snd_seq_open(&mut r, name.as_ptr(), direction, mode)).map(|_| Sequencer(r))
    }
    pub fn client_id(&self) -> Result<usize> { acheck!(snd_seq_client_id(self.0)).map(|i| i as usize) }
    pub fn set_name(&self, name: &CStr) -> Result<()> { acheck!(snd_seq_set_client_name(self.0, name.as_ptr())).map(|_| ()) }
    pub fn create_port(&self, name: &CStr, dir: Direction) -> Result<usize> {
        let flags = match dir {
            Direction::Capture => 0b10 | 0b1000000,
            Direction::Playback => 0b1 | 0b100000
        };
        acheck!(snd_seq_create_simple_port(self.0, name.as_ptr(), flags, 0b10)).map(|x| {x as usize})
    }
    pub fn event_input(&self, evt: &mut Event) -> Result<usize> {
        let mut ptr : *mut alsa::snd_seq_event_t= &mut evt.0;
        let val = acheck!(snd_seq_event_input(self.0, &mut ptr)).map(|x| x as usize);
        unsafe{ptr::copy(ptr, &mut evt.0, 1);}
        val
    }
    pub fn event_output(&self, evt: &mut Event) -> Result<()>
    {
        acheck!(snd_seq_event_output(self.0, &mut evt.0)).map(|_| ())
    }
    pub fn drain_output(&self) -> Result<()>
    {
        acheck!(snd_seq_drain_output(self.0)).map(|_| ())
    }
}

pub struct Event(alsa::snd_seq_event_t);

impl Event {
    pub fn new() -> Event {
        Event(alsa::snd_seq_event_t {
            _type:0,
            flags:0,
            tag:0,
            queue:254,
            time: alsa::snd_seq_timestamp_t {
                data:[0,0]
            },
            source: alsa::snd_seq_addr_t {
                client: 0,
                port: 0
            },
            dest: alsa::snd_seq_addr_t {
                client: 0,
                port: 0
            },
            data: alsa::Union_Unnamed10 {
                data: [0,0,0]
            }
        })
    }
    pub fn get_source(&self) -> (u8, u8) {
        (self.0.source.client, self.0.source.port)
    }
    pub fn set_source(&mut self, (client, port): (u8, u8)) {
        self.0.source.client = client;
        self.0.source.port = port;
    }
    pub fn get_dest(&self) -> (u8, u8) {
        (self.0.dest.client, self.0.dest.port)
    }
    pub fn set_dest(&mut self, (client, port): (u8, u8)) {
        self.0.dest.client = client;
        self.0.dest.port = port;
    }
    pub fn get_data(&self) -> [u32; 3] {
        self.0.data.data.clone()
    }
    pub fn set_data(&mut self, data: [u32;3]) {
        self.0.data.data = data;
    }
    pub fn get_type(&self) -> u8 {
        (self.0)._type
    }
    pub fn set_type(&mut self, t: u8) {
        (self.0)._type = t
    }
    pub fn get_tag(&self) -> u8 {
        self.0.tag
    }
    pub fn set_tag(&mut self, t: u8) {
        self.0.tag = t
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event{{ type: {:?}, flags: {:?}, tag: {:?}, time: {:?}, source: {:?}, dest: {:?}, queue: {:?}, time: {:?}, data: {:?}}}",
               (self.0)._type,
               self.0.flags,
               self.0.tag,
               self.0.time.data,
               self.get_source(),
               self.get_dest(),
               self.0.queue,
               self.0.time.data,
               self.0.data.data)
    }
}
