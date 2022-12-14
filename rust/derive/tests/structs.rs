// Derivation macro library for strict encoding.
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

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding_test;

mod common;

use common::Result;
use strict_encoding_test::test_encoding_roundtrip;

#[test]
fn struct_numbered_fields() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug)]
    #[derive(StrictEncode, StrictDecode)]
    struct NumberedFields(u8, String);

    let fields = NumberedFields(7, s!("some"));
    test_encoding_roundtrip(&fields, [
        0x07, 0x04, 0x00, b's', b'o', b'm', b'e',
    ])?;

    Ok(())
}
