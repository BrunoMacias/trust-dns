/*
 * Copyright (C) 2015 Benjamin Fry <benjaminfry@me.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use ::error::{DecodeError, DecodeResult};
use ::rr::record_type::RecordType;

//#[allow(plugin_as_library)] extern crate rustlex;
include!(concat!(env!("OUT_DIR"), "/master_lex.rs"));

/// This is non-destructive to the inner buffer, b/c for pointer types we need to perform a reverse
///  seek to lookup names
///
/// A note on serialization, there was a thought to have this implement the rustc-serialization,
///  but given that this is such a small subset of all the serialization which that performs
///  this is a simpler implementation without the cruft, at least for serializing to/from the
///  binary DNS protocols. rustc-serialization will be used for other coms, e.g. json over http
pub struct TxtDecoder {
  buffer: Vec<u8>,
  index: usize,
  record_type: Option<RecordType>,
  rdata_length: Option<u16>,
}
