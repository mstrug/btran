use std::collections::HashMap;
use super::data_types::*;

/// Error definition
#[derive(Debug)]
pub enum ClientDataError {
    InvalidInput
}
impl std::fmt::Display for ClientDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClientDataError::InvalidInput => write!(f, "Invalid csv input")
        }
    }
}
impl std::error::Error for ClientDataError {}

/// Encapsulation of particular transaction value and information if transaction is under dispute
struct ClientDataTransaction {
    amount: TransactionAmountType,
    under_dispute: bool
}

/// Client state representation with list of all transactions
#[derive(Default)]
pub struct ClientData {
    transactions: HashMap<TransactionIdType, ClientDataTransaction>,
    pub available: TransactionAmountType,
    pub held: TransactionAmountType,
    pub locked: bool
}

impl ClientData {

    /// Creates new ClientData with empty values
    pub fn new() -> ClientData {
        ClientData { .. Default::default() }
    }
    
    /// Main logic implementation basing on transaction type
    pub fn add_transaction(&mut self, transaction: &Transaction) -> Result<(),ClientDataError> {
        match transaction.r#type {
            TransactionType::Deposit => {
                if let Some(amount) = transaction.amount {                    
                    self.transactions.insert(transaction.tx, ClientDataTransaction { amount, under_dispute: false });
                    self.available += amount;
                    Ok(())
                } else {
                    Err(ClientDataError::InvalidInput)
                }
            }
            TransactionType::Withdrawal => {
                if let Some(amount) = transaction.amount {
                    if self.available - amount >= 0.0 {
                        self.transactions.insert(transaction.tx, ClientDataTransaction { amount, under_dispute: false });
                        self.available -= amount;
                    } // else -> ignore                    
                    Ok(())
                } else {
                    Err(ClientDataError::InvalidInput)
                }
            }
            TransactionType::Dispute => {
                if transaction.amount.is_none() {
                    if let Some(client_transaction) = self.transactions.get_mut(&transaction.tx) {
                        self.available -= client_transaction.amount;
                        self.held += client_transaction.amount;
                        client_transaction.under_dispute = true;
                    } // else -> ignore
                    Ok(())
                } else {
                    Err(ClientDataError::InvalidInput)
                }
            }
            TransactionType::Resolve => {
                if transaction.amount.is_none() {
                    if let Some(client_transaction) = self.transactions.get_mut(&transaction.tx) {
                        if client_transaction.under_dispute {
                            self.available += client_transaction.amount;
                            self.held -= client_transaction.amount;
                            client_transaction.under_dispute = false;
                        } // else -> ignore
                    } // else -> ignore
                    Ok(())
                } else {
                    Err(ClientDataError::InvalidInput)
                }
            }
            TransactionType::Chargeback => {
                if transaction.amount.is_none() {
                    if let Some(client_transaction) = self.transactions.get_mut(&transaction.tx) {
                        if client_transaction.under_dispute {
                            self.held -= client_transaction.amount;
                            self.locked = true;
                            client_transaction.under_dispute = false;
                        } // else -> ignore
                    } // else -> ignore
                    Ok(())
                } else {
                    Err(ClientDataError::InvalidInput)
                }
            }
        }
    }
}
