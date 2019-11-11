// Arbitration DLib is the combination of the on-chain protocol and off-chain
// protocol that work together to resolve any disputes that might occur during the
// execution of a Cartesi DApp.

// Copyright (C) 2019 Cartesi Pte. Ltd.

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.

// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
// PARTICULAR PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Note: This component currently has dependencies that are licensed under the GNU
// GPL, version 3, and so you should treat this component as a whole as being under
// the GPL version 3. But all Cartesi-written code in this component is licensed
// under the Apache License, version 2, or a compatible permissive license, and can
// be used independently under the Apache v2 license. After this component is
// rewritten, the entire component will be released under the Apache v2 license.


#![warn(unused_extern_crates)]
pub mod compute;
pub mod mm;
pub mod partition;
pub mod vg;
pub mod emulator_service;

extern crate configuration;
extern crate error;
extern crate grpc;
extern crate bytes;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate dispatcher;
extern crate ethabi;
extern crate ethereum_types;
extern crate transaction;
extern crate emulator_interface;

use ethereum_types::{Address, U256};

pub use compute::{Compute, ComputeCtx, ComputeCtxParsed};
pub use mm::MM;
pub use partition::Partition;
pub use vg::{VG, VGCtx, VGCtxParsed};
pub use emulator_interface::{cartesi_base, manager_high};
pub use emulator_service::{
    AccessOperation, NewSessionRequest, NewSessionResult,
    SessionRunRequest, SessionStepRequest,
    SessionRunResult, SessionStepResult,
    SessionReadMemoryRequest, SessionReadMemoryResult,
    SessionGetProofRequest, SessionGetProofResult,
    EMULATOR_SERVICE_NAME, EMULATOR_METHOD_NEW,
    EMULATOR_METHOD_RUN, EMULATOR_METHOD_STEP,
    EMULATOR_METHOD_READ, EMULATOR_METHOD_PROOF};

#[derive(Debug)]
enum Role {
    Claimer,
    Challenger,
}

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// we need to have a proper way to construct machine ids.
// but this will only make real sense when we have the scripting
// language or some other means to construct a machine inside the
// blockchain.
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

pub fn build_machine_id(_index: U256, _address: &Address) -> String {
    //return format!("{:x}:{}", address, index);
    //return "0000000000000000000000000000000000000000000000008888888888888888"
    //    .to_string();
    return "test_new_session_id".to_string();
}

pub fn build_session_run_key(id: String, times: Vec<u64>) -> String {
    return format!("{}_run_{:?}", id, times);
}

pub fn build_session_step_key(id: String, divergence_time: String) -> String {
    return format!("{}_step_{}", id, divergence_time);
}