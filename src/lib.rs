/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/

//! Chemfiles is a multi-language library written in modern C++ for reading and
//! writing from and to molecular trajectory files. These files are created by
//! your favorite theoretical chemistry program, and contains informations about
//! atomic or residues names and positions. Some format also have additional
//! informations, such as velocities, forces, energy, …
//!
//! This crate expose the C API of chemfiles to Rust, and make all the
//! functionalities accessibles. For more informations on the C++ library,
//! please see its [documentation](http://chemfiles.rtfd.org). Specifically, the
//! following pages are worth reading:
//!
//! - The [overview](http://chemfiles.rtfd.org/en/latest/overview.html) of the
//!   classes organisation;
//! - The [supported formats](http://chemfiles.rtfd.org/en/latest/formats.html);
//!
//!
//! As all the function call the underlying C library, they all can fail and
//! thus all return a `Result<_, Error>` value.
#![deny(missing_docs)]

#[macro_use] extern crate lazy_static;

extern crate chemfiles_sys;
use chemfiles_sys::chfl_version;

#[macro_use] mod tests;

mod string;

mod errors;
pub use errors::{Error, ErrorKind};

mod logging;
pub use logging::{Logger, LogLevel};

mod atom;
pub use atom::Atom;
pub use atom::AtomType;

mod cell;
pub use cell::UnitCell;
pub use cell::CellType;

mod topology;
pub use topology::Topology;

mod frame;
pub use frame::Frame;

mod trajectory;
pub use trajectory::Trajectory;

mod selection;
pub use selection::{Selection, Match};

/// Get the version of the chemfiles library
pub fn version() -> String {
    unsafe {
        string::from_c(chfl_version())
    }
}
