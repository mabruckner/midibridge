# midibridge

Midibridge is a simple method for transmitting midi events from one machine to another.
The midibridge client and server each expose an alsa sequencer client, input-only for the client and
output-only for the server. If everything is configured correctly, MIDI events sent to the client
sequencer will be produced by the server sequencer.

midibridge makes use of UDP, so it is possible to lose notes in transmission. In practice this will
be influenced by the reliability and consistency of the network.

## Usage

### On receiving end

```
$ cargo run --bin server 0.0.0.0:1234
```

### On transmitting end

```
$ cargo run --bin client <receiving_machine_ip>:1234
```
