#![no_std]
#![no_main]
// #![allow(dead_code, unused_variables, unused_mut, unused_imports)]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f103xx_hal as hal;
#[macro_use]
extern crate stm32f103xx;
extern crate embedded_hal;
extern crate nb;

mod motor;
mod motor_state;

use hal::prelude::*;
use hal::stm32f103xx::Peripherals;
use hal::timer::Timer;
use motor::Direction;
use motor_state::AllMotors;
use rt::{entry, exception};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let timer = Timer::tim2(dp.TIM2, 3000.hz(), clocks, &mut rcc.apb1);

    AllMotors::init(gpiob, timer);

    AllMotors::get().x.steps(Direction::Right, 500);
    AllMotors::get().y.steps(Direction::Left, 1000);

    AllMotors::start();

    while motor_state::AllMotors::is_running() {
        cortex_m::asm::nop();
    }

    loop {}
}

#[exception]
fn HardFault(ef: &rt::ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
