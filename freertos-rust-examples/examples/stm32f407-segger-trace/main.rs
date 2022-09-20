#![no_std]
#![no_main]
// For allocator
#![feature(lang_items)]
#![feature(alloc_error_handler)]

use cortex_m::asm;
use cortex_m_rt::exception;
use cortex_m_rt::{entry, ExceptionFrame};
// use embedded_hal::digital::v2::OutputPin;
use freertos_rust::*;
use core::alloc::Layout;
use stm32f4xx_hal::gpio::*;

use cortex_m;
use cortex_m::interrupt::free;
use stm32f4xx_hal as hal;

use hal::{
    gpio::{self, Output, PushPull},
    pac,
    prelude::*
};

extern crate panic_halt; // panic handler

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let mut output1 = gpioa.pa8.into_push_pull_output();
    output1.set_low();

    let mut output2 = gpioa.pa9.into_push_pull_output();
    output2.set_low();

    let mut output3 = gpioa.pa10.into_push_pull_output();
    output3.set_low();

    Task::new().name("led1").stack_size(512).priority(TaskPriority(2)).start(move |_| {
        loop{
            freertos_rust::CurrentTask::delay(Duration::ms(666));
            output1.toggle();
        }
    }).unwrap();

    Task::new().name("led2").stack_size(512).priority(TaskPriority(2)).start(move |_| {
        loop{
            freertos_rust::CurrentTask::delay(Duration::ms(1234));
            output2.toggle();
        }
    }).unwrap();

    Task::new().name("led3").stack_size(512).priority(TaskPriority(4)).start(move |_| {
        loop{
            freertos_rust::CurrentTask::delay(Duration::ms(2345));
            output3.toggle();
        }
    }).unwrap();

    FreeRtosUtils::start_scheduler();
}

// busywait delays, for use inside exception handlers
fn delay() {
    let mut _i = 0;
    for _ in 0..2_00 {
        _i += 1;
    }
}

fn delay_n(n: i32) {
    for _ in 0..n {
        delay();
    }
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
// custom default handler
// irqn is negative for Cortex-M exceptions
// irqn is positive for device specific (line IRQ)
// set_led(true);(true);
// panic!("Exception: {}", irqn);
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
// Blink 3 times long when exception occures
    delay_n(10);
    for _ in 0..3 {
        // set_led(true);
        // delay_n(1000);
        // set_led(false);
        // delay_n(555);
    }
    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    //set_led(true);
    asm::bkpt();
    loop {}
}

#[no_mangle]
fn vApplicationStackOverflowHook(pxTask: FreeRtosTaskHandle, pcTaskName: FreeRtosCharPtr) {
    asm::bkpt();
}
