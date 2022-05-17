/*
 SPDX-License-Identifier: MIT
 Copyright (c) 2022 BitFlux, Inc.
 Installer script for bitflux
*/

mod runcmd;
mod installerRust;

use crate::runcmd::RunCmd;

fn main() {
    RunCmd::new("echo \"Hello World\"").execute();
}