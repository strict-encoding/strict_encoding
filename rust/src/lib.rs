// Strict encoding library for deterministic binary serialization.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2023 UBIDECO Institute
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    //dead_code,
    //missing_docs
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
extern crate amplify;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;

#[macro_use]
mod macros;
mod types;
mod traits;
mod ident;
mod error;
mod reader;
mod writer;
mod util;
mod primitives;
#[cfg(test)]
pub(crate) mod test;

pub use error::{DecodeError, DeserializeError, SerializeError};
pub use ident::{FieldName, Ident, InvalidIdent, LibName, TypeName};
pub use primitives::{constants, NumCls, NumInfo, NumSize, Primitive};
pub use reader::StrictReader;
pub use traits::*;
pub use types::*;
pub use util::{Sizing, Variant};
pub use writer::{SplitParent, StrictParent, StrictWriter, StructWriter, UnionWriter};

pub const NO_LIB: &str = "No";
pub const STD_LIB: &str = "StdLib";
pub const STEN_LIB: &str = "StEn";
