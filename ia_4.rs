/*
--------------------------------------------------------------------------------------------------
Offical Klein Solution
This code is complementary with the task IA_4 created 2020
Use with your own caution.
i2c channel connected to accelerometer
pwm0 bound to primitive led code
pwm1 bound to advanced led code
Remeber to add resistors
Primitive led should get brighter then dim agian.
advanced led should reacte to ADXL data specifically X0 raw data
expected output is something like
led exited with panic message is FIRST LED paniced
led exited with panic message is SECOND LED paniced
adxl exited without panic message is true
--------------------------------------------------------------------------------------------------
*/

use std::thread;    //This will be used for real using spawn and sleep
use std::time::{Instant,Duration};//timing using elapsed and from...
use rpi_embedded::{pwm::{Pwm,Polarity,Channel},i2c::I2c};//all the stuff I will need from rpi_e

use std::sync::mpsc;    //message channel/post office using try_rev send and channel

fn main(){
    //First create a i2c communicateion channel with 0x53
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(0x53).unwrap();
    //read the id from 0x00 using a buffer
    let mut buffer=[0u8];
    i2c.cmd_read(0x00 as u8, &mut buffer).unwrap();//cmd read reads from ONE register
    println!("ID is {}",buffer[0]);
    //reading the powerstatus of the adxl on 0x0D
    i2c.cmd_read(0x2D as u8, &mut buffer).unwrap();
    println!("power status is set at {}, setting to 0x08",buffer[0]);
    //making sure the thing is on
    i2c.cmd_write(0x2D,8).unwrap();//8 is wakeup
    thread::sleep(Duration::from_millis(10));//10ms to give it time to wakeup
    //update the user
    i2c.cmd_read(0x2D as u8, &mut buffer).unwrap();
    println!("power status is now at {}",buffer[0]);


    let (tx,rx) = mpsc::channel();//start comm channel
    let timer0 = Instant::now(); //start timer

    let led_1 =thread::spawn(||{//this led is the one in 4
        let pwm = Pwm::with_period(Channel::Pwm0,
             Duration::from_millis(20),
             Duration::from_micros(1800),
             Polarity::Normal, true).unwrap();//set up pwm

        let crash = Instant::now();//timer manual crash
        loop{
            //make the led slowly turn on, 10s sleep here is used so
            //the thread does not lock up the processor
            for i in 0..1000{
                pwm.set_pulse_width(Duration::from_micros(i)).unwrap();//increase by 1 micro second evry 10 milli
                thread::sleep(Duration::from_millis(10));
            }//eta is 10secs
            //same as aboce but in reverse
            for i in 0..1000{
                pwm.set_pulse_width(Duration::from_micros(1500-i)).unwrap();
                thread::sleep(Duration::from_millis(10));
            }//eta is 10 secs
            //check for crash (should always happend as 2x10s)
            if crash.elapsed() > Duration::from_secs(20){
                panic!("First LED paniced");//panics with message
            }
        }

    });

    let timer1 = Instant::now();//timer 1 started
    let led_2 =thread::spawn(move||{//this led is the one in  8
        let pwm = Pwm::with_period(Channel::Pwm1,
             Duration::from_millis(1800),
             Duration::from_micros(1500),
             Polarity::Normal, true).unwrap();
        let crash = Instant::now(); //crash timer
        loop{
            //check for message
            match rx.try_recv(){
                //if there is something available use it, else dont do anything
                Ok(x) => {pwm.set_pulse_width(Duration::from_millis(x*5)).unwrap()},
                Err(_) => {},
            };
            if crash.elapsed() > Duration::from_secs(20){
                panic!("Second LED paniced");//panic with message
            }
        }
    });
    let timer2 = Instant::now();//timer for thread 3
    let adxl = thread::spawn(move ||{
        let stop = Instant::now();//timer for thread stopping
        loop{
            //read from register 0x32 and send it every 100ms
            let mut buf = [0u8;1];
            i2c.cmd_read(0x32 as u8,&mut buf).unwrap();

            tx.send(buf[0] as u64).unwrap();
            thread::sleep(Duration::from_millis(100));
            //check if it is time to stop
            if stop.elapsed() > Duration::from_secs(30){
                return true;//returns a value on exit, used in join
            }
        }
    });

    println!("waiting for led1 to join");
    match led_1.join(){
        Ok(x) => {
            println!("led exited without panic, message is {:?} , timer is at {:?}",
                        x,
                        timer0.elapsed()
                    );
            //note that it doesn't matter what x is here, it can be string int or anything really
        },
        Err(mut x) => {
            println!("led exited with panic, message {:?}, timer is {:?}",
                        x.downcast_mut::<&str>(),
                        timer0.elapsed()
                    );
            //as panic returns Any which is a Box we have to downcast it as a string so we can read it
        },
    };
    println!("waiting for led2 to join");
    match led_2.join(){
        Ok(x) => {
            println!("led exited without panic, message is {:?} , timer is at {:?}",
                        x,
                        timer1.elapsed()
                    );
        },
        Err(mut x) => {
            println!("led exited with panic, message {:?}, timer is {:?}",
                        x.downcast_mut::<&str>(),
                        timer1.elapsed()
                    );
        },
    };

    println!("waiting for accelerometer to join");
    match adxl.join(){
        Ok(x) => {
            println!("accel exited without panic, message is {:?} , timer is at {:?}",
                        x,
                        timer2.elapsed()
                    );
        },
        Err(mut x) => {
            println!("led exited with panic, message {:?}, timer is {:?}",
                        x.downcast_mut::<&str>(),
                        timer2.elapsed()
                    );
        },
    };
    //it doesn't really matter to us to see the realtime crashes, you could have join set up in a callback
    //to see it real time and check every 100ms or something.


}
