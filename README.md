# chipmunk

<p align="center">
  <img width="150px" src="https://cdn-icons-png.flaticon.com/512/616/616570.png" />
</p>

A [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) interpreter written in Rust.

This was a project I built live during many of my programming streams, which you can [find here](https://twitch.tv/reaganmcf_)

### Demo

### Roms

[kripod/chip8-roms](https://github.com/kripod/chip8-roms/)

### Usage

To model the keyboard of the time, the following keys are mapped like so

```
Keypad                   Keyboard
+-+-+-+-+                +-+-+-+-+
|1|2|3|C|                |1|2|3|4|
+-+-+-+-+                +-+-+-+-+
|4|5|6|D|                |Q|W|E|R|
+-+-+-+-+       =>       +-+-+-+-+
|7|8|9|E|                |A|S|D|F|
+-+-+-+-+                +-+-+-+-+
|A|0|B|F|                |Z|X|C|V|
+-+-+-+-+                +-+-+-+-+
```

##### Run a ROM

```console
chipmunk run roms/pong.rom
```

##### Run a ROM in Debug Mode

Roms can be ran in debug mode, where the program will wait for `F` (mapped to `V`) to be pressed before moving on the next instruction.

```console
chipmunk run --debug roms/pong.rom 
```

##### Disassemble a rom
```console
chipmunk dis roms/pong.rom
```
