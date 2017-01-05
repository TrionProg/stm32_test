
#![deny(unsafe_code)]
#![no_std]
#![no_main]
#![feature(const_fn)]

#[macro_use]
extern crate f3;

//use core::mem;

/*

struct E0;

struct Spi{
    rate:i8,
}

static spi:Spi=Spi::unconfigured();

use f3::stm32::pin::Pin;

fn config<C:f3::stm32::pin::Pin>(c:C){
    panic!("Hello, world! {}",f3::stm32::pin::Pin::get_index(&c));
}

*/



use f3::gpioe::{E8,E9};
use f3::output::{Output,OutputFun};
//use f3::stm32::gpio::functions::output::OutputPin;

//use f3::delay;

static led1:Output<E8>=Output::new();
//static led2:Output<E9>=Output::new();



/*
fn activate(pin:& OutputFun<Pin=OutputPin<GpioRB=f3::memory_map::gpio::gpio_rb::Gpio>>){
    pin.turn(true);
}
*/

/*
fn activate(led:&Output<E8>){
    led.turn(true);
}
*/

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    led1.enable();
    led1.turn(true);
    //led2.enable();

    /*
    {
        let l2=led2.enable_temporary();

        l2.turn(true);
        delay::ms(500);
        l2.turn(false);
        delay::ms(500);
    }

    //activate(&led1);
    //activate(&led2);

    //let arr[=[&led1,&led2];

    //iprint!("Hello, world!");

    loop{
        //led2.turn(false);
        led1.turn(true);
        delay::ms(500);
        led1.turn(false);
        //led2.turn(true);
        delay::ms(500);
    }

    //let e0=f3::gpioe::E0;
    //panic!("Hello, world! {}",f3::stm32::pin::Pin::get_index(&e0));
    //config(e0);

    //iprint!("Hello, world!");
*/
    loop{}

}
