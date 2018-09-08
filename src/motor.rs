use embedded_hal::digital::OutputPin;

pub struct Motor<StepPin: OutputPin, DirectionPin: OutputPin> {
    step: StepPin,
    direction: DirectionPin,
    current_direction: Direction,

    steps_remaining: u16,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl<StepPin: OutputPin, DirectionPin: OutputPin> Motor<StepPin, DirectionPin> {
    pub fn new(step: StepPin, mut direction: DirectionPin) -> Self {
        direction.set_low();
        Motor {
            step,
            direction,
            current_direction: Direction::Left,
            steps_remaining: 0,
        }
    }

    pub fn has_steps_remaining(&self) -> bool {
        self.steps_remaining > 0
    }

    pub fn update(&mut self) {
        if self.steps_remaining > 0 {
            self.step_high();
            self.steps_remaining -= 1;
        }
    }

    pub fn steps(&mut self, direction: Direction, steps: u16) {
        self.set_direction(direction);
        self.steps_remaining += steps;
    }

    pub fn step_low(&mut self) {
        self.step.set_low();
    }

    fn step_high(&mut self) {
        self.step.set_high();
    }

    fn set_direction(&mut self, direction: Direction) {
        if direction != self.current_direction {
            self.current_direction = direction;
            match direction {
                Direction::Left => self.direction.set_low(),
                Direction::Right => self.direction.set_high(),
            }
        }
    }
}
