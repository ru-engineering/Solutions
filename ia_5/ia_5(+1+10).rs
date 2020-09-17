use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;

const ADDR_NANO3: u16 = 0x68;

fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;

    i2c.set_slave_address(ADDR_NANO3)?;

    let mut n1 = [1 as u8;1];
    let mut n2 = [1 as u8;1];
    let a:u8 = 10;
    loop {
        n2[0] = n1[0];
        println!("Pi sends {}", n2[0]);
        i2c.write(&mut n2)?;
        while n2[0] == n1[0]{
        i2c.read(&mut n1)?;
        }

        println!("Pi gets {}", n1[0]); 
        if n1[0] > 244 { n1[0] = 1; }
        n1[0] = n1[0] + a;
        thread::sleep(Duration::from_millis(1000));
        }
}