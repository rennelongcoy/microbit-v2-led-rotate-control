Press B to rotate the LED clockwise.

Press A to rotate counter clockwise.

```
rustup target add thumbv7em-none-eabihf
rustup show
```

`rustup show` output:
```
installed targets for active toolchain
--------------------------------------

thumbv7em-none-eabihf
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.82.0 (f6e511eec 2024-10-15)
```

Add required crates:
```
cargo add cortex-m --features critical-section-single-core
cargo add cortex-m-rt
cargo add embedded-hal
cargo add microbit-v2
cargo add nrf52833-pac
cargo add panic-halt
```
