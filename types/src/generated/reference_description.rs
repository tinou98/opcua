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

/// The description of a reference.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceDescription {
    pub reference_type_id: NodeId,
    pub is_forward: Boolean,
    pub node_id: ExpandedNodeId,
    pub browse_name: QualifiedName,
    pub display_name: LocalizedText,
    pub node_class: NodeClass,
    pub type_definition: ExpandedNodeId,
}

impl MessageInfo for ReferenceDescription {
    fn object_id(&self) -> ObjectId {
        ObjectId::ReferenceDescription_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ReferenceDescription> for ReferenceDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.reference_type_id.byte_len();
        size += self.is_forward.byte_len();
        size += self.node_id.byte_len();
        size += self.browse_name.byte_len();
        size += self.display_name.byte_len();
        size += self.node_class.byte_len();
        size += self.type_definition.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_forward.encode(stream)?;
        size += self.node_id.encode(stream)?;
        size += self.browse_name.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.node_class.encode(stream)?;
        size += self.type_definition.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let reference_type_id = NodeId::decode(stream)?;
        let is_forward = Boolean::decode(stream)?;
        let node_id = ExpandedNodeId::decode(stream)?;
        let browse_name = QualifiedName::decode(stream)?;
        let display_name = LocalizedText::decode(stream)?;
        let node_class = NodeClass::decode(stream)?;
        let type_definition = ExpandedNodeId::decode(stream)?;
        Ok(ReferenceDescription {
            reference_type_id,
            is_forward,
            node_id,
            browse_name,
            display_name,
            node_class,
            type_definition,
        })
    }
}
