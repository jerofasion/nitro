;; Copyright 2022-2023, Offchain Labs, Inc.
;; For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE

(module
    (func (export "forward__read_args")              (param i32) unreachable)
    (func (export "forward__write_result")           (param i32 i32) unreachable)
    (func (export "forward__account_load_bytes32")   (param i32 i32) unreachable)
    (func (export "forward__account_store_bytes32")  (param i32 i32) unreachable)
    (func (export "forward__call_contract")          (param i32 i32 i32 i32 i64 i32) (result i32) unreachable)
    (func (export "forward__delegate_call_contract") (param i32 i32 i32 i64 i32) (result i32) unreachable)
    (func (export "forward__static_call_contract")   (param i32 i32 i32 i64 i32) (result i32) unreachable)
    (func (export "forward__create1")                (param i32 i32 i32 i32 i32) unreachable)
    (func (export "forward__create2")                (param i32 i32 i32 i32 i32 i32) unreachable)
    (func (export "forward__read_return_data")       (param i32 i32 i32) (result i32) unreachable)
    (func (export "forward__return_data_size")       (result i32) unreachable)
    (func (export "forward__emit_log")               (param i32 i32 i32) unreachable)
    (func (export "forward__account_balance")        (param i32 i32) unreachable)
    (func (export "forward__account_codehash")       (param i32 i32) unreachable)
    (func (export "forward__evm_gas_left")           (result i64) unreachable)
    (func (export "forward__evm_ink_left")           (result i64) unreachable)
    (func (export "forward__block_basefee")          (param i32) unreachable)
    (func (export "forward__chainid")                (param i32) unreachable)
    (func (export "forward__block_coinbase")         (param i32) unreachable)
    (func (export "forward__block_gas_limit")        (result i64) unreachable)
    (func (export "forward__block_number")           (param i32) unreachable)
    (func (export "forward__block_timestamp")        (result i64) unreachable)
    (func (export "forward__contract_address")       (param i32) unreachable)
    (func (export "forward__msg_sender")             (param i32) unreachable)
    (func (export "forward__msg_value")              (param i32) unreachable)
    (func (export "forward__tx_gas_price")           (param i32) unreachable)
    (func (export "forward__tx_ink_price")           (result i64) unreachable)
    (func (export "forward__tx_origin")              (param i32) unreachable)
    (func (export "forward__memory_grow")            (param i32) unreachable)
)
