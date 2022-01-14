pub mod processor;
pub mod instruction;

use lazy_static::lazy_static;
use std::env;
use std::error::Error;
use std::sync::Arc;
use transport::{interface::{InterfaceRegistrar, InstructionInterface}, export_interface};
use crate::instruction::InstructionParser;

export_interface!(register);

#[allow(dead_code, improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn InterfaceRegistrar) {
    registrar.register_parser(Box::new(InstructionParser));
}