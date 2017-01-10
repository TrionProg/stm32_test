
//#![deny(unsafe_code)]
#![no_std]
#![no_main]
#![feature(const_fn)]
#![feature(asm)]

#[macro_use]
extern crate f3;

use f3::Box;

use f3::gpioe::{E8,E9};
use f3::output::{Output,OutputFun};
//use f3::stm32::gpio::functions::output::OutputPin;

//use f3::delay;

static led1:Output<E8>=Output::new();
//static led2:Output<E9>=Output::new(); // E9 do not support Output function!

use f3::stm::timer::{BasicTimer, BasicTimerConfig};
use f3::timer6::Timer6;

static tim6:BasicTimer<Timer6>=BasicTimer::new(
    BasicTimerConfig{
        prescaler:8000,
    }
);

/*
fn activate(pin:& OutputFun<Pin=OutputPin<GpioRB=f3::memory_map::gpioe::GpioRB>>){
    pin.turn(true);
}
*/

/*
fn activate(led:&Output<E8>){
    led.turn(true);
}
*/

fn event(){
    led1.turn(false);
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    unsafe{
        f3::int_en();
    }

    led1.enable();
    //led1.turn(true);

    tim6.config();

    /*
    let state=true;

    tim6.delay(500, Some(Box::new(move ||{ //TODO Hide Box::new if it is possible
        led1.turn(state);
    })), None);
    */


    loop{
        led1.turn(true);
        tim6.wait(500);
        led1.turn(false);
        tim6.wait(500);
    }


    //iprint!("Hello, world!");
    loop{}

}
