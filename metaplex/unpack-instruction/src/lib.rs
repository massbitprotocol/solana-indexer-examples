pub mod instruction;

use transport::{interface::{InterfaceRegistrar, InstructionInterface}, export_interface};
use crate::instruction::InstructionParser;

export_interface!(register);

#[allow(dead_code, improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn InterfaceRegistrar) {
    registrar.register_parser(Box::new(InstructionParser));
}