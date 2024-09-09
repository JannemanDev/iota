// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This module contains the public APIs supported by the bytecode verifier.

use iota_protocol_config::ProtocolConfig;
use iota_types::{error::ExecutionError, move_package::FnInfoMap};
use move_binary_format::file_format::CompiledModule;
use move_bytecode_verifier_meter::{dummy::DummyMeter, Meter};

use crate::{
    entry_points_verifier, global_storage_access_verifier, id_leak_verifier,
    one_time_witness_verifier, private_generics, struct_with_key_verifier,
};

/// Helper for a "canonical" verification of a module.
pub fn iota_verify_module_metered(
    config: &ProtocolConfig,
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
    meter: &mut (impl Meter + ?Sized),
) -> Result<(), ExecutionError> {
    struct_with_key_verifier::verify_module(module)?;
    global_storage_access_verifier::verify_module(module)?;
    id_leak_verifier::verify_module(module, meter)?;
    private_generics::verify_module(module)?;
    entry_points_verifier::verify_module(config, module, fn_info_map)?;
    one_time_witness_verifier::verify_module(module, fn_info_map)
}

/// Runs the Iota verifier and checks if the error counts as a Iota verifier
/// timeout NOTE: this function only check if the verifier error is a timeout
/// All other errors are ignored
pub fn iota_verify_module_metered_check_timeout_only(
    config: &ProtocolConfig,
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
    meter: &mut (impl Meter + ?Sized),
) -> Result<(), ExecutionError> {
    // Checks if the error counts as a Iota verifier timeout
    if let Err(error) = iota_verify_module_metered(config, module, fn_info_map, meter) {
        if matches!(
            error.kind(),
            iota_types::execution_status::ExecutionFailureStatus::IotaMoveVerificationTimedout
        ) {
            return Err(error);
        }
    }
    // Any other scenario, including a non-timeout error counts as Ok
    Ok(())
}

pub fn iota_verify_module_unmetered(
    config: &ProtocolConfig,
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
) -> Result<(), ExecutionError> {
    iota_verify_module_metered(config, module, fn_info_map, &mut DummyMeter).map_err(|err| {
        // We must never see timeout error in execution
        debug_assert!(
            !matches!(
                err.kind(),
                iota_types::execution_status::ExecutionFailureStatus::IotaMoveVerificationTimedout
            ),
            "Unexpected timeout error in execution"
        );
        err
    })
}
