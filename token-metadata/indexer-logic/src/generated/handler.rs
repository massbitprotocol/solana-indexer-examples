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
            "CreateMetadataAccount" => {
                self.process_create_metadata_account(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "UpdateMetadataAccount" => {
                self.process_update_metadata_account(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedCreateMasterEdition" => {
                self.process_deprecated_create_master_edition(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedMintNewEditionFromMasterEditionViaPrintingToken" => {
                self.process_deprecated_mint_new_edition_from_master_edition_via_printing_token(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "UpdatePrimarySaleHappenedViaToken" => {
                self.process_update_primary_sale_happened_via_token(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedSetReservationList" => {
                self.process_deprecated_set_reservation_list(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedCreateReservationList" => {
                self.process_deprecated_create_reservation_list(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "SignMetadata" => {
                self.process_sign_metadata(block, transaction, program_id, accounts, &mut input);
            }
            "DeprecatedMintPrintingTokensViaToken" => {
                self.process_deprecated_mint_printing_tokens_via_token(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "DeprecatedMintPrintingTokens" => {
                self.process_deprecated_mint_printing_tokens(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "CreateMasterEdition" => {
                self.process_create_master_edition(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "MintNewEditionFromMasterEditionViaToken" => {
                self.process_mint_new_edition_from_master_edition_via_token(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "ConvertMasterEditionV1ToV2" => {
                self.process_convert_master_edition_v1_to_v2(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "MintNewEditionFromMasterEditionViaVaultProxy" => {
                self.process_mint_new_edition_from_master_edition_via_vault_proxy(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "PuffMetadata" => {
                self.process_puff_metadata(block, transaction, program_id, accounts, &mut input);
            }
            "UpdateMetadataAccountV2" => {
                self.process_update_metadata_account_v2(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "CreateMetadataAccountV2" => {
                self.process_create_metadata_account_v2(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "CreateMasterEditionV3" => {
                self.process_create_master_edition_v3(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "VerifyCollection" => {
                self.process_verify_collection(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "Utilize" => {
                self.process_utilize(block, transaction, program_id, accounts, &mut input);
            }
            "ApproveUseAuthority" => {
                self.process_approve_use_authority(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "RevokeUseAuthority" => {
                self.process_revoke_use_authority(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "UnverifyCollection" => {
                self.process_unverify_collection(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "ApproveCollectionAuthority" => {
                self.process_approve_collection_authority(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            "RevokeCollectionAuthority" => {
                self.process_revoke_collection_authority(
                    block,
                    transaction,
                    program_id,
                    accounts,
                    &mut input,
                );
            }
            _ => {}
        }
    }
    fn process_create_metadata_account(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_update_metadata_account(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_create_master_edition(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_mint_new_edition_from_master_edition_via_printing_token(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_update_primary_sale_happened_via_token(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_set_reservation_list(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_create_reservation_list(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_sign_metadata(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_mint_printing_tokens_via_token(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_deprecated_mint_printing_tokens(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_create_master_edition(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_mint_new_edition_from_master_edition_via_token(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_convert_master_edition_v1_to_v2(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_mint_new_edition_from_master_edition_via_vault_proxy(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_puff_metadata(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_update_metadata_account_v2(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_create_metadata_account_v2(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_create_master_edition_v3(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_verify_collection(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_utilize(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_approve_use_authority(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_revoke_use_authority(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_unverify_collection(
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
        input.set_value(
            "account_name",
            TransValue::from(accounts.get(0).map(|acc| acc.to_string())),
        );
        input.save();
        println!("Write to db {:?}", input);
        Ok(())
    }
    fn process_approve_collection_authority(
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
    fn process_revoke_collection_authority(
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
