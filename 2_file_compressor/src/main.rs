
extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    //here we are taking the input
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //here we are generating the output
    let output = File::create(args().nth(2).unwrap()).unwrap();
    //here we are compressing the input into the output file using gzip compression.
    let mut encoder = GzEncoder::new(output, Compression::default());
    //we are measuring the time taken to compress the input.
    let start = Instant::now();
    //copying the input to the output file.  If there is an error, we will print it out.  If not, we will continue.  
    //Note that we are not closing the input file here.  We let it be closed when the main function exits.  
    //If we close it, then the input file will be locked until the compression is finished.  
    //This is a potential bottleneck if the input file is large.  In that case, we could consider reading the input file
    copy(&mut input, &mut encoder).unwrap();
    //we are measuring the time taken to compress the input.
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}