use std::error::Error;
use mpl_metaplex::instruction::MetaplexInstruction;
use borsh::BorshDeserialize;
use transport::interface::InstructionParser as InstructionParserTrait;
use transport::TransportValue;

#[derive(Debug, Clone, PartialEq)]
pub struct InstructionParser;

impl InstructionParserTrait for InstructionParser {
    fn unpack_instruction(&self, input: &[u8]) -> Result<TransportValue, anyhow::Error> {
        println!("Unpack instruction");
        let instruction = MetaplexInstruction::try_from_slice(input)?;
        match instruction {
            MetaplexInstruction::DeprecatedInitAuctionManagerV1(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::DeprecatedValidateSafetyDepositBoxV1 => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::RedeemBid => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::RedeemFullRightsTransferBid => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::DeprecatedRedeemParticipationBid => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::StartAuction => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::ClaimBid => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::EmptyPaymentAccount(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::SetStore(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::SetWhitelistedCreator(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::DeprecatedValidateParticipation => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::DeprecatedPopulateParticipationPrintingAccount => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::RedeemUnusedWinningConfigItemsAsAuctioneer(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::DecommissionAuctionManager => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::RedeemPrintingV2Bid(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::WithdrawMasterEdition => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::DeprecatedRedeemParticipationBidV2 => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::InitAuctionManagerV2(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::ValidateSafetyDepositBoxV2(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::RedeemParticipationBidV3(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::EndAuction(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::SetStoreIndex(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::SetAuctionCache => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
            MetaplexInstruction::SetStoreV2(_) => {
                let mut transport_value = TransportValue::default();
                Ok(transport_value)
            }
        }
    }
}