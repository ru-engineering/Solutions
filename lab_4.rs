use std::error::Error;
use std::thread;
use std::time::Duration;
use adxl345::adxl345::Adxl;
use rppal::pwm::{Channel, Polarity, Pwm};

use std::sync::mpsc; 

const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 500;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 2500;

fn main() -> Result<(), Box<dyn Error>> {

    let pwm = Pwm::with_period(                     // Initializing Servo on PWM Channel 0 of Raspberry Pi
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true,
    )?;

    let pwm2 = Pwm::with_period(                    // Initializing Servo 2 on PWM Channel 1 of Raspberry Pi
        Channel::Pwm1,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true,
    )?;

    let mut accel = Adxl::new();                    // Initializing Accelerometer 1 with normal address
    accel.start();
    accel.get_offsets();
    accel.get_power_status();
    let mut accel2 = Adxl::new_alt_adress(0x1D);     // Initializing Accelerometer 2 with altered address
    accel2.start();
    accel2.get_offsets();
    accel2.get_power_status();
    
    while accel.get_power_status() != 8 {             // Activating read/write on Accelerometer 1
        println!("Power status is wrong writing 8");
        accel.set_power_status(8);
        thread::sleep(Duration::from_millis(100));
    }
    while accel2.get_power_status() != 8 {              // Activating read/write on Accelerometer 2
        println!("Power status is wrong writing 8");
        accel2.set_power_status(8);
        thread::sleep(Duration::from_millis(100));
    }
    println!("Starting mesurements");

    let (tx, rx) = mpsc::channel();                     // Creating channels to transmit data to Servo thread
    let (txs, rxs) = mpsc::channel();                   // Channel for step size data
    thread::spawn(move || {                             // Initializing parallel thread with moving data from channels
        
        let mut pulse1:u64 = PULSE_NEUTRAL_US;                      // Initializing variable of current pulse width

        loop{
            let acc1_1 = rx.recv().unwrap();                                        // Receiving angular position data 
            let mut step = rxs.recv().unwrap();                                 // Receiving step size data
            let pulse:u64 = ((PULSE_NEUTRAL_US as i64) + acc1_1) as u64;        // Updating needed pulse width
                if pulse > pulse1 {                                             // Case 1: to go to bigger pulse width
                    println!("Pulse1 < Pulse");
                    while pulse1 != pulse {
                        pulse1 = pulse1 + (step as u64);                                // Adding as step to current pulse width
                        let acc1_1 = rx.recv().unwrap();                                // Getting new angular position data
                        step = rxs.recv().unwrap();                                         // Getting new step size data
                        let pulse:u64 = ((PULSE_NEUTRAL_US as i64) + acc1_1) as u64;            // Updating needed pulse width
                        if pulse <= pulse1 {break;}                                             // If direction changed -> leave the Case 1
                        pwm.set_pulse_width(Duration::from_micros(pulse1)).unwrap();            // Let servo go to the updated current pulse width
                        thread::sleep(Duration::from_millis(2));                
                    }

            } else if pulse < pulse1 {                                          // Case 2: to go to smaller pulse width
                println!("Pulse1 > Pulse");
                while pulse1 != pulse {
                    pulse1 = pulse1 - (step as u64);
                    let acc1_1 = rx.recv().unwrap();
                    step = rxs.recv().unwrap();
                    let pulse:u64 = ((PULSE_NEUTRAL_US as i64) + acc1_1) as u64;
                    if pulse >= pulse1 {break;}
                    pwm.set_pulse_width(Duration::from_micros(pulse1)).unwrap();
                    thread::sleep(Duration::from_millis(2));
                }
            } else {println!("Pulse1 = Pulse");}                                // Case 3: to do nothing when needed to do nothing
        }
    });

    let (tx2, rx2) = mpsc::channel();                                           // Channels to transmit data from Accelerometer 2
    let (txs2, rxs2) = mpsc::channel();
    thread::spawn(move || {                                                     // Thread for Servo 2 
        
        let mut pulse1:u64 = PULSE_NEUTRAL_US;
        loop{
            let acc2_1 = rx2.recv().unwrap();
            let mut step2 = rxs2.recv().unwrap();
            let pulse:u64 = ((PULSE_NEUTRAL_US as i64) + acc2_1) as u64;
                if pulse > pulse1 {
                    while pulse1 != pulse {
                        pulse1 = pulse1 + (step2 as u64);
                        let acc2_1 = rx2.recv().unwrap();
                        step2 = rxs2.recv().unwrap();
                        let pulse:u64 = ((PULSE_NEUTRAL_US as i64) + acc2_1) as u64;
                        if pulse <= pulse1 {break;}
                        pwm2.set_pulse_width(Duration::from_micros(pulse1)).unwrap();
                        thread::sleep(Duration::from_millis(2));
                    }

            } else if pulse < pulse1 {
                while pulse1 != pulse {
                    pulse1 = pulse1 - (step2 as u64);
                    let acc2_1 = rx2.recv().unwrap();
                    step2 = rxs2.recv().unwrap();
                    let pulse:u64 = ((PULSE_NEUTRAL_US as i64) + acc2_1) as u64;
                    if pulse >= pulse1 {break;}
                    pwm2.set_pulse_width(Duration::from_micros(pulse1)).unwrap();
                    thread::sleep(Duration::from_millis(2));
                }
            } else { }
        }
    });

    let mut accstep1 = 10;                          // Initializing variable of current step
    let mut accstep2 = 10;
    loop{
        println!("GOT A1 ROLL [ {} ]",accel.roll);          // Printing data from Accelerometers
        println!("GOT A1 PITCH [ {} ]",accel.pitch);
        println!("GOT A2 ROLL [ {} ]",accel2.roll);
        println!("GOT A2 PTICH [ {} ]",accel2.pitch);

        accel.get_data();                                   // Actual getting data from Accelerometers 
        accel2.get_data();
        accel.rotations();                                  // Calculating rotations from read data
        accel2.rotations();
        let acc1 = (accel.roll as i64)*(((PULSE_MAX_US-PULSE_MIN_US)/2) as i64)/180;       // Calculating a difference from neutral pulse width for Servos
        let acc2 = (accel2.roll as i64)*(((PULSE_MAX_US-PULSE_MIN_US)/2) as i64)/180;
        let accs1 = (accel.pitch as i64)*10/180;                                            // Calculating a manipulation variable for steps
        let accs2 = (accel2.pitch as i64)*10/180;
         
        if accs1 > 3 && accstep1 < 20 {accstep1=accstep1 + 1;}                              // Increasing or decreasing the steps in range of [1 to 20]
        if accs1 < -3 && accstep1 > 1 {accstep1=accstep1 - 1;}
        if accs2 > 3 && accstep2 < 20 {accstep2=accstep2 + 1;}
        if accs2 < -3 && accstep2 > 1 {accstep2=accstep2 - 1;}

        thread::sleep(Duration::from_millis(2));
        tx.send(acc1).unwrap();                                                             // Sending data to channels for Servos parallel threads
        txs.send(accstep1).unwrap();
        tx2.send(acc2).unwrap();
        txs2.send(accstep2).unwrap();           
    }
}
