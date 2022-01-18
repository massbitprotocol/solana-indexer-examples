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
lazy_static! {
    pub static ref SOLANA_CLIENT: Arc<RpcClient> = Arc::new(RpcClient::new(
        env::var("SOLANA_RPC_URL").unwrap_or(String::from("http://194.163.156.242:8899"))
    ));
}
pub const ADDRESS: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

#[doc(hidden)]
#[no_mangle]
pub static mut STORE: Option<&mut dyn IndexStore> = None;

export_plugin!(register);

#[allow(dead_code, improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn PluginRegistrar, unpack_lib: &String) {
    registrar.register_solana_handler(Box::new(SolanaHandlerAdapter::new(unpack_lib)));
}

#[derive(Clone)]
pub struct SolanaHandlerAdapter {
    pub registrar: SmartContractRegistrar
}

impl SolanaHandlerAdapter {
    fn new(unpack_lib: &String) -> Self {
        println!(
            "Start new, unpack_lib: {}",
            unpack_lib
        );
        //Load library here
        let mut registrar = SmartContractRegistrar::new();
        unsafe {
            println!("unpack_lib path: {} ", unpack_lib.as_str());
            //let lib = Arc::new(Library::new(unpack_lib.as_str()).unwrap());
            let lib = Arc::new(Library::new("/home/huy/work/block_chain/indexer/solana-indexer-examples/metaplex/unpack-instruction/target/release/libunpack_instruction.so").unwrap());
            // inject store to plugin
            let sm_entrypoint = lib
            .get::<*mut InstructionInterface>(b"entrypoint\0").unwrap()
            .read();
            (sm_entrypoint.register)(&mut registrar);
        };
        Self {
            registrar
        }
    }
    fn get_proxy(&self) -> Option<Arc<SmartContractProxy>> {
        self.registrar.parser_proxies.as_ref().and_then(|proxy| Some(proxy.clone()))
    }
}

impl SolanaHandler for SolanaHandlerAdapter {
    fn handle_blocks(&self, blocks: &Vec<SolanaBlock>) -> Result<i64, Box<dyn Error>> {
        println!("Start handle_blocks, block len: {}", blocks.len());
        let mut block_slot = -1_i64;
        // Todo: Rewrite the flush so it will flush after finish the array of blocks for better performance. For now, we flush after each block.
        let proxy = self.registrar.parser_proxies.as_ref();
        if let Some(proxy) = proxy {
            for block in blocks {
                mapping::handle_block(proxy.clone(), block);
                block_slot = block_slot.max(block.block_number as i64);
                unsafe {
                    if let Some(store) = &mut STORE {
                        store.flush(&block.block.blockhash, block.block_number);
                    }
                }
            }
        }
        Ok(block_slot)
    }
}
