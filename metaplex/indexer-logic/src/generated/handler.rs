use crate::STORE;
use massbit_solana_sdk::entity::{Attribute, Entity, Value};
use massbit_solana_sdk::{
    transport::{TransportValue, Value as TransValue},
    types::SolanaBlock,
};
use serde_json;
use solana_program::pubkey::Pubkey;
use solana_transaction_status::TransactionWithStatusMeta;
use std::collections::HashMap;
use uuid::Uuid;

pub trait TransportValueExt {
    fn save(&self);
}
impl TransportValueExt for TransportValue {
    fn save(&self) {
        unsafe {
            STORE
                .as_mut()
                .unwrap()
                .save_values(&self.name, &self.values);
        }
    }
}

pub struct Handler {}
impl Handler {
    pub fn process(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        mut input: TransportValue,
    ) {
        //println!("Process block {} with input {:?}", block.block_number, input);
        match input.name.as_str() {
            "DeprecatedInitAuctionManagerV1" => {
                self.process_deprecated_init_auction_manager_v1(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedValidateSafetyDepositBoxV1" => {
                self.process_deprecated_validate_safety_deposit_box_v1(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "RedeemBid" => {
                self.process_redeem_bid(block, transaction, program_id, accounts, &mut input);
            }
            "RedeemFullRightsTransferBid" => {
                self.process_redeem_full_rights_transfer_bid(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedRedeemParticipationBid" => {
                self.process_deprecated_redeem_participation_bid(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "StartAuction" => {
                self.process_start_auction(block, transaction, program_id, accounts, &mut input);
            }
            "ClaimBid" => {
                self.process_claim_bid(block, transaction, program_id, accounts, &mut input);
            }
            "EmptyPaymentAccount" => {
                self.process_empty_payment_account(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "SetStore" => {
                self.process_set_store(block, transaction, program_id, accounts, &mut input);
            }
            "SetWhitelistedCreator" => {
                self.process_set_whitelisted_creator(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedValidateParticipation" => {
                self.process_deprecated_validate_participation(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedPopulateParticipationPrintingAccount" => {
                self.process_deprecated_populate_participation_printing_account(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "RedeemUnusedWinningConfigItemsAsAuctioneer" => {
                self.process_redeem_unused_winning_config_items_as_auctioneer(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DecommissionAuctionManager" => {
                self.process_decommission_auction_manager(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "RedeemPrintingV2Bid" => {
                self.process_redeem_printing_v2_bid(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "WithdrawMasterEdition" => {
                self.process_withdraw_master_edition(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedRedeemParticipationBidV2" => {
                self.process_deprecated_redeem_participation_bid_v2(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "InitAuctionManagerV2" => {
                self.process_init_auction_manager_v2(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "ValidateSafetyDepositBoxV2" => {
                self.process_validate_safety_deposit_box_v2(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "RedeemParticipationBidV3" => {
                self.process_redeem_participation_bid_v3(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "EndAuction" => {
                self.process_end_auction(block, transaction, program_id, accounts, &mut input);
            }
            "SetStoreIndex" => {
                self.process_set_store_index(block, transaction, program_id, accounts, &mut input);
            }
            "SetAuctionCache" => {
                self.process_set_auction_cache(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "SetStoreV2" => {
                self.process_set_store_v2(block, transaction, program_id, accounts, &mut input);
            }
            _ => {}
        }
    }
    fn process_deprecated_init_auction_manager_v1(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_validate_safety_deposit_box_v1(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_redeem_bid(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_redeem_full_rights_transfer_bid(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_redeem_participation_bid(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_start_auction(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_claim_bid(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_empty_payment_account(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_set_store(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_set_whitelisted_creator(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_validate_participation(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_populate_participation_printing_account(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_redeem_unused_winning_config_items_as_auctioneer(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_decommission_auction_manager(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_redeem_printing_v2_bid(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_withdraw_master_edition(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_redeem_participation_bid_v2(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_init_auction_manager_v2(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_validate_safety_deposit_box_v2(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_redeem_participation_bid_v3(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_end_auction(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_set_store_index(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_set_auction_cache(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_set_store_v2(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &mut TransportValue,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_initialize for handle incoming block {} with argument {:?}",
            block.block_number, &input.name
        );
        input.set_value("block_timestamp", TransValue::from(block.timestamp));
        input.set_value(
            "tx_hash",
            TransValue::from(
                transaction
                    .transaction
                    .signatures
                    .iter()
                    .map(|sig| sig.to_string())
                    .collect::<Vec<String>>()
                    .join(",'"),
            ),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
}
