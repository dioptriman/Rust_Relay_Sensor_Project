#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

#[arduino_hal::entry]
fn main() -> !{
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut led = pins.d13.into_output();
    let pir = pins.d12.into_floating_input();
    let magnet = pins.d10.into_pull_up_input();
    let mut relay = pins.d11.into_output();

    loop{
        if pir.is_low() && magnet.is_high(){
            relay.set_high();
            led.set_low();
        } else if pir.is_high() && magnet.is_low(){
            relay.set_low();
            led.set_high();
        }    
    }
}

