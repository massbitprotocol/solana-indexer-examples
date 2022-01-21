use borsh::BorshDeserialize;
use mpl_metaplex::instruction::MetaplexInstruction;
use std::error::Error;
use transport::interface::InstructionParser as InstructionParserTrait;
use transport::{TransportValue, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct InstructionParser;
impl InstructionParserTrait for InstructionParser {
    fn unpack_instruction(&self, input: &[u8]) -> Result<TransportValue, anyhow::Error> {
        let instruction = MetaplexInstruction::try_from_slice(input)?;
        match instruction {
            MetaplexInstruction::DeprecatedInitAuctionManagerV1(input) => {
                self.unpack_deprecated_init_auction_manager_v1(input)
            }
            MetaplexInstruction::DeprecatedValidateSafetyDepositBoxV1 => {
                self.unpack_deprecated_validate_safety_deposit_box_v1()
            }
            MetaplexInstruction::RedeemBid => self.unpack_redeem_bid(),
            MetaplexInstruction::RedeemFullRightsTransferBid => {
                self.unpack_redeem_full_rights_transfer_bid()
            }
            MetaplexInstruction::DeprecatedRedeemParticipationBid => {
                self.unpack_deprecated_redeem_participation_bid()
            }
            MetaplexInstruction::StartAuction => self.unpack_start_auction(),
            MetaplexInstruction::ClaimBid => self.unpack_claim_bid(),
            MetaplexInstruction::EmptyPaymentAccount(input) => {
                self.unpack_empty_payment_account(input)
            }
            MetaplexInstruction::SetStore(input) => self.unpack_set_store(input),
            MetaplexInstruction::SetWhitelistedCreator(input) => {
                self.unpack_set_whitelisted_creator(input)
            }
            MetaplexInstruction::DeprecatedValidateParticipation => {
                self.unpack_deprecated_validate_participation()
            }
            MetaplexInstruction::DeprecatedPopulateParticipationPrintingAccount => {
                self.unpack_deprecated_populate_participation_printing_account()
            }
            MetaplexInstruction::RedeemUnusedWinningConfigItemsAsAuctioneer(input) => {
                self.unpack_redeem_unused_winning_config_items_as_auctioneer(input)
            }
            MetaplexInstruction::DecommissionAuctionManager => {
                self.unpack_decommission_auction_manager()
            }
            MetaplexInstruction::RedeemPrintingV2Bid(input) => {
                self.unpack_redeem_printing_v2_bid(input)
            }
            MetaplexInstruction::WithdrawMasterEdition => self.unpack_withdraw_master_edition(),
            MetaplexInstruction::DeprecatedRedeemParticipationBidV2 => {
                self.unpack_deprecated_redeem_participation_bid_v2()
            }
            MetaplexInstruction::InitAuctionManagerV2(input) => {
                self.unpack_init_auction_manager_v2(input)
            }
            MetaplexInstruction::ValidateSafetyDepositBoxV2(input) => {
                self.unpack_validate_safety_deposit_box_v2(input)
            }
            MetaplexInstruction::RedeemParticipationBidV3(input) => {
                self.unpack_redeem_participation_bid_v3(input)
            }
            MetaplexInstruction::EndAuction(input) => self.unpack_end_auction(input),
            MetaplexInstruction::SetStoreIndex(input) => self.unpack_set_store_index(input),
            MetaplexInstruction::SetAuctionCache => self.unpack_set_auction_cache(),
            MetaplexInstruction::SetStoreV2(input) => self.unpack_set_store_v2(input),
        }
    }
}
impl InstructionParser {
    fn unpack_deprecated_init_auction_manager_v1(
        &self,
        input: mpl_metaplex::deprecated_state::AuctionManagerSettingsV1,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedInitAuctionManagerV1");
        transport_value.set_value(
            "winning_configs",
            Value::from(
                input
                    .winning_configs
                    .iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<String>>(),
            ),
        );
        transport_value.set_value(
            "participation_config",
            Value::from(input.participation_config.map(|item| format!("{:?}", item))),
        );
        Ok(transport_value)
    }
    fn unpack_deprecated_validate_safety_deposit_box_v1(
        &self,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedValidateSafetyDepositBoxV1");
        Ok(transport_value)
    }
    fn unpack_redeem_bid(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("RedeemBid");
        Ok(transport_value)
    }
    fn unpack_redeem_full_rights_transfer_bid(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("RedeemFullRightsTransferBid");
        Ok(transport_value)
    }
    fn unpack_deprecated_redeem_participation_bid(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedRedeemParticipationBid");
        Ok(transport_value)
    }
    fn unpack_start_auction(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("StartAuction");
        Ok(transport_value)
    }
    fn unpack_claim_bid(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("ClaimBid");
        Ok(transport_value)
    }
    fn unpack_empty_payment_account(
        &self,
        input: mpl_metaplex::instruction::EmptyPaymentAccountArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("EmptyPaymentAccount");
        transport_value.set_value(
            "winning_config_index",
            Value::from(input.winning_config_index),
        );
        transport_value.set_value(
            "winning_config_item_index",
            Value::from(input.winning_config_item_index),
        );
        transport_value.set_value("creator_index", Value::from(input.creator_index));
        Ok(transport_value)
    }
    fn unpack_set_store(
        &self,
        input: mpl_metaplex::instruction::SetStoreArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("SetStore");
        transport_value.set_value("public", Value::from(input.public));
        Ok(transport_value)
    }
    fn unpack_set_whitelisted_creator(
        &self,
        input: mpl_metaplex::instruction::SetWhitelistedCreatorArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("SetWhitelistedCreator");
        transport_value.set_value("activated", Value::from(input.activated));
        Ok(transport_value)
    }
    fn unpack_deprecated_validate_participation(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedValidateParticipation");
        Ok(transport_value)
    }
    fn unpack_deprecated_populate_participation_printing_account(
        &self,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value =
            TransportValue::new("DeprecatedPopulateParticipationPrintingAccount");
        Ok(transport_value)
    }
    fn unpack_redeem_unused_winning_config_items_as_auctioneer(
        &self,
        input: mpl_metaplex::instruction::RedeemUnusedWinningConfigItemsAsAuctioneerArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("RedeemUnusedWinningConfigItemsAsAuctioneer");
        transport_value.set_value(
            "winning_config_item_index",
            Value::from(input.winning_config_item_index),
        );
        transport_value.set_value(
            "proxy_call",
            match input.proxy_call {
                mpl_metaplex::instruction::ProxyCallAddress::RedeemBid => Value::from("RedeemBid"),
                mpl_metaplex::instruction::ProxyCallAddress::RedeemFullRightsTransferBid => {
                    Value::from("RedeemFullRightsTransferBid")
                }
            },
        );
        Ok(transport_value)
    }
    fn unpack_decommission_auction_manager(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DecommissionAuctionManager");
        Ok(transport_value)
    }
    fn unpack_redeem_printing_v2_bid(
        &self,
        input: mpl_metaplex::instruction::RedeemPrintingV2BidArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("RedeemPrintingV2Bid");
        transport_value.set_value("edition_offset", Value::from(input.edition_offset));
        transport_value.set_value("win_index", Value::from(input.win_index));
        Ok(transport_value)
    }
    fn unpack_withdraw_master_edition(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("WithdrawMasterEdition");
        Ok(transport_value)
    }
    fn unpack_deprecated_redeem_participation_bid_v2(
        &self,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("DeprecatedRedeemParticipationBidV2");
        Ok(transport_value)
    }
    fn unpack_init_auction_manager_v2(
        &self,
        input: mpl_metaplex::instruction::InitAuctionManagerV2Args,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("InitAuctionManagerV2");
        transport_value.set_value(
            "amount_type",
            match input.amount_type {
                mpl_metaplex::state::TupleNumericType::Padding0 => Value::from("Padding0"),
                mpl_metaplex::state::TupleNumericType::U8 => Value::from("U8"),
                mpl_metaplex::state::TupleNumericType::U16 => Value::from("U16"),
                mpl_metaplex::state::TupleNumericType::Padding1 => Value::from("Padding1"),
                mpl_metaplex::state::TupleNumericType::U32 => Value::from("U32"),
                mpl_metaplex::state::TupleNumericType::Padding2 => Value::from("Padding2"),
                mpl_metaplex::state::TupleNumericType::Padding3 => Value::from("Padding3"),
                mpl_metaplex::state::TupleNumericType::Padding4 => Value::from("Padding4"),
                mpl_metaplex::state::TupleNumericType::U64 => Value::from("U64"),
            },
        );
        transport_value.set_value(
            "length_type",
            match input.length_type {
                mpl_metaplex::state::TupleNumericType::Padding0 => Value::from("Padding0"),
                mpl_metaplex::state::TupleNumericType::U8 => Value::from("U8"),
                mpl_metaplex::state::TupleNumericType::U16 => Value::from("U16"),
                mpl_metaplex::state::TupleNumericType::Padding1 => Value::from("Padding1"),
                mpl_metaplex::state::TupleNumericType::U32 => Value::from("U32"),
                mpl_metaplex::state::TupleNumericType::Padding2 => Value::from("Padding2"),
                mpl_metaplex::state::TupleNumericType::Padding3 => Value::from("Padding3"),
                mpl_metaplex::state::TupleNumericType::Padding4 => Value::from("Padding4"),
                mpl_metaplex::state::TupleNumericType::U64 => Value::from("U64"),
            },
        );
        transport_value.set_value("max_ranges", Value::from(input.max_ranges));
        Ok(transport_value)
    }
    fn unpack_validate_safety_deposit_box_v2(
        &self,
        input: mpl_metaplex::state::SafetyDepositConfig,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("ValidateSafetyDepositBoxV2");
        transport_value.set_value(
            "key",
            match input.key {
                mpl_metaplex::state::Key::Uninitialized => Value::from("Uninitialized"),
                mpl_metaplex::state::Key::OriginalAuthorityLookupV1 => {
                    Value::from("OriginalAuthorityLookupV1")
                }
                mpl_metaplex::state::Key::BidRedemptionTicketV1 => {
                    Value::from("BidRedemptionTicketV1")
                }
                mpl_metaplex::state::Key::StoreV1 => Value::from("StoreV1"),
                mpl_metaplex::state::Key::WhitelistedCreatorV1 => {
                    Value::from("WhitelistedCreatorV1")
                }
                mpl_metaplex::state::Key::PayoutTicketV1 => Value::from("PayoutTicketV1"),
                mpl_metaplex::state::Key::SafetyDepositValidationTicketV1 => {
                    Value::from("SafetyDepositValidationTicketV1")
                }
                mpl_metaplex::state::Key::AuctionManagerV1 => Value::from("AuctionManagerV1"),
                mpl_metaplex::state::Key::PrizeTrackingTicketV1 => {
                    Value::from("PrizeTrackingTicketV1")
                }
                mpl_metaplex::state::Key::SafetyDepositConfigV1 => {
                    Value::from("SafetyDepositConfigV1")
                }
                mpl_metaplex::state::Key::AuctionManagerV2 => Value::from("AuctionManagerV2"),
                mpl_metaplex::state::Key::BidRedemptionTicketV2 => {
                    Value::from("BidRedemptionTicketV2")
                }
                mpl_metaplex::state::Key::AuctionWinnerTokenTypeTrackerV1 => {
                    Value::from("AuctionWinnerTokenTypeTrackerV1")
                }
                mpl_metaplex::state::Key::StoreIndexerV1 => Value::from("StoreIndexerV1"),
                mpl_metaplex::state::Key::AuctionCacheV1 => Value::from("AuctionCacheV1"),
                mpl_metaplex::state::Key::StoreConfigV1 => Value::from("StoreConfigV1"),
            },
        );
        transport_value.set_value(
            "auction_manager",
            Value::String(format!("{:?}", input.auction_manager)),
        );
        transport_value.set_value("order", Value::from(input.order));
        transport_value.set_value(
            "winning_config_type",
            match input.winning_config_type {
                mpl_metaplex::state::WinningConfigType::TokenOnlyTransfer => {
                    Value::from("TokenOnlyTransfer")
                }
                mpl_metaplex::state::WinningConfigType::FullRightsTransfer => {
                    Value::from("FullRightsTransfer")
                }
                mpl_metaplex::state::WinningConfigType::PrintingV1 => Value::from("PrintingV1"),
                mpl_metaplex::state::WinningConfigType::PrintingV2 => Value::from("PrintingV2"),
                mpl_metaplex::state::WinningConfigType::Participation => {
                    Value::from("Participation")
                }
            },
        );
        transport_value.set_value(
            "amount_type",
            match input.amount_type {
                mpl_metaplex::state::TupleNumericType::Padding0 => Value::from("Padding0"),
                mpl_metaplex::state::TupleNumericType::U8 => Value::from("U8"),
                mpl_metaplex::state::TupleNumericType::U16 => Value::from("U16"),
                mpl_metaplex::state::TupleNumericType::Padding1 => Value::from("Padding1"),
                mpl_metaplex::state::TupleNumericType::U32 => Value::from("U32"),
                mpl_metaplex::state::TupleNumericType::Padding2 => Value::from("Padding2"),
                mpl_metaplex::state::TupleNumericType::Padding3 => Value::from("Padding3"),
                mpl_metaplex::state::TupleNumericType::Padding4 => Value::from("Padding4"),
                mpl_metaplex::state::TupleNumericType::U64 => Value::from("U64"),
            },
        );
        transport_value.set_value(
            "length_type",
            match input.length_type {
                mpl_metaplex::state::TupleNumericType::Padding0 => Value::from("Padding0"),
                mpl_metaplex::state::TupleNumericType::U8 => Value::from("U8"),
                mpl_metaplex::state::TupleNumericType::U16 => Value::from("U16"),
                mpl_metaplex::state::TupleNumericType::Padding1 => Value::from("Padding1"),
                mpl_metaplex::state::TupleNumericType::U32 => Value::from("U32"),
                mpl_metaplex::state::TupleNumericType::Padding2 => Value::from("Padding2"),
                mpl_metaplex::state::TupleNumericType::Padding3 => Value::from("Padding3"),
                mpl_metaplex::state::TupleNumericType::Padding4 => Value::from("Padding4"),
                mpl_metaplex::state::TupleNumericType::U64 => Value::from("U64"),
            },
        );
        transport_value.set_value(
            "amount_ranges",
            Value::from(
                input
                    .amount_ranges
                    .iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<String>>(),
            ),
        );
        transport_value.set_value(
            "participation_config",
            Value::from(input.participation_config.map(|item| format!("{:?}", item))),
        );
        transport_value.set_value(
            "participation_state",
            Value::from(input.participation_state.map(|item| format!("{:?}", item))),
        );
        Ok(transport_value)
    }
    fn unpack_redeem_participation_bid_v3(
        &self,
        input: mpl_metaplex::instruction::RedeemParticipationBidV3Args,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("RedeemParticipationBidV3");
        transport_value.set_value("win_index", Value::from(input.win_index));
        Ok(transport_value)
    }
    fn unpack_end_auction(
        &self,
        input: mpl_metaplex::instruction::EndAuctionArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("EndAuction");
        transport_value.set_value("reveal", Value::Null);
        Ok(transport_value)
    }
    fn unpack_set_store_index(
        &self,
        input: mpl_metaplex::instruction::SetStoreIndexArgs,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("SetStoreIndex");
        transport_value.set_value("page", Value::from(input.page));
        transport_value.set_value("offset", Value::from(input.offset));
        Ok(transport_value)
    }
    fn unpack_set_auction_cache(&self) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("SetAuctionCache");
        Ok(transport_value)
    }
    fn unpack_set_store_v2(
        &self,
        input: mpl_metaplex::instruction::SetStoreV2Args,
    ) -> Result<TransportValue, anyhow::Error> {
        let mut transport_value = TransportValue::new("SetStoreV2");
        transport_value.set_value("public", Value::from(input.public));
        transport_value.set_value("settings_uri", Value::Null);
        Ok(transport_value)
    }
}
