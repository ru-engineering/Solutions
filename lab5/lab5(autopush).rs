use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let mut iter = 0;
    let iter1 = iter;
    println!("{}", iter1);
    let s: String = iter1.to_string();
    thread::sleep(Duration::from_secs(1));
    let mut ofile = File::create("auto_push.txt").expect("unable to create file");    
    let mut o1file = File::create("file_under_vc.txt").expect("unable to create file");    
    o1file.write_all(b"Version changed").expect("unable to write");
    ofile.write_all(s.as_bytes()).expect("unable to write");
    ofile.write_all(b"
").expect("unable to write");
    
    thread::spawn(|| { 
        loop{
            thread::sleep(Duration::from_secs(6));
            println!("A0");
            Command::new("svn") .arg("add") .arg("auto_push.txt") .spawn()
            .expect("add command failed to start")
            ;
            println!("A1");
            thread::sleep(Duration::from_secs(2));
            println!("A2"); 
            Command::new("svn") .arg("cleanup") .arg("cleanup") .spawn()
            .expect("cleanup command failed to start")
            ;
            println!("A3");
            thread::sleep(Duration::from_secs(2));            
            Command::new("svn") .arg("commit") .arg("--username") .arg("***") .arg("--password") .arg("***")
            .arg("--force-log") .arg("-F") .arg("file_under_vc.txt") .arg("auto_push.txt")
            .spawn()
            .expect("commit command failed to start")
            ;
            println!("A4");
        }
    }); 

    loop {
        iter = iter + 1;
        thread::sleep(Duration::from_millis(1000));
        println!("{}", iter);
        let iter1 = iter;
        let s: String = iter1.to_string(); 
        ofile.write_all(s.as_bytes()).expect("unable to write");
        ofile.write_all(b"
").expect("unable to write");
 
        
    }
}
