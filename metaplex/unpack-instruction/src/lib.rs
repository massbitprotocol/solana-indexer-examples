pub mod processor;
pub mod mapping;

use lazy_static::lazy_static;
// use massbit_solana_sdk::{
//     export_plugin,
//     plugin::{handler::SolanaHandler, PluginRegistrar},
//     store::IndexStore,
//     types::SolanaBlock,
// };
use std::env;
use std::error::Error;
use std::sync::Arc;

pub const ADDRESS: &str = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin";

// #[doc(hidden)]
// #[no_mangle]
// pub static mut STORE: Option<&mut dyn IndexStore> = None;
//
// export_plugin!(register);
//
// #[allow(dead_code, improper_ctypes_definitions)]
// extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
//     registrar.register_solana_handler(Box::new(SolanaHandlerAdapter));
// }
//
// #[derive(Debug, Clone, PartialEq)]
// pub struct SolanaHandlerAdapter;
//
// impl SolanaHandler for SolanaHandlerAdapter {
//     fn handle_blocks(&self, blocks: &Vec<SolanaBlock>) -> Result<i64, Box<dyn Error>> {
//         let mut block_slot = -1_i64;
//         // Todo: Rewrite the flush so it will flush after finish the array of blocks for better performance. For now, we flush after each block.
//         for block in blocks {
//             mapping::handle_block(block);
//             block_slot = block_slot.max(block.block_number as i64);
//             unsafe {
//                 if let Some(store) = &mut STORE {
//                     store.flush(&block.block.blockhash, block.block_number);
//                 }
//             }
//         }
//         Ok(block_slot)
//     }
// }
