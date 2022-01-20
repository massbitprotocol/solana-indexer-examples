pub mod generated;
pub mod mapping;

use lazy_static::lazy_static;
use massbit_solana_sdk::{export_plugin, plugin::{handler::SolanaHandler, PluginRegistrar}, store::IndexStore, types::SolanaBlock};
use solana_client::rpc_client::RpcClient;
use std::env;
use std::error::Error;
use std::sync::Arc;
use libloading::Library;
use massbit_solana_sdk::smart_contract::SmartContractProxy;
use massbit_solana_sdk::smart_contract::{InstructionInterface, InstructionParser, SmartContractRegistrar};
use massbit_solana_sdk::transport::interface::InterfaceRegistrar;
lazy_static! {
    pub static ref SOLANA_CLIENT: Arc<RpcClient> = Arc::new(RpcClient::new(
        env::var("SOLANA_RPC_URL").unwrap_or(String::from("http://194.163.156.242:8899"))
    ));
}
pub const ADDRESS: &str = "p1exdMJcjVao65QdewkaZRUnU6VPSXhus9n2GzWfh98";

#[doc(hidden)]
#[no_mangle]
pub static mut STORE: Option<&mut dyn IndexStore> = None;
#[no_mangle]
pub static mut INTERFACE: Option<&mut dyn InstructionParser> = None;
export_plugin!(register);

#[allow(dead_code, improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_solana_handler(Box::new(SolanaHandlerAdapter));
}

#[derive(Clone)]
pub struct SolanaHandlerAdapter;


impl SolanaHandler for SolanaHandlerAdapter {
     fn handle_blocks(&self, blocks: &Vec<SolanaBlock>) -> Result<i64, Box<dyn Error>> {
        println!("Start handle_blocks, block len: {}", blocks.len());
        let mut block_slot = -1_i64;
        // Todo: Rewrite the flush so it will flush after finish the array of blocks for better performance. For now, we flush after each block.
         unsafe {
             if let Some(interface) = INTERFACE.as_mut() {
                 for block in blocks {
                     mapping::handle_block(*interface, block);
                     block_slot = block_slot.max(block.block_number as i64);
                     if let Some(store) = &mut STORE {
                         store.flush(&block.block.blockhash, block.block_number);
                     }
                 }
             }
         }
        Ok(block_slot)
    }
}
