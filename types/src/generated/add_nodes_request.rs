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
use generated::AddNodesItem;

/// Adds one or more nodes to the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct AddNodesRequest {
    pub request_header: RequestHeader,
    pub nodes_to_add: Option<Vec<AddNodesItem>>,
}

impl MessageInfo for AddNodesRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::AddNodesRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<AddNodesRequest> for AddNodesRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.nodes_to_add);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.nodes_to_add)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let nodes_to_add: Option<Vec<AddNodesItem>> = read_array(stream)?;
        Ok(AddNodesRequest {
            request_header,
            nodes_to_add,
        })
    }
}
