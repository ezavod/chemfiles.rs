// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed

//! Chemfiles is a multi-language library written in modern C++ for reading and
//! writing from and to molecular trajectory files. These files are created by
//! your favorite theoretical chemistry program, and contains information about
//! atomic or residues names and positions. Some format also have additional
//! information, such as velocities, forces, energy, …
//!
//! This crate expose the C API of chemfiles to Rust, and make all the
//! functionalities accessible. For more information on the C++ library,
//! please see its [documentation][cxx_doc]. Specifically, the following pages
//! are worth reading:
//!
//! - The [overview][overview] of the classes organisation;
//! - The list of [supported formats][formats];
//! - The documentation for the [selection language][selections];
//!
//! [cxx_doc]: https://chemfiles.org/chemfiles
//! [overview]: https://chemfiles.org/chemfiles/latest/overview.html
//! [formats]: https://chemfiles.org/chemfiles/latest/formats.html
//! [selections]: https://chemfiles.org/chemfiles/latest/selections.html

#![deny(missing_docs)]
#![warn(trivial_casts, unused_import_braces, variant_size_differences)]
#![warn(unused_qualifications, unused_results)]
// Configuration for clippy lints
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_return, clippy::redundant_field_names, clippy::use_self)]
#![allow(clippy::missing_docs_in_private_items, clippy::or_fun_call, clippy::indexing_slicing)]
#![allow(clippy::module_name_repetitions)]

// deny(warnings) in doc tests
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(unused_variables))))]

#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate chemfiles_sys;
use chemfiles_sys::{chfl_add_configuration, chfl_version};

mod strings;

mod errors;
pub use errors::{Error, Status};
pub use errors::set_warning_callback;

mod atom;
pub use atom::Atom;
pub use atom::AtomRef;
pub use atom::AtomMut;

mod cell;
pub use cell::UnitCell;
pub use cell::UnitCellRef;
pub use cell::UnitCellMut;
pub use cell::CellShape;

mod residue;
pub use residue::Residue;
pub use residue::ResidueRef;

mod topology;
pub use topology::Topology;
pub use topology::TopologyRef;
pub use topology::BondOrder;

mod frame;
pub use frame::Frame;

mod trajectory;
pub use trajectory::Trajectory;

mod selection;
pub use selection::{Match, Selection};

mod property;
pub use property::Property;
pub use property::PropertiesIter;

/// Get the version of the chemfiles library.
///
/// # Example
/// ```
/// let version = chemfiles::version();
/// assert!(version.starts_with("0.9"));
/// ```
pub fn version() -> String {
    unsafe { strings::from_c(chfl_version()) }
}

/// Read configuration data from the file at `path`.
///
/// By default, chemfiles reads configuration from any file name `.chemfilesrc`
/// in the current directory or any parent directory. This function can be used
/// to add data from another configuration file.
///
/// This function will fail if there is no file at `path`, or if the file is
/// incorrectly formatted. Data from the new configuration file will overwrite
/// any existing data.
///
/// # Example
/// ```no_run
/// chemfiles::add_configuration("local-config.toml").unwrap();
/// // from now on, the data from "local-config.toml" will be used
/// ```
pub fn add_configuration<S>(path: S) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let buffer = strings::to_c(path.as_ref());
    unsafe {
        errors::check(chfl_add_configuration(buffer.as_ptr()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        assert!(::version().len() > 0);
        assert!(::version().starts_with("0.9"));
    }
}
