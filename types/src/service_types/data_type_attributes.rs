// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::enums::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use service_types::impls::*;
#[allow(unused_imports)]
use node_ids::ObjectId;
#[allow(unused_imports)]
use status_codes::StatusCode;

/// The attributes for a data type node.
#[derive(Debug, Clone, PartialEq)]
pub struct DataTypeAttributes {
    pub specified_attributes: UInt32,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
    pub write_mask: UInt32,
    pub user_write_mask: UInt32,
    pub is_abstract: Boolean,
}

impl BinaryEncoder<DataTypeAttributes> for DataTypeAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.is_abstract.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += self.is_abstract.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let specified_attributes = UInt32::decode(stream)?;
        let display_name = LocalizedText::decode(stream)?;
        let description = LocalizedText::decode(stream)?;
        let write_mask = UInt32::decode(stream)?;
        let user_write_mask = UInt32::decode(stream)?;
        let is_abstract = Boolean::decode(stream)?;
        Ok(DataTypeAttributes {
            specified_attributes,
            display_name,
            description,
            write_mask,
            user_write_mask,
            is_abstract,
        })
    }
}