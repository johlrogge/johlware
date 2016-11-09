#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::bindings;

#[no_mangle]
pub unsafe extern fn main() {
    // Blink Loop
    bindings::pinMode(13, bindings::OUTPUT as u8);
    bindings::digitalWrite(13, bindings::LOW as u8);

    bindings::pinMode(23, bindings::INPUT as u8);

    bindings::pinMode(0, bindings::OUTPUT as u8);
    bindings::digitalWrite(0, bindings::HIGH as u8);

    bindings::pinMode(11, bindings::OUTPUT as u8);
    bindings::digitalWrite(11, bindings::HIGH as u8);

  loop {
      bindings::digitalWrite(11, bindings::digitalRead (23));
      bindings::delay(10);
    }
}

/// Blink the light twice to know we're alive
pub unsafe fn alive() {
    for _ in 0..2 {
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(200);
        bindings::digitalWrite(13, bindings::HIGH as u8);
        bindings::delay(200);
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(200);
    }
}
