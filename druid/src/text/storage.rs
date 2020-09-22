// Copyright 2020 The xi-editor Authors.
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

//! A type for representing editable, selectable text buffers.

use std::ops::RangeBounds;
use std::sync::Arc;

use super::{Attribute, AttributeSpans};
use crate::piet::{util, TextLayoutBuilder};
use crate::{Data, Env};

/// A type that represents text that can be displayed.
pub trait TextStorage {
    fn as_str(&self) -> &str;

    fn as_arc_str(&self) -> ArcStr {
        self.as_str().into()
    }

    #[allow(unused_variables)]
    fn add_attributes<T: TextLayoutBuilder>(&self, builder: T, env: &Env) -> T {
        builder
    }
}

/// A reference counted string slice.
///
/// This is a data-friendly way to represent strings in druid. Unlike `String`
/// it cannot be mutated, but unlike `String` it can be cheaply cloned.
pub type ArcStr = Arc<str>;

#[derive(Debug, Clone, Data)]
pub struct RichText {
    buffer: ArcStr,
    attrs: Arc<AttributeSpans>,
}

impl RichText {
    pub fn new(buffer: ArcStr) -> Self {
        RichText {
            buffer,
            attrs: Arc::new(AttributeSpans::default()),
        }
    }

    /// The length of the buffer, in utf8 code units.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn add(&mut self, range: impl RangeBounds<usize>, attr: Attribute) {
        let range = util::resolve_range(range, self.buffer.len());
        Arc::make_mut(&mut self.attrs).add(range, attr);
    }
}

impl TextStorage for ArcStr {
    fn as_str(&self) -> &str {
        self
    }

    fn as_arc_str(&self) -> ArcStr {
        self.clone()
    }
}

impl TextStorage for RichText {
    fn as_str(&self) -> &str {
        self.buffer.as_str()
    }

    fn as_arc_str(&self) -> ArcStr {
        self.buffer.clone()
    }

    fn add_attributes<T: TextLayoutBuilder>(&self, mut builder: T, env: &Env) -> T {
        for (range, attr) in self.attrs.to_piet_attrs(env) {
            builder = builder.range_attribute(range, attr);
        }
        builder
    }
}
