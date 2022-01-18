use std::sync::Arc;
use libloading::Library;
use massbit_solana_sdk::smart_contract::{InstructionInterface, InstructionParser, SmartContractRegistrar};

const UNPACKING_LIB_PATH : &str = "/home/huy/work/block_chain/indexer/solana-indexer-examples/metaplex/unpack-instruction/target/release/libunpack_instruction.so";
mod load_parser {
    use std::error::Error;
    use super::*;

    #[tokio::test]
    async fn call_instruction_parser() {
        unsafe {
            let lib = Arc::new(Library::new(UNPACKING_LIB_PATH).unwrap());
            // inject store to plugin
            let sm_entrypoint = lib
                .get::<*mut InstructionInterface>(b"entrypoint\0").unwrap()
                .read();
            let mut registrar = SmartContractRegistrar::new();
            (sm_entrypoint.register)(&mut registrar);
            if let Some(proxy) = registrar.parser_proxies.as_ref() {
                let content: [u8; 238] = [1, 1, 16, 0, 0, 0, 67, 108, 117, 98, 32, 83, 117, 105, 116, 101, 32, 35, 52, 56, 50, 48, 5, 0, 0, 0, 83, 85, 73, 84, 69, 92, 0, 0, 0, 104, 116, 116, 112, 115, 58, 47, 47, 98, 97, 102, 121, 98, 101, 105, 102, 104, 52, 52, 53, 119, 101, 116, 115, 113, 110, 102, 117, 105, 108, 100, 102, 121, 98, 102, 108, 122, 107, 119, 116, 101, 106, 118, 118, 122, 51, 99, 52, 50, 51, 110, 114, 113, 121, 107, 112, 106, 109, 53, 108, 104, 115, 106, 119, 103, 108, 105, 46, 105, 112, 102, 115, 46, 100, 119, 101, 98, 46, 108, 105, 110, 107, 47, 52, 56, 49, 57, 46, 106, 115, 111, 110, 244, 1, 1, 3, 0, 0, 0, 139, 166, 189, 24, 187, 126, 222, 195, 58, 127, 249, 242, 55, 191, 228, 160, 168, 87, 114, 208, 22, 222, 84, 150, 230, 71, 25, 54, 238, 230, 36, 207, 1, 0, 6, 65, 156, 6, 69, 48, 246, 214, 66, 103, 66, 217, 59, 252, 205, 155, 71, 130, 219, 39, 90, 159, 10, 158, 105, 79, 210, 149, 191, 206, 251, 237, 0, 10, 13, 10, 167, 88, 83, 155, 49, 35, 66, 86, 112, 205, 49, 38, 22, 30, 22, 59, 47, 75, 110, 133, 40, 188, 60, 130, 30, 149, 65, 28, 200, 32, 0, 90, 0, 0];
                match proxy.unpack_instruction(&content) {
                    Ok(value) => {
                        println!("{:?}", &value);
                    }
                    Err(err) => {
                        println!("{:?}", &err);
                    }
                }


            }
        }
        assert!(true)
    }
}