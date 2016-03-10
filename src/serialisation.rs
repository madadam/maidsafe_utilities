// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use rmp_serialize::{decode, Decoder, encode, Encoder};
use rustc_serialize::{Decodable, Encodable};
use std::io::{Read, Write, Cursor};

quick_error! {
    /// Serialisation error.
    #[derive(Debug)]
    pub enum SerialisationError {
        /// Error during serialisation (encoding).
        Serialise(err: encode::Error) {
            description("Serialise error")
            display("Serialise error: {}", err)
            cause(err)
            from()
        }

        /// Error during deserialisation (decoding).
        Deserialise(err: decode::Error) {
            description("Deserialise error")
            display("Deserialise error: {}", err)
            cause(err)
            from()
        }
    }
}

/// Serialise an Encodable type
pub fn serialise<T>(data: &T) -> Result<Vec<u8>, SerialisationError>
    where T: Encodable
{
    let mut buffer = Vec::new();
    try!(serialise_into(data, &mut buffer));
    Ok(buffer)
}

/// Deserialise a Decodable type
pub fn deserialise<T>(data: &[u8]) -> Result<T, SerialisationError>
    where T: Decodable
{
    let mut data = Cursor::new(data);
    deserialise_from(&mut data)
}

/// Serialise an Encodable type directly into a Write.
pub fn serialise_into<T: Encodable, W: Write>(data: &T,
                                              write: &mut W)
                                              -> Result<(), SerialisationError> {
    let mut encoder = Encoder::new(write);
    Ok(try!(data.encode(&mut encoder)))
}

/// Deserialise a Decodable type directly from a Read
pub fn deserialise_from<R: Read, T: Decodable>(read: &mut R) -> Result<T, SerialisationError> {
    let mut decoder = Decoder::new(read);
    Ok(try!(Decodable::decode(&mut decoder)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialise_deserialise() {
        let original_data = (vec![0u8, 1, 3, 9],
                             vec![-1i64, 888, -8765],
                             "Some-String".to_owned());

        let serialised_data = unwrap_result!(serialise(&original_data));
        let deserialised_data: (Vec<u8>, Vec<i64>, String) =
            unwrap_result!(deserialise(&serialised_data));
        assert_eq!(original_data, deserialised_data);
    }
}
