use std::io::{Read, Write};

use basic_types::{LocalizedText, Int32, UInt32};
use node_id::NodeId;
use string::UAString;
use status_codes::StatusCode;
use encoding::*;

// From OPC UA Part 3 - Address Space Model 1.03 Specification
//
// This Structured DataType defines a Method input or output argument specification. It is for
// example used in the input and output argument Properties for Methods. Its elements are described in
// Table23

pub struct Argument {
    pub name: UAString,
    pub data_type: NodeId,
    pub value_rank: Int32,
    pub array_dimensions: Option<Vec<UInt32>>,
    pub description: LocalizedText,
}

impl BinaryEncoder<Argument> for Argument {
    fn byte_len(&self) -> usize {
        // Length plus the actual length of bytes (if not null)
        let mut size = 0;
        size += self.name.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();

        // Array dimensions
        size += 4;
        if self.value_rank > 0 {
            let array_dimensions = self.array_dimensions.as_ref().unwrap();
            size += 4 * array_dimensions.len();
        }

        size += self.description.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.name.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        // Encode the array dimensions
        if self.value_rank > 0 {
            if let Some(ref array_dimensions) = self.array_dimensions {
                if self.value_rank as usize != array_dimensions.len() {
                    error!("The array dimensions {} of the Argument should match value rank {} and they don't", array_dimensions.len(), self.value_rank);
                    return Err(StatusCode::BadDataEncodingInvalid);
                }
                size += write_u32(stream, array_dimensions.len() as UInt32)?;
                for d in array_dimensions.iter() {
                    size += d.encode(stream)?;
                }
            } else {
                error!("The array dimensions are expected in the Argument matching value rank {} and they aren't", self.value_rank);
                return Err(StatusCode::BadDataEncodingInvalid);
            }
        } else {
            size += write_u32(stream, 0)?;
        }

        size += self.description.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let name = UAString::decode(stream)?;
        let data_type = NodeId::decode(stream)?;
        let value_rank = Int32::decode(stream)?;
        // Decode array dimensions
        let array_dimensions_len = UInt32::decode(stream)?;
        let array_dimensions = if array_dimensions_len > 0 {
            if value_rank > 0 && value_rank as UInt32 != array_dimensions_len {
                error!("The array dimensions {} of the Argument should match value rank {} and they don't", array_dimensions_len, value_rank);
                return Err(StatusCode::BadDataEncodingInvalid);
            }
            let mut array_dimensions = Vec::with_capacity(array_dimensions_len as usize);
            for _ in 0..array_dimensions_len {
                array_dimensions.push(read_u32(stream)?);
            }
            Some(array_dimensions)
        } else {
            None
        };
        let description = LocalizedText::decode(stream)?;
        Ok(Argument {
            name,
            data_type,
            value_rank,
            array_dimensions,
            description,
        })
    }
}