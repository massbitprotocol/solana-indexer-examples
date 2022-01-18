use arrayref::{array_ref, array_refs};
//use bytemuck::cast;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use solana_program::{pubkey::Pubkey, sysvar::rent};
use std::num::*;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DepositData {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub min_mint_amount: u64,
}
impl DepositData {
    pub fn unpack(input: &[u8; 24]) -> Option<Self> {
        let (token_a_amount, token_b_amount, min_mint_amount) = array_refs![input, 8, 8, 8];

        Some(DepositData {
            token_a_amount: u64::from_le_bytes(*token_a_amount),
            token_b_amount: u64::from_le_bytes(*token_b_amount),
            min_mint_amount: u64::from_le_bytes(*min_mint_amount),
        })
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Fees {
    pub admin_trade_fee_numerator: u64,
    pub admin_trade_fee_denominator: u64,
    pub admin_withdraw_fee_numerator: u64,
    pub admin_withdraw_fee_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub withdraw_fee_numerator: u64,
    pub withdraw_fee_denominator: u64,
}
impl Fees {
    pub fn unpack(input: &[u8; 64]) -> Option<Self> {
        let (
            admin_trade_fee_numerator,
            admin_trade_fee_denominator,
            admin_withdraw_fee_numerator,
            admin_withdraw_fee_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            withdraw_fee_numerator,
            withdraw_fee_denominator,
        ) = array_refs![input, 8, 8, 8, 8, 8, 8, 8, 8];

        Some(Fees {
            admin_trade_fee_numerator: u64::from_le_bytes(*admin_trade_fee_numerator),
            admin_trade_fee_denominator: u64::from_le_bytes(*admin_trade_fee_denominator),
            admin_withdraw_fee_numerator: u64::from_le_bytes(*admin_withdraw_fee_numerator),
            admin_withdraw_fee_denominator: u64::from_le_bytes(*admin_withdraw_fee_denominator),
            trade_fee_numerator: u64::from_le_bytes(*trade_fee_numerator),
            trade_fee_denominator: u64::from_le_bytes(*trade_fee_denominator),
            withdraw_fee_numerator: u64::from_le_bytes(*withdraw_fee_numerator),
            withdraw_fee_denominator: u64::from_le_bytes(*withdraw_fee_denominator),
        })
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeData {
    pub nonce: u8,
    pub amp_factor: u64,
    pub fees: Fees,
}
impl InitializeData {
    pub fn unpack(input: &[u8; 73]) -> Option<Self> {
        let (nonce, amp_factor, fees) = array_refs![input, 1, 8, 64];
        let fees = Fees::unpack(fees);
        if fees.is_some() {
            Some(InitializeData {
                nonce: u8::from_le_bytes(*nonce),
                amp_factor: u64::from_le_bytes(*amp_factor),
                fees: fees.unwrap(),
            })
        } else {
            None
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RampAData {
    pub target_amp: u64,
    pub stop_ramp_ts: u64,
}
impl RampAData {
    pub fn unpack(input: &[u8; 16]) -> Option<Self> {
        let (target_amp, stop_ramp_ts) = array_refs![input, 8, 8];

        Some(RampAData {
            target_amp: u64::from_le_bytes(*target_amp),
            stop_ramp_ts: u64::from_le_bytes(*stop_ramp_ts),
        })
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SwapData {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
impl SwapData {
    pub fn unpack(input: &[u8; 16]) -> Option<Self> {
        let (amount_in, minimum_amount_out) = array_refs![input, 8, 8];

        Some(SwapData {
            amount_in: u64::from_le_bytes(*amount_in),
            minimum_amount_out: u64::from_le_bytes(*minimum_amount_out),
        })
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WithdrawData {
    pub pool_token_amount: u64,
    pub minimum_token_a_amount: u64,
    pub minimum_token_b_amount: u64,
}
impl WithdrawData {
    pub fn unpack(input: &[u8; 24]) -> Option<Self> {
        let (pool_token_amount, minimum_token_a_amount, minimum_token_b_amount) =
            array_refs![input, 8, 8, 8];

        Some(WithdrawData {
            pool_token_amount: u64::from_le_bytes(*pool_token_amount),
            minimum_token_a_amount: u64::from_le_bytes(*minimum_token_a_amount),
            minimum_token_b_amount: u64::from_le_bytes(*minimum_token_b_amount),
        })
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WithdrawOneData {
    pub pool_token_amount: u64,
    pub minimum_token_amount: u64,
}
impl WithdrawOneData {
    pub fn unpack(input: &[u8; 16]) -> Option<Self> {
        let (pool_token_amount, minimum_token_amount) = array_refs![input, 8, 8];

        Some(WithdrawOneData {
            pool_token_amount: u64::from_le_bytes(*pool_token_amount),
            minimum_token_amount: u64::from_le_bytes(*minimum_token_amount),
        })
    }
}

