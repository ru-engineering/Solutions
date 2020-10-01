use rpi_embedded::uart::Uart;
use rpi_embedded::uart::Parity;
use std::time::Duration;
use std::thread;

fn main(){
    let mut uart = Uart::new(9_600, Parity::None, 8, 1).unwrap();
    uart.set_read_mode(1, Duration::default()).unwrap();

    loop{
    let s = uart.read_line().unwrap();
    let spl = s.split(",");
    let vectstr: Vec<&str> = spl.collect();
    if vectstr[0] == "$GPGGA" {
        let nv: Vec<char> = vectstr[2].chars().collect();
        let ndeg = [nv[0],nv[1]].iter().collect::<String>().parse::<f64>().unwrap();
        let nmin = [nv[2],nv[3],nv[4],nv[5],nv[6],nv[7],nv[8]].iter().collect::<String>().parse::<f64>().unwrap();
        let north = ndeg + nmin/60.0; 
        let wv: Vec<char> = vectstr[4].chars().collect();
        let wdeg = [wv[0],wv[1],wv[2]].iter().collect::<String>().parse::<f64>().unwrap();
        let wmin = [wv[3],wv[4],wv[5],wv[6],wv[7],wv[8],wv[9]].iter().collect::<String>().parse::<f64>().unwrap();
        let west = wdeg + wmin/60.0;
        println!("{} N {} W", north, west);}
    thread::sleep(Duration::from_millis(200));
}
} 