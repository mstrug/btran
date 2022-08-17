use std::collections::HashMap;

// data types are defined in separate submodule
pub mod data_types;
use data_types::*;

mod client_data;
use client_data::*;


// Transaction Processor struct which encapsulates all requried data
#[derive(Default)]
pub struct TransactionProcessor
{
    data: HashMap<TransactionClientType, ClientData> // using Hash map to map client IDs to list of client transactions and current client state
}

impl TransactionProcessor {
    
    /// Creates TransactionProcessor instance with empty internal data
    pub fn new() -> TransactionProcessor {
        TransactionProcessor { ..Default::default() }
    }
    
    /// Creates TransactionProcessor and processes data from passed io::Read
    pub fn new_from_csv<R>(reader: R) -> Result<TransactionProcessor, Box<dyn std::error::Error>>
        where R: std::io::Read 
    {
        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(reader);

        let mut tp: TransactionProcessor = TransactionProcessor::new();
    
        // processing loop - each record one by one
        for result in rdr.deserialize() {            
            let record: Transaction = result?;            
            tp.process_transaction(record)?;            
        }
        
        Ok(tp)
    }
    
    /// Processed specified transaction
    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<(),ClientDataError> {
        // firstly check if client with specified ID exists in Hash map
        if let Some(client) = self.data.get_mut(&transaction.client) {
            client.add_transaction(&transaction)?
        } else {
            // if not then create new client data and add it to Hash map (client ID is the key)
            let mut client = ClientData::new();
            client.add_transaction(&transaction)?;
            self.data.insert(transaction.client, client);
        }
        Ok(())
    }
}
    
/// Disply trait implementation, which prints TransactionProcessor in csv format
impl std::fmt::Display for TransactionProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "client,available,held,total,locked").ok();
        for (client, data) in &self.data {
            writeln!(f, "{},{},{},{},{}", client, data.available, data.held, data.available + data.held, data.locked).ok();
        }
        Ok(())
    }
}




/// TransactionProcessor tests
#[cfg(test)]
mod tests {
    use super::*;
    type OutputType = Vec<(TransactionClientType, TransactionAmountType, TransactionAmountType, bool)>; // client id, available, held, account locked

    fn validate_tp(input: &str, output: OutputType) {
        let tp = TransactionProcessor::new_from_csv(input.as_bytes());
        assert!( tp.is_ok() );
        
        for (client,available,held,locked) in output {
            let c = tp.as_ref().unwrap().data.get( &client ).unwrap(); // client must be in hash map if not -> test should fail
            assert_eq!(c.available, available);
            assert_eq!(c.held, held);
            assert_eq!(c.locked, locked);
        }
    }

    #[test]
    fn test_input_1() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 3.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 3.0";
        let output: OutputType = vec![ (1,1.5,0.0,false), (2,0.0,0.0,false) ];
        validate_tp(input, output);
    }

    #[test]
    fn test_input_2() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 3.0"; // this transaction should fail because of negative result
        let output: OutputType = vec![ (1,1.5,0.0,false), (2,2.0,0.0,false) ];
        validate_tp(input, output);
    }
    
    #[test]
    fn test_input_3() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.1234
deposit, 2, 2, 2.1234
deposit, 1, 3, 2.1234
withdrawal, 1, 4, 1.1234
withdrawal, 2, 5, 3.1234"; // this transaction should fail because of negative result
        let output: OutputType = vec![ (1,2.1234,0.0,false), (2,2.1234,0.0,false) ];
        validate_tp(input, output);
    }
    
    #[test]
    fn test_dispute_1() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
dispute, 1, 2
dispute, 1, 3"; // this should be ignored
        let output: OutputType = vec![ (1,1.0,2.0,false) ];
        validate_tp(input, output);
    }
    
    #[test]
    fn test_dispute_2() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
dispute, 1, 2
dispute, 1, 1
deposit, 1, 3, 6.0";
        let output: OutputType = vec![ (1,6.0,3.0,false) ];
        validate_tp(input, output);
    }

    #[test]
    fn test_dispute_3() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
withdrawal, 1, 3, 2.5
dispute, 1, 2";
        let output: OutputType = vec![ (1,-1.5,2.0,false) ];
        validate_tp(input, output);
    }

    #[test]
    fn test_resolve_1() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
