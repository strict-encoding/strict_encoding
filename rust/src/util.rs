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

use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};

use crate::FieldName;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Sizing {
    pub min: u16,
    pub max: u16,
}

impl Sizing {
    pub const ONE: Sizing = Sizing { min: 1, max: 1 };

    pub const U8: Sizing = Sizing {
        min: 0,
        max: u8::MAX as u16,
    };

    pub const U16: Sizing = Sizing {
        min: 0,
        max: u16::MAX,
    };

    pub const U8_NONEMPTY: Sizing = Sizing {
        min: 1,
        max: u8::MAX as u16,
    };

    pub const U16_NONEMPTY: Sizing = Sizing {
        min: 1,
        max: u16::MAX,
    };

    pub const fn new(min: u16, max: u16) -> Self { Sizing { min, max } }

    pub const fn fixed(len: u16) -> Self { Sizing { min: len, max: len } }
}

impl Display for Sizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (self.min, self.max) {
            (0, u16::MAX) => Ok(()),
            (0, max) => write!(f, " ^ ..{}", max),
            (min, u16::MAX) => write!(f, " ^ {}..", min),
            (min, max) => write!(f, " ^ {}..{:#04x}", min, max),
        }
    }
}

#[derive(Clone, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Field {
    pub name: Option<FieldName>,
    pub ord: u8,
}

impl Field {
    pub fn named(name: FieldName, value: u8) -> Field {
        Field {
            name: Some(name),
            ord: value,
        }
    }
    pub fn unnamed(value: u8) -> Field {
        Field {
            name: None,
            ord: value,
        }
    }

    pub fn none() -> Field {
        Field {
            name: Some(FieldName::from("None")),
            ord: 0,
        }
    }
    pub fn some() -> Field {
        Field {
            name: Some(FieldName::from("Some")),
            ord: 1,
        }
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        match (&self.name, &other.name) {
            (None, None) => self.ord == other.ord,
            (Some(name1), Some(name2)) => name1 == name2 || self.ord == other.ord,
            _ => false,
        }
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        self.ord.cmp(&other.ord)
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", name)?;
        }
        if f.alternate() {
            if self.name.is_some() {
                f.write_str(" = ")?;
            }
            Display::fmt(&self.ord, f)?;
        }
        Ok(())
    }
}