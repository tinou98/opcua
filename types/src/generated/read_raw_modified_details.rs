// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use attribute::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use generated::node_ids::*;
#[allow(unused_imports)]
use generated::status_codes::StatusCode;
#[allow(unused_imports)]
use generated::status_codes::StatusCode::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ReadRawModifiedDetails {
    pub is_read_modified: Boolean,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub num_values_per_node: UInt32,
    pub return_bounds: Boolean,
}

impl BinaryEncoder<ReadRawModifiedDetails> for ReadRawModifiedDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.is_read_modified.byte_len();
        size += self.start_time.byte_len();
        size += self.end_time.byte_len();
        size += self.num_values_per_node.byte_len();
        size += self.return_bounds.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.is_read_modified.encode(stream)?;
        size += self.start_time.encode(stream)?;
        size += self.end_time.encode(stream)?;
        size += self.num_values_per_node.encode(stream)?;
        size += self.return_bounds.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let is_read_modified = Boolean::decode(stream)?;
        let start_time = DateTime::decode(stream)?;
        let end_time = DateTime::decode(stream)?;
        let num_values_per_node = UInt32::decode(stream)?;
        let return_bounds = Boolean::decode(stream)?;
        Ok(ReadRawModifiedDetails {
            is_read_modified,
            start_time,
            end_time,
            num_values_per_node,
            return_bounds,
        })
    }
}
