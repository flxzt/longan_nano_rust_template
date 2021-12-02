# Sipeed Longan Nano w/ Rust template

**Fork from [nanoball](https://github.com/pdx-cs-rust/nanoball)  
(original Author: Bart Massey)**

## Setup

Unplug your Longan Nano from USB if plugged in.

Install `99-ftdi.rules` from this directory in
`/etc/udev/rules.d/` and run

    udevadm control --reload-rules

    cargo install cargo-binutils

    rustup component add llvm-tools-preview

* Get the Rust compiler for this chipset.

You are then ready to build a binary.

    sh build.sh firmware

Now install `dfu-util` from your distro repo's ( version must be >= 0.11)

* Hook your Longan Nano to your box via USB. The loader
  doesn't cope well with USB hubs, so hook directly to your
  host's USB.

* Reset the Longan Nano by holding down the "BOOT" button
  (the taller one in the standard case, on the right when
  USB port is at the bottom), then pressing and releasing
  the "RESET" button (the other one).

* Upload the demo. 

    sh dfuload.sh firmware

  You may need to power the Nano down and back up afterward.

There should be a color changing triangle in the center of the display.

## Debugging

You'll need a JTAG unit to get started with this. I'm using
the
[Sipeed USB-JTAG/TTL RISC-V Debugger](https://www.seeedstudio.com/Sipeed-USB-JTAG-TTL-RISC-V-Debugger-p-2910.html),
which is about $8. Connect as follows:

     JTAG      Nano
     TDI       JTDI
     TMS       JTMS
     TDO       JTDO
     TCK       JTCK
     RXD       R0
     TXD       T0
     GND       GND
     
You'll also need a version of OpenOCD that supports the
Nano's processor. Sadly, Debian's is not good enough: you'll
want to build and install
[`riscv-openocd`](https://github.com/riscv/riscv-openocd).

Finally, you'll need `gdb-multiarch` or similar to actually
do the debugging.

Once you have everything set up, get the Nano ready for
upload as described at the beginning of this document
(+BOOT,+RESET,-RESET,-BOOT). Then run `sh openocd.sh`. You
should see `openocd` come up, give some mostly-meaningless
errors, then attach to your Nano. Go to a separate terminal
and run `cargo run --release`. You should now see `gdb`
start up, upload `firmware.bin`, and start it running. Hit
`^C` now to interrupt the running program. Gratz, you're
debugging away!

## Memory Size

There's an older version of the Longan Nano that has less
memory. I doubt anyone has one anymore. Anyway, all the
stuff to handle these is here, but will require some
rewiring to access.

## Resources

* https://github.com/pdx-cs-rust/nanoball

* https://dl.sipeed.com/LONGAN/platformio/dl-packages/

    tool-gd32vflash-v0.1.0-linux.tar.gz

* https://pramode.net/2019/10/07/rust-on-riscv-board-sipeed-longan-nano/

* https://blog.tonari.no/rust-simple-hardware-project
