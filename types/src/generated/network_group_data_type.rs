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
use generated::EndpointUrlListDataType;

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkGroupDataType {
    pub server_uri: UAString,
    pub network_paths: Option<Vec<EndpointUrlListDataType>>,
}

impl MessageInfo for NetworkGroupDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::NetworkGroupDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<NetworkGroupDataType> for NetworkGroupDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.server_uri.byte_len();
        size += byte_len_array(&self.network_paths);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.server_uri.encode(stream)?;
        size += write_array(stream, &self.network_paths)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let server_uri = UAString::decode(stream)?;
        let network_paths: Option<Vec<EndpointUrlListDataType>> = read_array(stream)?;
        Ok(NetworkGroupDataType {
            server_uri,
            network_paths,
        })
    }
}
