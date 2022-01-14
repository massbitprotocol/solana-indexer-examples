use std::sync::Arc;
use libloading::Library;
use massbit_solana_sdk::smart_contract::{InstructionInterface, InstructionParser, SmartContractRegistrar};

const UNPACKING_LIB_PATH : &str = "/home/viettai/Massbit/solana-indexer-examples/metaplex/unpack-instruction/target/release/libunpack_instruction.so";
mod load_parser {
    use super::*;

    #[tokio::test]
    async fn call_instruction_parser() {
        println!("Test");
        unsafe {
            let lib = Arc::new(Library::new(UNPACKING_LIB_PATH).unwrap());
            // inject store to plugin
            let sm_entrypoint = lib
                .get::<*mut InstructionInterface>(b"entrypoint\0").unwrap()
                .read();
            let mut registrar = SmartContractRegistrar::new();
            (sm_entrypoint.register)(&mut registrar);
            if let Some(proxy) = registrar.parser_proxies.as_ref() {
                let content: [u8; 2] = [1, 2];
                proxy.unpack_instruction(&content);
            }
        }
        assert!(true)
    }
}