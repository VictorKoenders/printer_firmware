use hal::gpio::gpiob::{Parts as GpioB, PB10, PB11, PB12, PB13, PB14, PB15, PB8, PB9};
use hal::gpio::{Output, PushPull};
use hal::timer::{Event, Timer};
use motor::Motor;
use stm32f103xx::TIM2;

pub struct AllMotors {
    is_high: bool,
    has_steps_remaining: bool,
    timer: Timer<TIM2>,
    pub x: Motor<PB8<Output<PushPull>>, PB9<Output<PushPull>>>,
    pub y: Motor<PB10<Output<PushPull>>, PB11<Output<PushPull>>>,
    pub z: (
        Motor<PB12<Output<PushPull>>, PB13<Output<PushPull>>>,
        Motor<PB14<Output<PushPull>>, PB15<Output<PushPull>>>,
    ),
}

impl AllMotors {
    pub fn init(mut gpiob: GpioB, timer: Timer<TIM2>) {
        let pb8 = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
        let pb9 = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
        let pb10 = gpiob.pb10.into_push_pull_output(&mut gpiob.crh);
        let pb11 = gpiob.pb11.into_push_pull_output(&mut gpiob.crh);
        let pb12 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
        let pb13 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
        let pb14 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
        let pb15 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);

        let state = AllMotors {
            is_high: false,
            has_steps_remaining: false,
            timer,
            x: Motor::new(pb8, pb9),
            y: Motor::new(pb10, pb11),
            z: (Motor::new(pb12, pb13), Motor::new(pb14, pb15)),
        };

        unsafe { MOTOR_STATE = Some(state) };

        interrupt!(TIM2, motor_timer_tick);
    }

    pub fn get() -> &'static mut AllMotors {
        unsafe { MOTOR_STATE.as_mut().unwrap() }
    }

    pub fn is_running() -> bool {
        AllMotors::get().has_steps_remaining
    }

    pub fn start() {
        let motor_state = AllMotors::get();
        if motor_state.has_steps_remaining {
            // Already running
            return;
        }
        motor_state.has_steps_remaining = true;
        assert!(
            !motor_state.is_high,
            "MotorState is being started while high"
        );
        motor_state.timer.listen(Event::Update);
    }
}

static mut MOTOR_STATE: Option<AllMotors> = None;

#[allow(dead_code)]
fn motor_timer_tick() {
    let motor_state = unsafe { MOTOR_STATE.as_mut().unwrap() };
    if motor_state.is_high {
        motor_state.x.step_low();
        motor_state.y.step_low();
        motor_state.z.0.step_low();
        motor_state.z.1.step_low();
        if !motor_state.has_steps_remaining {
            motor_state.timer.unlisten(Event::Update);
        }
    } else {
        motor_state.x.update();
        motor_state.y.update();
        motor_state.z.0.update();
        motor_state.z.1.update();

        motor_state.has_steps_remaining = motor_state.x.has_steps_remaining()
            || motor_state.y.has_steps_remaining()
            || motor_state.z.0.has_steps_remaining()
            || motor_state.z.1.has_steps_remaining();
    }
    motor_state.is_high = !motor_state.is_high;
}
