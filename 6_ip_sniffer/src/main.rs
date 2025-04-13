use std::env;
use std::net::IpAddr;

struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16
}

impl Arguments{
    fn new(args: &[String])->Result<Arguments, &'static str>{
      if args.len() < 2{
        return Err("not enough arguments");
      } else if args.len() > 4 {
        return err("too many arguments");
      }
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();


}