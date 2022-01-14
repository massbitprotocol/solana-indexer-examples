use crate::generated::instruction::*;
use crate::STORE;
//use crate::models::*;
use serde_json;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use solana_transaction_status::{parse_instruction, ConfirmedBlock, TransactionWithStatusMeta};
use std::collections::HashMap;
use massbit_solana_sdk::entity::{Attribute, Entity, Value};
use massbit_solana_sdk::smart_contract::TransportValue;
use massbit_solana_sdk::types::SolanaBlock;
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
        if let Some(instruction) = RootInstruction::unpack(input) {
            match instruction {
                RootInstruction::Initialize(arg) => {
                    self.process_initialize(block, transaction, program_id, accounts, arg);
                }
                RootInstruction::Swap(arg) => {
                    self.process_swap(block, transaction, program_id, accounts, arg);
                }
                RootInstruction::Deposit(arg) => {
                    self.process_deposit(block, transaction, program_id, accounts, arg);
                }
                RootInstruction::Withdraw(arg) => {
                    self.process_withdraw(block, transaction, program_id, accounts, arg);
                }
                RootInstruction::WithdrawOne(arg) => {
                    self.process_withdraw_one(block, transaction, program_id, accounts, arg);
                }
                RootInstruction::RampA(arg) => {
                    self.process_ramp_a(block, transaction, program_id, accounts, arg);
                }
                RootInstruction::StopRampA => {
                    self.process_stop_ramp_a(block, transaction, program_id, accounts);
                }
                RootInstruction::Pause => {
                    self.process_pause(block, transaction, program_id, accounts);
                }
                RootInstruction::Unpause => {
                    self.process_unpause(block, transaction, program_id, accounts);
                }
                RootInstruction::SetFeeAccount => {
                    self.process_set_fee_account(block, transaction, program_id, accounts);
                }
                RootInstruction::ApplyNewAdmin => {
                    self.process_apply_new_admin(block, transaction, program_id, accounts);
                }
                RootInstruction::CommitNewAdmin => {
                    self.process_commit_new_admin(block, transaction, program_id, accounts);
                }
                RootInstruction::SetNewFees(arg) => {
                    self.process_set_new_fees(block, transaction, program_id, accounts, arg);
                }
            }
        }
    }
    pub fn process_initialize(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: InitializeData,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "stable_swap".to_string(),
            Value::try_from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "authority_base".to_string(),
            Value::try_from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_a_authority".to_string(),
            Value::try_from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_b_authority".to_string(),
            Value::try_from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_a_base_account".to_string(),
            Value::try_from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_b_base_account".to_string(),
            Value::try_from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_mint_account".to_string(),
            Value::try_from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_account".to_string(),
            Value::try_from(
                accounts
                    .get(7)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "program_id".to_string(),
            Value::try_from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "clock_sysvar".to_string(),
            Value::try_from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert("nonce".to_string(), Value::try_from(arg.nonce));
        map.insert("amp_factor".to_string(), Value::try_from(arg.amp_factor));
        map.insert(
            "fees".to_string(),
            Value::try_from(serde_json::to_string(&arg.fees).unwrap_or(Default::default())),
        );
        Entity::from(map).save("Initialize");
        Ok(())
    }
    pub fn process_swap(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: SwapData,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_swap for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "stable_swap".to_string(),
            Value::try_from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "authority_base".to_string(),
            Value::try_from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "authority_source".to_string(),
            Value::try_from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "source_account".to_string(),
            Value::try_from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "base_into".to_string(),
            Value::try_from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "base_from".to_string(),
            Value::try_from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "destination_account".to_string(),
            Value::try_from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "admin_fee_account".to_string(),
            Value::try_from(
                accounts
                    .get(7)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "program_id".to_string(),
            Value::try_from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "clock_sysvar".to_string(),
            Value::try_from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert("amount_in".to_string(), Value::try_from(arg.amount_in));
        map.insert(
            "minimum_amount_out".to_string(),
            Value::try_from(arg.minimum_amount_out),
        );
        Entity::from(map).save("Swap");
        Ok(())
    }
    pub fn process_deposit(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: DepositData,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_deposit for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "stable_swap".to_string(),
            Value::try_from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "base_authority".to_string(),
            Value::try_from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "owner_authority".to_string(),
            Value::try_from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_a".to_string(),
            Value::try_from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_b".to_string(),
            Value::try_from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_a_base".to_string(),
            Value::try_from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_b_base".to_string(),
            Value::try_from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "mint_account".to_string(),
            Value::try_from(
                accounts
                    .get(7)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_account".to_string(),
            Value::try_from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "program_id".to_string(),
            Value::try_from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "clock_sysvar".to_string(),
            Value::try_from(
                accounts
                    .get(10)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_a_amount".to_string(),
            Value::try_from(arg.token_a_amount),
        );
        map.insert(
            "token_b_amount".to_string(),
            Value::try_from(arg.token_b_amount),
        );
        map.insert(
            "min_mint_amount".to_string(),
            Value::try_from(arg.min_mint_amount),
        );
        Entity::from(map).save("Deposit");
        Ok(())
    }
    pub fn process_withdraw(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: WithdrawData,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_withdraw for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "stable_swap".to_string(),
            Value::try_from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "base_authority".to_string(),
            Value::try_from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "owner_authority".to_string(),
            Value::try_from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_mint".to_string(),
            Value::try_from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_account".to_string(),
            Value::try_from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_a_swap".to_string(),
            Value::try_from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_b_swap".to_string(),
            Value::try_from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_a".to_string(),
            Value::try_from(
                accounts
                    .get(7)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_b".to_string(),
            Value::try_from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "admin_fee_a_account".to_string(),
            Value::try_from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "admin_fee_b_account".to_string(),
            Value::try_from(
                accounts
                    .get(10)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_token_amount".to_string(),
            Value::try_from(arg.pool_token_amount),
        );
        map.insert(
            "minimum_token_a_amount".to_string(),
            Value::try_from(arg.minimum_token_a_amount),
        );
        map.insert(
            "minimum_token_b_amount".to_string(),
            Value::try_from(arg.minimum_token_b_amount),
        );
        Entity::from(map).save("Withdraw");
        Ok(())
    }
    pub fn process_withdraw_one(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: WithdrawOneData,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_withdraw_one for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "stable_swap".to_string(),
            Value::try_from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "swap_authority".to_string(),
            Value::try_from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_authority".to_string(),
            Value::try_from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_mint".to_string(),
            Value::try_from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_account".to_string(),
            Value::try_from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "swap_base_account".to_string(),
            Value::try_from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "swap_quote_account".to_string(),
            Value::try_from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "admin_fee_account".to_string(),
            Value::try_from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "program_id".to_string(),
            Value::try_from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "clock_sysvar".to_string(),
            Value::try_from(
                accounts
                    .get(10)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pool_token_amount".to_string(),
            Value::try_from(arg.pool_token_amount),
        );
        map.insert(
            "minimum_token_amount".to_string(),
            Value::try_from(arg.minimum_token_amount),
        );
        Entity::from(map).save("WithdrawOne");
        Ok(())
    }
    pub fn process_ramp_a(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: RampAData,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_ramp_a for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert("target_amp".to_string(), Value::try_from(arg.target_amp));
        map.insert(
            "stop_ramp_ts".to_string(),
            Value::try_from(arg.stop_ramp_ts),
        );
        Entity::from(map).save("RampA");
        Ok(())
    }
    pub fn process_stop_ramp_a(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_stop_ramp_a for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );

        Entity::from(map).save("StopRampA");
        Ok(())
    }
    pub fn process_pause(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_pause for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );

        Entity::from(map).save("Pause");
        Ok(())
    }
    pub fn process_unpause(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_unpause for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );

        Entity::from(map).save("Unpause");
        Ok(())
    }
    pub fn process_set_fee_account(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_set_fee_account for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );

        Entity::from(map).save("SetFeeAccount");
        Ok(())
    }
    pub fn process_apply_new_admin(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_apply_new_admin for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );

        Entity::from(map).save("ApplyNewAdmin");
        Ok(())
    }
    pub fn process_commit_new_admin(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_commit_new_admin for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );

        Entity::from(map).save("CommitNewAdmin");
        Ok(())
    }
    pub fn process_set_new_fees(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: Fees,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_set_new_fees for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::try_from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "admin_trade_fee_numerator".to_string(),
            Value::try_from(arg.admin_trade_fee_numerator),
        );
        map.insert(
            "admin_trade_fee_denominator".to_string(),
            Value::try_from(arg.admin_trade_fee_denominator),
        );
        map.insert(
            "admin_withdraw_fee_numerator".to_string(),
            Value::try_from(arg.admin_withdraw_fee_numerator),
        );
        map.insert(
            "admin_withdraw_fee_denominator".to_string(),
            Value::try_from(arg.admin_withdraw_fee_denominator),
        );
        map.insert(
            "trade_fee_numerator".to_string(),
            Value::try_from(arg.trade_fee_numerator),
        );
        map.insert(
            "trade_fee_denominator".to_string(),
            Value::try_from(arg.trade_fee_denominator),
        );
        map.insert(
            "withdraw_fee_numerator".to_string(),
            Value::try_from(arg.withdraw_fee_numerator),
        );
        map.insert(
            "withdraw_fee_denominator".to_string(),
            Value::try_from(arg.withdraw_fee_denominator),
        );
        Entity::from(map).save("SetNewFees");
        Ok(())
    }
}
