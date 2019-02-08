# `blink-sam3x8e`

This project is a minimal setup to run blinky in Rust on an ARM MCU.

I bootstrapped part of this environment from the 
`cortex-m-quickstart` crate and used this [SAM3X](https://github.com/klangner/sam3x) crate for the device (Arduinu Due).

Some questions are open, e.g. the best interaction between the cargo binutils and programmer (currently hacked around by the `upload.sh` script).
