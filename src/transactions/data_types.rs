use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback
}

pub type TransactionClientType = u16;
pub type TransactionIdType = u32;
pub type TransactionAmountType = f32;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub r#type: TransactionType,
    pub client: TransactionClientType,
    pub tx: TransactionIdType,
    pub amount: Option<TransactionAmountType>
}
