use std::sync::Arc;
use massbit_solana_sdk::smart_contract::{TransportValue, InstructionParser, SmartContractProxy};
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
use uuid::Uuid;

pub fn handle_block(proxy: Arc<SmartContractProxy>, block: &SolanaBlock) -> Result<(), Box<dyn std::error::Error>> {
    for (tx_ind, tran) in block.block.transactions.iter().enumerate() {
        if tran
            .transaction
            .message
            .account_keys
            .iter()
            .any(|key| key.to_string().as_str() == ADDRESS)
        {
            let entities = parse_instructions(proxy.clone(), block, tran, tx_ind);
        }
    }
    Ok(())
}
fn parse_instructions(proxy: Arc<SmartContractProxy>, block: &SolanaBlock, tran: &TransactionWithStatusMeta, tx_ind: usize) {
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
            // if let Some(account_infos) = SOLANA_CLIENT
            //     .get_multiple_accounts_with_config(
            //         accounts.as_slice(),
            //         RpcAccountInfoConfig {
            //             encoding: Some(UiAccountEncoding::JsonParsed),
            //             commitment: None,
            //             data_slice: None,
            //         },
            //     )
            //     .map(|res| {
            //         res.value
            //             .into_iter()
            //             .filter_map(|elm| elm)
            //             .collect::<Vec<Account>>()
            //     })
            //     .ok()
            // {
            //println!("account_infos {:?}", &account_infos);
            let handler = Handler {};
            // Fixme: Get account_infos from chain take a lot of time. For now, use empty vector.
            if let Some(trans_value) = proxy.unpack_instruction(inst.data.as_slice()) {
                handler.process(block, tran, program_key, &accounts, trans_value);
            }
            // }
        }
    }
}
