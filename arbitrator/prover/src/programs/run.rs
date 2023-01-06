// Copyright 2022, Offchain Labs, Inc.
// For license information, see https://github.com/nitro/blob/master/LICENSE

use eyre::{ensure, Result};
use std::fmt::Display;

use crate::Machine;

use super::{
    config::StylusConfig,
    depth::DepthCheckedMachine,
    meter::{MachineMeter, MeteredMachine},
    STYLUS_ENTRY_POINT, USER_HOST,
};

pub enum UserOutcome {
    Success(Vec<u8>),
    Revert(Vec<u8>),
    OutOfGas,
    OutOfStack,
}

impl Display for UserOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UserOutcome::*;
        match self {
            Success(output) => write!(f, "success {}", hex::encode(output)),
            Revert(output) => write!(f, "revert {}", hex::encode(output)),
            OutOfGas => write!(f, "out of gas"),
            OutOfStack => write!(f, "out of stack"),
        }
    }
}

pub trait RunProgram {
    fn run_main(&mut self, args: Vec<u8>, config: &StylusConfig) -> Result<UserOutcome>;
}

impl RunProgram for Machine {
    fn run_main(&mut self, args: Vec<u8>, config: &StylusConfig) -> Result<UserOutcome> {
        let pricing = &config.pricing;

        macro_rules! call {
            ($module:expr, $func:expr, $args:expr) => {
                call!($module, $func, $args, |error| Err(error))
            };
            ($module:expr, $func:expr, $args:expr, $error:expr) => {{
                match self.call_function($module, $func, $args) {
                    Ok(value) => value[0].try_into().unwrap(),
                    Err(error) => return $error(error),
                }
            }};
        }

        // push the args
        let args_len = (args.len() as u32).into();
        let push_vec = vec![
            args_len,
            pricing.wasm_gas_price.into(),
            pricing.hostio_cost.into(),
        ];
        let args_ptr = call!(USER_HOST, "push_program", push_vec);
        let user_host = self.find_module(USER_HOST)?;
        self.write_memory(user_host, args_ptr, &args)?;

        let status: u32 = call!("user", STYLUS_ENTRY_POINT, vec![args_len], |error| {
            if self.gas_left() == MachineMeter::Exhausted {
                return Ok(UserOutcome::OutOfGas);
            }
            if self.stack_left() == 0 {
                return Ok(UserOutcome::OutOfStack);
            }
            return Err(error);
        });

        let outs_len = call!(USER_HOST, "get_output_len", vec![]);
        let outs_ptr = call!(USER_HOST, "get_output_ptr", vec![]);
        let outs = self.read_memory(user_host, outs_len, outs_ptr)?.to_vec();

        let num_progs: u32 = call!(USER_HOST, "pop_program", vec![]);
        ensure!(num_progs == 0, "dirty user_host");

        Ok(match status {
            0 => UserOutcome::Success(outs),
            _ => UserOutcome::Revert(outs),
        })
    }
}