withdrawal, 1, 3, 2.5
dispute, 1, 2
deposit, 1, 4, 1.0
resolve, 1, 2";
        let output: OutputType = vec![ (1,1.5,0.0,false) ];
        validate_tp(input, output);
    }

    #[test]
    fn test_resolve_2() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
withdrawal, 1, 3, 2.5
dispute, 1, 2
dispute, 1, 1
deposit, 1, 4, 1.0
resolve, 1, 2,
resolve, 1, 1";
        let output: OutputType = vec![ (1,1.5,0.0,false) ];
        validate_tp(input, output);
    }

    #[test]
    fn test_resolve_3() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
withdrawal, 1, 3, 2.5
dispute, 1, 2
deposit, 1, 4, 1.0
resolve, 1, 3"; // this should be ignored
        let output: OutputType = vec![ (1,-0.5,2.0,false) ];
        validate_tp(input, output);
    }
    
    #[test]
    fn test_charge_back_1() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
withdrawal, 1, 3, 2.5
dispute, 1, 2
deposit, 1, 4, 1.0
chargeback, 1, 2";
        let output: OutputType = vec![ (1,-0.5,0.0,true) ];
        validate_tp(input, output);
    }
    
    #[test]
    fn test_charge_back_2() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
withdrawal, 1, 3, 2.5
dispute, 1, 2
deposit, 1, 4, 1.0
dispute, 1, 4
chargeback, 1, 2
deposit, 1, 5, 1.0
chargeback, 1, 4";
        let output: OutputType = vec![ (1,-0.5,0.0,true) ];
        validate_tp(input, output);
    }
    
    #[test]
    fn test_dispute_mix() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
withdrawal, 1, 3, 2.5
dispute, 1, 2
deposit, 1, 4, 1.0
dispute, 1, 4
chargeback, 1, 2
deposit, 1, 5, 1.0
resolve, 1, 4";
        let output: OutputType = vec![ (1,0.5,0.0,true) ];
        validate_tp(input, output);
    }   
    
    #[test]
    fn test_output_1() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
withdrawal, 1, 3, 1.5
dispute, 1, 2";
        
        let output = 
"client,available,held,total,locked
1,-0.5,2,1.5,false
";

        let tp = TransactionProcessor::new_from_csv(input.as_bytes());
        assert!( tp.is_ok() );
        let out = format!("{}", tp.unwrap());
        assert_eq!(out, output);
    }
    
    #[test]
    fn test_output_2() {
        let input = 
"type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 1, 2, 2.0
withdrawal, 1, 3, 1.5
dispute, 1, 2
deposit, 1, 4, 1.0001
chargeback, 1, 2";

        let output = 
"client,available,held,total,locked
1,0.5001,0,0.5001,true
";

        let tp = TransactionProcessor::new_from_csv(input.as_bytes());
        assert!( tp.is_ok() );
        let out = format!("{}", tp.unwrap());
        assert_eq!(out, output);
    }
    
    #[test]
    fn test_big_1() {
        let mut input = String::from("type, client, tx, amount\n");
        input.reserve(0xffff*12 + 0xffff * 2);
        
        for i in 1..0xffff {
            input.push_str(&format!("deposit,{},{},{}\n",i,i,i));
        }

        let tp = TransactionProcessor::new_from_csv(input.as_bytes());
        assert!( tp.is_ok() );
        
        for i in 1..0xffff {
            let c = tp.as_ref().unwrap().data.get( &i ).unwrap(); // client must be in hash map if not -> test should fail
            assert_eq!(c.available, i as f32);
            assert_eq!(c.held, 0.0);
            assert_eq!(c.locked, false);
        }
    }
    
    #[test]
    fn test_big_2() {
        let mut input = String::from("type, client, tx, amount\n");
        input.reserve(0xffff*14);
        
        let mut tx = 1;
        for i in 1..0xff {
            for j in 1..0xff {
                input.push_str(&format!("deposit,{},{},{}\n",i,tx,j));
                tx += 1;
            }
        }

        let tp = TransactionProcessor::new_from_csv(input.as_bytes());
        assert!( tp.is_ok() );
        
        for i in 1..0xff {
            let c = tp.as_ref().unwrap().data.get( &i ).unwrap(); // client must be in hash map if not -> test should fail
            assert_eq!(c.available, (0xff*0xfe/2) as f32);
            assert_eq!(c.held, 0.0);
            assert_eq!(c.locked, false);
        }
    }
}



