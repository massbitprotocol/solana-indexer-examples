use std::sync::Arc;
use massbit_solana_sdk::smart_contract::{InstructionParser, SmartContractProxy};
use massbit_solana_sdk::types::SolanaBlock;
use crate::generated::handler::Handler;
use crate::generated::instruction::*;
//use crate::models::*;
use crate::ADDRESS;
use crate::SOLANA_CLIENT;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_client::rpc_response::RpcResult;
use solana_client::{client_error::Result as ClientResult, rpc_request::RpcRequest};
use solana_program::account_info::AccountInfo;
use solana_program::instruction::CompiledInstruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use solana_transaction_status::{parse_instruction, ConfirmedBlock, TransactionWithStatusMeta};
use massbit_solana_sdk::transport::interface::InterfaceRegistrar;
use uuid::Uuid;


pub fn handle_block(interface: &mut dyn InstructionParser, block: &SolanaBlock) -> Result<(), Box<dyn std::error::Error>> {
    println!("Start handle_block, block.block_number: {}", block.block_number);
    for (tx_ind, tran) in block.block.transactions.iter().enumerate() {
        if tran
            .transaction
            .message
            .account_keys
            .iter()
            .any(|key| key.to_string().as_str() == ADDRESS)
        {
            let entities = parse_instructions(interface, block, tran, tx_ind);
        }
    }
    Ok(())
}
fn parse_instructions(interface: &mut dyn InstructionParser, block: &SolanaBlock, tran: &TransactionWithStatusMeta, tx_ind: usize) {
    for (ind, inst) in tran.transaction.message.instructions.iter().enumerate() {
        let program_key = inst.program_id(tran.transaction.message.account_keys.as_slice());
        if program_key.to_string().as_str() == ADDRESS {
            let mut accounts = Vec::default();
            let mut work = |unique_ind: usize, acc_ind: usize| {
                if let Some(key) = tran.transaction.message.account_keys.get(acc_ind) {
                    accounts.push(key.clone());
                };
                Ok(())
            };
            inst.visit_each_account(&mut work);

            let handler = Handler {};
            // Fixme: Get account_infos from chain take a lot of time. For now, use empty vector.
            println!("Start unpack_instruction, inst {:?}", &inst);
            match interface.unpack_instruction(inst.data.as_slice()) {
                Ok(trans_value) => {
                    println!("unpack_instruction Ok, trans_value: {:?}", &trans_value);
                    handler.process(block, tran, program_key, &accounts, trans_value);
                },
                Err(e) => {
                    println!("Error unpack_instruction: {:?}",e);
                }
            }
        }
    }
}
