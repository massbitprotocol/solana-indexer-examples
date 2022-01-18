use crate::generated::instruction::*;
use crate::STORE;
//use crate::models::*;
use serde_json;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use solana_transaction_status::{parse_instruction, ConfirmedBlock, TransactionWithStatusMeta};
use std::collections::HashMap;
use massbit_solana_sdk::entity::{Attribute, Entity, Value};
use transport::TransportValue;
use massbit_solana_sdk::types::SolanaBlock;
use serde_json::to_string;
use uuid::Uuid;

pub struct Handler {}
impl Handler {
    pub fn process(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: TransportValue,
    ) {
        println!(
            "Process block {} with input {:?}",
            block.block_number, input
        );
        match input.name.as_str() {
            "SetStoreIndex" => {
                self.process_set_store_index(block, transaction, program_id, accounts, input);
            }
            _ => {
                println!("Not support instruction {:?}",input.name.as_str());
            }

        }
    }
    pub fn process_set_store_index(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );

        //Entity::from(input).save("Initialize");
        println!("Write to db {:?}",input);
        Ok(())
    }

}
