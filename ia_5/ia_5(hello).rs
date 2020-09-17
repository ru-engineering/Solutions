use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;

const ADDR_NANO3: u16 = 0x68;

fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;

    i2c.set_slave_address(ADDR_NANO3)?;

    let mut n1 = [1 as u8;5];
    let mut n2 = [1 as u8;1];
    let a:u8 = 10;
    loop {
        println!("Pi sends {}", n2[0]);
        i2c.write(&mut n2)?;
        thread::sleep(Duration::from_millis(200));
        i2c.read(&mut n1)?;
        if n1[1] != 255 {
            let n1_1 = n1;
            let n1_2 = n1;
            let string1 = String::from_utf8_lossy(&n1_1);
            println!("From u8 to String_utf8_lossy: {}", string1);

            let mut n1_v = vec![];
            n1_v.extend_from_slice(&n1_2);
            let string2 = String::from_utf8(n1_v).unwrap();
            println!("From u8 to vector to String_utf8_result: {}", string2);
        }

        if n2[0] > 244 { n2[0] = 1; }
        n2[0] = n2[0] + a;
        thread::sleep(Duration::from_millis(200));
        }
}