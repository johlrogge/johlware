#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::bindings;

#[no_mangle]
pub unsafe extern fn main() {


    for pin in 18..21 {
        bindings::pinMode(pin, bindings::INPUT_PULLUP as u8);
    }

    for pin in 21..24 {
        bindings::pinMode(pin, bindings::OUTPUT as u8);
        bindings::digitalWrite(pin, bindings::HIGH as u8);
    }

    for pin in 0..9 {
        bindings::pinMode (pin, bindings::OUTPUT as u8);
        bindings::digitalWrite (pin, bindings::HIGH as u8);
    }

    loop {
        scan();
        bindings::delay(1);
    }
}

unsafe fn scan(){
    let output = [
        [0,1,2],
        [3,4,5],
        [6,7,8]
    ];
    for (rindex,  row) in (21..24).enumerate() {
        bindings::digitalWrite(row, bindings::LOW as u8);
        bindings::delay(1);
        for (cindex, col) in (18..21).enumerate() {
            bindings::digitalWrite(output[rindex][cindex], bindings::digitalRead(col));
        }
        bindings::delay(1);
        bindings::digitalWrite(row, bindings::HIGH as u8);
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
