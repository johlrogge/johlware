#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::bindings;
mod gpio;

struct Pin {
    pin:u8
}

impl gpio::Read<bool> for Pin {
    fn read(&self)->bool {
        unsafe {
            bindings::digitalRead (self.pin) == bindings::HIGH as u8
        }
    }
}

impl gpio::Write<bool> for Pin {
    fn write(&self, value:bool) {
        unsafe {
            bindings::digitalWrite(self.pin, (if value {bindings::HIGH} else {bindings::LOW}) as u8);
        }
    }
}

impl Pin {
    fn  input_pullup(pin:u8)-> Pin {
        unsafe {
            bindings::pinMode(pin, bindings::INPUT_PULLUP as u8);
        }
        Pin {
            pin: pin
        }
    }

    fn output(pin:u8, initial:bool) -> Pin{
        use gpio::Write;
        unsafe {
            bindings::pinMode(pin, bindings::OUTPUT as u8);
        }
        let res = Pin {
            pin: pin
        };
        res.write(initial);
        res
    }
}

#[no_mangle]
pub unsafe extern fn main() {

    let cols:[&gpio::Read<bool>; 3] = [
        &Pin::input_pullup(18),
        &Pin::input_pullup(19),
        &Pin::input_pullup(20)];

    let rows:[&gpio::Write<bool>; 3] = [
        &Pin::output(21, true),
        &Pin::output(22, true),
        &Pin::output(23, true)];

    let leds = [
        &Pin::output(0, true),
        &Pin::output(1, true),
        &Pin::output(2, true),
        &Pin::output(3, true),
        &Pin::output(4, true),
        &Pin::output(5, true),
        &Pin::output(6, true),
        &Pin::output(7, true),
        &Pin::output(8, true)];

    loop {
        scan(&cols[..], &rows[..]);
    }
}

unsafe fn scan(cols:&[&gpio::Read<bool>], rows:&[&gpio::Write<bool>]){
    let output = [
        [0,1,2],
        [3,4,5],
        [6,7,8]
    ];
    for (rindex,  row) in rows.iter().enumerate() {
        row.write(false);
        bindings::delay(1);
        for (cindex, col) in cols.iter().enumerate() {
            bindings::digitalWrite(output[rindex][cindex], if col.read() {bindings::HIGH}else{bindings::LOW} as u8);
        }
        bindings::delay(1);
        row.write(true);
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
