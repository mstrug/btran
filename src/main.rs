use std::{env, error::Error, process};

mod transactions;
use transactions::*;


fn read_from_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open(file_name)?; // try to open file
    let reader = std::io::BufReader::new(file);
    
    let data = TransactionProcessor::new_from_csv(reader)?; // try to process transactions
            
    print!("{}", data); // print output from transaction processor to stdout
    
    Ok(())
}


fn main() {
    // check application command line arguments, if input file is not specified inform user and exit with error code
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Application usage:\nbtran <csv input file>");
        process::exit(1);
    } 
    else if let Err(e) = read_from_file(&args[1]) { // if input file is specified try to process transactions
        eprintln!("Error during transaction processing: {}", e); // in case of error print some informations
        process::exit(1);    
    }
}
