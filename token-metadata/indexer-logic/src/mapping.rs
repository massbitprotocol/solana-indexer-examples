use std::collections::HashMap;
use crate::generated::handler::Handler;
use crate::ADDRESS;
use crate::SOLANA_CLIENT;
use massbit_solana_sdk::smart_contract::{InstructionParser, SmartContractProxy};
use massbit_solana_sdk::transport::interface::InterfaceRegistrar;
use massbit_solana_sdk::types::SolanaBlock;
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
use std::sync::Arc;
use uuid::Uuid;

pub fn handle_block(
    interface: &mut dyn InstructionParser,
    block: &SolanaBlock,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Start handle_block, block.block_number: {}",
        block.block_number
    );
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
fn parse_instructions(
    interface: &mut dyn InstructionParser,
    block: &SolanaBlock,
    tran: &TransactionWithStatusMeta,
    tx_ind: usize,
) {
    let map_inner_instructions  = tran.meta.as_ref().and_then(|trans_meta|
        trans_meta.inner_instructions.as_ref().map(|insts| insts.iter().map(|inner_inst|{
            (inner_inst.index, &inner_inst.instructions)
        }).collect::<HashMap<u8, &Vec<CompiledInstruction>>>())).unwrap_or_default();
    let handler = Handler {};
    for (ind, inst) in tran.transaction.message.instructions.iter().enumerate() {
        process_instruction(interface, &handler, block, tran, inst);
        let inner_key = ind as u8;
        if let Some(inner_instructions) = map_inner_instructions.get(&inner_key) {
            inner_instructions.iter().for_each(|inner_instruction| {
                process_instruction(interface, &handler, block, tran, inner_instruction);
            })
        }
    }
}

fn process_instruction(interface: &mut dyn InstructionParser, handler: &Handler, block: &SolanaBlock, tran: &TransactionWithStatusMeta, instruction: &CompiledInstruction) {
    let program_key = instruction.program_id(tran.transaction.message.account_keys.as_slice());
    if program_key.to_string().as_str() == ADDRESS {
        let mut accounts = Vec::default();
        let mut work = |unique_ind: usize, acc_ind: usize| {
            if let Some(key) = tran.transaction.message.account_keys.get(acc_ind) {
                accounts.push(key.clone());
            };
            Ok(())
        };
        instruction.visit_each_account(&mut work);

        // Fixme: Get account_infos from chain take a lot of time. For now, use empty vector.

        println!("Start unpack_instruction, inst {:?}", &instruction);
        match interface.unpack_instruction(instruction.data.as_slice()) {
            Ok(trans_value) => {
                println!("unpack_instruction Ok, trans_value: {:?}", &trans_value);
                &handler.process(block, tran, program_key, &accounts, trans_value);
            }
            Err(e) => {
                println!("Error unpack_instruction: {:?}", e);
            }
        }
    }
}