use rpi_embedded::gpio::Gpio;
use rpi_embedded::gpio::{OutputPin,InputPin};
use std::time::{Duration,Instant};
//note not using thread becuse sleep is ineficeant
const RED_PIN :u8= 21;
const YELLOW_PIN:u8 = 22;
const GREEN_PIN:u8 = 23;

const BUTTON_PIN: u8 =24;

fn main(){
    let mut sl = StopLights::new(Duration::from_millis(5000));
    sl.set_location(Location::EU);
    loop{
        sl.state_machine();
    }
}


/*
Cant really show moving between files. but it is explained in the book. simply use mod FILENAME
 to add external file.
*/

pub struct StopLights{
    pub red : bool,
    pub yellow : bool,
    pub green : bool,
    pub wait_timer : Duration,
    pub max_time : Duration,
    pub state : i32,
    pub location : Location,

    red_pin : OutputPin,
    green_pin : OutputPin,
    yellow_pin : OutputPin,
    walk_button: InputPin,
}

impl StopLights{
    //Initlizes the Struct with defult values
    pub fn new(max :Duration)-> Self{
        let mut _out =Self{
            red: true,
            yellow: false,
            green: false,
            wait_timer: Duration::from_millis(1000),
            max_time: max,
            state: 0,
            location: Location::NA,
            red_pin: Gpio::output(RED_PIN).unwrap(),
            green_pin: Gpio::output(GREEN_PIN).unwrap(),
            yellow_pin: Gpio::output(YELLOW_PIN).unwrap(),
            walk_button : Gpio::input(BUTTON_PIN).unwrap(),
        };
        _out.set();
        _out
    }
    //FSM
    pub fn state_machine(&mut self){
        match self.state{
            0 => {
                    match self.location{
                    Location::NA =>{ panic!("location not initilized")}
                    _ =>{self.state = 1}
                }
            }
            1 => {
                match self.location{
                    Location::US => {
                        self.red = false;
                        self.yellow = true;
                    }
                    Location::EU => {
                        self.red = true;
                        self.yellow = true;
                    }
                    Location::NA => {panic!("location is not set right")}
                }
                self.set();
                self.button_timer(self.wait_timer); //transient state wait
                self.state = 2;
            }
            2 => {
                self.red = false;
                self.yellow = false;
                self.green = true;
                self.set();
                self.button_timer(self.max_time);
                self.state = 3;
            }
            3 => {
                self.red = false;
                self.green = false;
                self.yellow = true;
                self.set();
                self.button_timer(self.wait_timer); //transient state wait
                self.state = 4;
            }
            4 =>{
                self.red = true;
                self.yellow = false;
                self.green = false;
                self.set();
                self.button_timer(self.max_time);
                self.state = 1;
            }
            _ => {panic!("UNKNOWN STATE")}
        }
    }
    //translates bool to real
    fn set(&mut self){
        if self.red {
            self.red_pin.set_high();
        }else{
            self.red_pin.set_low();
        }
        if self.yellow {
            self.yellow_pin.set_high();
        }else{
            self.yellow_pin.set_low();
        }
        if self.green {
            self.green_pin.set_high();
        }else{
            self.green_pin.set_low();
        }
    }
    fn button_timer(&mut self,timer :Duration){
        let runner = Instant::now();
        while runner.elapsed() < timer {
            if self.walk_button.is_high(){
                break;
            }
        }
    }
    pub fn set_location(&mut self,loc:Location){
        self.location = loc;
    }
}

pub enum Location{
    US,
    EU,
    NA,         //not avialable, good starting state
}

//END OF extern

/*
fn main() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

the reference is not mutable so you can not change the string by pushing. mutable means you can change it
to fix it

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


the later code

let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);

the problem here is that you cant take the same string twice as a mutable reference which is forbidden becuse of data races
now to fix it i simply would clone

let mut s = String::from("hello");
let r1 = s.clone;
let r2 = s.clone;
println!("{}, {}", r1, r2);

or take it as a immutable reference if possible
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);
but we always loose functionality as we can not change the original without it failing
so the anwser is.
IT IS IMPOSIBLE TO DO THIS WITHOUT MAKING A SACRIFICE
aka trick question

*/
