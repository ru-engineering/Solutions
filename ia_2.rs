use rand::distributions::{Distribution, Uniform};

pub enum MyEnum{
    One,
    Two,
    Three,
    Error,
}

fn main(){
    let range  = Uniform::new(0,3);
    let mut rng = rand::thread_rng();
    let error_val=range.sample(&mut rng);
    let error = match error_val{
        1=>MyEnum::One,
        2=>MyEnum::Two,
        3=>MyEnum::Three,
        _=>MyEnum::Error,
    };//usign match to set values
    match error {
        MyEnum::One => {println!("User got a one")}
        MyEnum::Two => {println!("User got a two")}
        MyEnum::Three => {println!("User got a three")}
        MyEnum::Error => {panic!("UNEXPECTED VALUE")}
    }//using match to predict output

}




/*
fn old_main(){
    let range = Uniform::new(0, 3);
    let mut rng = rand::thread_rng();
    let error_val = range.sample(&mut rng);

    if error_val ==0{
        println!("UNEXPECTED VALUE");
        panic!(error_val);
    }else{
        if error_val==1{
            println!("User got a one");
        }else{
            if error_val==2{
                println!("user got a two");
            }else{
                if error_val==3{
                    println!("user got a three");
                }
            }
        }
    }
}
*/

/*
GPIO, simlpy input and output. One sets the mode to reading with various internal functions while the other to writing.
there are also voltage pins (5v and 3.3v) some Ground pins and configurable pins that can be set to PWM, I2C , SPI, UART and more (we dont need more then those anwsers)


enum or Enumeraions allow the user to go throgh the diffrent variants of said type. This can be used to encode data, definie spesific states and much more.

using gdp, add a breakpoint to line 6 gdb -b 6 and run it.
*/
