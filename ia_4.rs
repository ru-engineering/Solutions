use std::thread;
use std::time::{Instant,Duration};
use rpi_embedded::{pwm::{Pwm,Polarity,Channel},i2c::I2c};

use std::sync::mpsc;

fn main(){
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(0x53).unwrap();

    let mut buffer=[0u8];
    i2c.cmd_read(0x01 as u8, &mut buffer).unwrap();
    println!("ID is {}",buffer[0]);

    i2c.cmd_read(0x2D as u8, &mut buffer).unwrap();
    println!("power status is set at {}, setting to 0x08",buffer[0]);
    i2c.cmd_write(0x2D,8).unwrap();
    thread::sleep(Duration::from_millis(10));
    i2c.cmd_read(0x2D as u8, &mut buffer).unwrap();
    println!("power status is now at {}",buffer[0]);


    let (tx,rx) = mpsc::channel();
    let timer0 = Instant::now();
    let led_1 =thread::spawn(||{//this led is the one in 4
        let pwm = Pwm::with_period(Channel::Pwm0, Duration::from_millis(20), Duration::from_micros(1800), Polarity::Normal, true).unwrap();
        let crash = Instant::now();
        loop{
            for i in 0..1500{
                pwm.set_pulse_width(Duration::from_micros(i)).unwrap();
                thread::sleep(Duration::from_millis(10));
            }
            for i in 0..1500{
                pwm.set_pulse_width(Duration::from_micros(1500-i)).unwrap();
                thread::sleep(Duration::from_millis(10));
            }
            if crash.elapsed() > Duration::from_secs(20){
                panic!("First LED paniced");
            }
        }

    });

    let timer1 = Instant::now();
    let led_2 =thread::spawn(move||{//this led is the one in  8
        let pwm = Pwm::with_period(Channel::Pwm1, Duration::from_millis(1800), Duration::from_micros(1500), Polarity::Normal, true).unwrap();
        let crash = Instant::now();
        loop{
            match rx.try_recv(){
                Ok(x) => {pwm.set_pulse_width(Duration::from_millis(x*5)).unwrap()},
                Err(_) => {},
            };
            if crash.elapsed() > Duration::from_secs(20){
                panic!("Second LED paniced");
            }
        }
    });
    let timer2 = Instant::now();
    let adxl = thread::spawn(move ||{
        let stop = Instant::now();
        loop{
            let mut buf = [0u8;1];
            i2c.cmd_read(0x1E as u8,&mut buf).unwrap();
            tx.send(buf[0] as u64).unwrap();
            thread::sleep(Duration::from_millis(100));
            if stop.elapsed() > Duration::from_secs(30){
                return true;
            }
        }
    });

    println!("waiting for led1 to join");
    match led_1.join(){
        Ok(x) => {println!("led exited without panic, message is {:?} , timer is at {:?}",x,timer0.elapsed())},
        Err(mut x) => {println!("led exited with panic, message {:?}, timer is {:?}",x.downcast_mut::<&str>(),timer0.elapsed())},
    };
    println!("waiting for led2 to join");
    match led_2.join(){
        Ok(x) => {println!("led exited without panic, message is {:?} , timer is at {:?}",x,timer1.elapsed())},
        Err(mut x) => {println!("led exited with panic, message {:?}, timer is {:?}",x.downcast_mut::<&str>(),timer1.elapsed())},
    };
    println!("waiting for accelerometer to join");
    match adxl.join(){
        Ok(x) => {println!("accel exited without panic, message is {:?} , timer is at {:?}",x,timer2.elapsed())},
        Err(mut x) => {println!("led exited with panic, message {:?}, timer is {:?}",x.downcast_mut::<&str>(),timer2.elapsed())},
    };



}
