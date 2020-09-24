use rpi_embedded::pwm::{Pwm,Channel,Polarity};
use std::time::Duration;
use std::thread;

fn main(){
let stp = Stepper::new(Channel::Pwm0, Duration::from_millis(100), Duration::from_millis(0));
stp.steps(100);
}

struct Stepper{
    pwm : Pwm,
    period : Duration,
}
impl Stepper{
    fn new(channel:Channel,period_inn:Duration,pulse:Duration)-> Self{
        let mut _pwm = Pwm::with_period(channel, period_inn, pulse, Polarity::Normal, false).unwrap();
        Self{
            pwm: _pwm,
            period : period_inn,
        }
    }
    fn steps(self,step:u32){
        self.pwm.enable().unwrap();
        thread::sleep(self.period*step);
        self.pwm.disable().unwrap();
    }
}
