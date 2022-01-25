use borsh::BorshDeserialize;
use mpl_token_metadata::instruction::MetadataInstruction;
use std::error::Error;
use transport::interface::InstructionParser as InstructionParserTrait;
use transport::{TransportValue, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct InstructionParser;
impl InstructionParserTrait for InstructionParser {
    fn unpack_instruction(&self, input: &[u8]) -> Result<TransportValue, anyhow::Error> {
        let instruction = MetadataInstruction::try_from_slice(input)?;
        match instruction {
            MetadataInstruction::CreateMetadataAccount(input) => {
                self.unpack_create_metadata_account(input)
            }
            MetadataInstruction::UpdateMetadataAccount(input) => {
                self.unpack_update_metadata_account(input)
            }
            MetadataInstruction::DeprecatedCreateMasterEdition(input) => {
                self.unpack_deprecated_create_master_edition(input)
            }
            MetadataInstruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken => {
                self.unpack_deprecated_mint_new_edition_from_master_edition_via_printing_token()
            }
            MetadataInstruction::UpdatePrimarySaleHappenedViaToken => {
                self.unpack_update_primary_sale_happened_via_token()
            }
            MetadataInstruction::DeprecatedSetReservationList(input) => {
                self.unpack_deprecated_set_reservation_list(input)
            }
            MetadataInstruction::DeprecatedCreateReservationList => {
                self.unpack_deprecated_create_reservation_list()
            }
            MetadataInstruction::SignMetadata => self.unpack_sign_metadata(),
            MetadataInstruction::DeprecatedMintPrintingTokensViaToken(input) => {
                self.unpack_deprecated_mint_printing_tokens_via_token(input)
            }
            MetadataInstruction::DeprecatedMintPrintingTokens(input) => {
                self.unpack_deprecated_mint_printing_tokens(input)
            }
            MetadataInstruction::CreateMasterEdition(input) => {
                self.unpack_create_master_edition(input)
            }
            MetadataInstruction::MintNewEditionFromMasterEditionViaToken(input) => {
                self.unpack_mint_new_edition_from_master_edition_via_token(input)
            }
            MetadataInstruction::ConvertMasterEditionV1ToV2 => {
                self.unpack_convert_master_edition_v1_to_v2()
            }
            MetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy(input) => {
                self.unpack_mint_new_edition_from_master_edition_via_vault_proxy(input)
            }
            MetadataInstruction::PuffMetadata => self.unpack_puff_metadata(),
            MetadataInstruction::UpdateMetadataAccountV2(input) => {
                self.unpack_update_metadata_account_v2(input)
            }
            MetadataInstruction::CreateMetadataAccountV2(input) => {
                self.unpack_create_metadata_account_v2(input)
            }
            MetadataInstruction::CreateMasterEditionV3(input) => {
                self.unpack_create_master_edition_v3(input)
            }
            MetadataInstruction::VerifyCollection => self.unpack_verify_collection(),
            MetadataInstruction::Utilize(input) => self.unpack_utilize(input),
            MetadataInstruction::ApproveUseAuthority(input) => {
                self.unpack_approve_use_authority(input)
            }
            MetadataInstruction::RevokeUseAuthority => self.unpack_revoke_use_authority(),
            MetadataInstruction::UnverifyCollection => self.unpack_unverify_collection(),
            MetadataInstruction::ApproveCollectionAuthority => {
                self.unpack_approve_collection_authority()
            }
            MetadataInstruction::RevokeCollectionAuthority => {
                self.unpack_revoke_collection_authority()
            }
        }
    }
}
impl InstructionParser {
    fn unpack_create_metadata_account(
        &self,
        input: mpl_token_metadata::instruction::CreateMetadataAccountArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("CreateMetadataAccount");
        transport_value.set_value("data", Value::Null);
        transport_value.set_value("is_mutable", Value::from(input.is_mutable));
        Ok(transport_value)
    }
    fn unpack_update_metadata_account(
        &self,
        input: mpl_token_metadata::instruction::UpdateMetadataAccountArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("UpdateMetadataAccount");
        transport_value.set_value(
            "data",
            Value::from(input.data.map(|item| format!("{:?}", item))),
        );
        transport_value.set_value("update_authority", Value::Null);
        transport_value.set_value(
            "primary_sale_happened",
            Value::from(input.primary_sale_happened),
        );
        Ok(transport_value)
    }
    fn unpack_deprecated_create_master_edition(
        &self,
        input: mpl_token_metadata::instruction::CreateMasterEditionArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedCreateMasterEdition");
        transport_value.set_value("max_supply", Value::from(input.max_supply));
        Ok(transport_value)
    }
    fn unpack_deprecated_mint_new_edition_from_master_edition_via_printing_token(
        &self,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value =
            TransportValue::new("DeprecatedMintNewEditionFromMasterEditionViaPrintingToken");
        Ok(transport_value)
    }
    fn unpack_update_primary_sale_happened_via_token(
        &self,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("UpdatePrimarySaleHappenedViaToken");
        Ok(transport_value)
    }
    fn unpack_deprecated_set_reservation_list(
        &self,
        input: mpl_token_metadata::deprecated_instruction::SetReservationListArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedSetReservationList");
        transport_value.set_value(
            "reservations",
            Value::from(
                input
                    .reservations
                    .iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<String>>(),
            ),
        );
        transport_value.set_value(
            "total_reservation_spots",
            Value::from(input.total_reservation_spots),
        );
        transport_value.set_value("offset", Value::from(input.offset));
        transport_value.set_value("total_spot_offset", Value::from(input.total_spot_offset));
        Ok(transport_value)
    }
    fn unpack_deprecated_create_reservation_list(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedCreateReservationList");
        Ok(transport_value)
    }
    fn unpack_sign_metadata(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("SignMetadata");
        Ok(transport_value)
    }
    fn unpack_deprecated_mint_printing_tokens_via_token(
        &self,
        input: mpl_token_metadata::deprecated_instruction::MintPrintingTokensViaTokenArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedMintPrintingTokensViaToken");
        transport_value.set_value("supply", Value::from(input.supply));
        Ok(transport_value)
    }
    fn unpack_deprecated_mint_printing_tokens(
        &self,
        input: mpl_token_metadata::deprecated_instruction::MintPrintingTokensViaTokenArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedMintPrintingTokens");
        transport_value.set_value("supply", Value::from(input.supply));
        Ok(transport_value)
    }
    fn unpack_create_master_edition(
        &self,
        input: mpl_token_metadata::instruction::CreateMasterEditionArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("CreateMasterEdition");
        transport_value.set_value("max_supply", Value::from(input.max_supply));
        Ok(transport_value)
    }
    fn unpack_mint_new_edition_from_master_edition_via_token(
        &self,
        input: mpl_token_metadata::instruction::MintNewEditionFromMasterEditionViaTokenArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("MintNewEditionFromMasterEditionViaToken");
        transport_value.set_value("edition", Value::from(input.edition));
        Ok(transport_value)
    }
    fn unpack_convert_master_edition_v1_to_v2(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("ConvertMasterEditionV1ToV2");
        Ok(transport_value)
    }
    fn unpack_mint_new_edition_from_master_edition_via_vault_proxy(
        &self,
        input: mpl_token_metadata::instruction::MintNewEditionFromMasterEditionViaTokenArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value =
            TransportValue::new("MintNewEditionFromMasterEditionViaVaultProxy");
        transport_value.set_value("edition", Value::from(input.edition));
        Ok(transport_value)
    }
    fn unpack_puff_metadata(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("PuffMetadata");
        Ok(transport_value)
    }
    fn unpack_update_metadata_account_v2(
        &self,
        input: mpl_token_metadata::instruction::UpdateMetadataAccountArgsV2,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("UpdateMetadataAccountV2");
        transport_value.set_value(
            "data",
            Value::from(input.data.map(|item| format!("{:?}", item))),
        );
        transport_value.set_value("update_authority", Value::Null);
        transport_value.set_value(
            "primary_sale_happened",
            Value::from(input.primary_sale_happened),
        );
        transport_value.set_value("is_mutable", Value::from(input.is_mutable));
        Ok(transport_value)
    }
    fn unpack_create_metadata_account_v2(
        &self,
        input: mpl_token_metadata::instruction::CreateMetadataAccountArgsV2,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("CreateMetadataAccountV2");
        transport_value.set_value("data", Value::Null);
        transport_value.set_value("is_mutable", Value::from(input.is_mutable));
        Ok(transport_value)
    }
    fn unpack_create_master_edition_v3(
        &self,
        input: mpl_token_metadata::instruction::CreateMasterEditionArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("CreateMasterEditionV3");
        transport_value.set_value("max_supply", Value::from(input.max_supply));
        Ok(transport_value)
    }
    fn unpack_verify_collection(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("VerifyCollection");
        Ok(transport_value)
    }
    fn unpack_utilize(
        &self,
        input: mpl_token_metadata::instruction::UtilizeArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("Utilize");
        transport_value.set_value("number_of_uses", Value::from(input.number_of_uses));
        Ok(transport_value)
    }
    fn unpack_approve_use_authority(
        &self,
        input: mpl_token_metadata::instruction::ApproveUseAuthorityArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("ApproveUseAuthority");
        transport_value.set_value("number_of_uses", Value::from(input.number_of_uses));
        Ok(transport_value)
    }
    fn unpack_revoke_use_authority(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("RevokeUseAuthority");
        Ok(transport_value)
    }
    fn unpack_unverify_collection(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("UnverifyCollection");
        Ok(transport_value)
    }
    fn unpack_approve_collection_authority(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("ApproveCollectionAuthority");
        Ok(transport_value)
    }
    fn unpack_revoke_collection_authority(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("RevokeCollectionAuthority");
        Ok(transport_value)
    }
}
