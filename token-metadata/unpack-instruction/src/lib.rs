pub mod instruction;
use crate::instruction::InstructionParser;
use transport::{
    export_interface,
    interface::{InstructionInterface, InterfaceRegistrar},
};
export_interface!(register);
#[allow(dead_code, improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn InterfaceRegistrar) {
    {
        registrar.register_parser(Box::new(InstructionParser));
    }
}
