
use std::error::Error;
use std::thread;
use std::time::Duration;
use rppal_w_frontend::uart::{Parity, Uart};
fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the primary UART and configure it for 115.2 kbit/s, no
    // parity bit, 8 data bits and 1 stop bit.
    let mut uart = Uart::setup(115_200, Parity::None, 8, 1)?;
println!("UART Initialized");
    // Configure read() to block until at least 1 byte is received.
    uart.set_read_mode(1, Duration::default())?;

    
    loop {
        // Fill the buffer variable with any incoming data.
        thread::sleep(Duration::from_millis(200));
        let s = uart.read().unwrap();
        println!("{}", s);
        
    }
}