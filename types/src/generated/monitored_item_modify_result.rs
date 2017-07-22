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
pub struct MonitoredItemModifyResult {
    pub status_code: StatusCode,
    pub revised_sampling_interval: Double,
    pub revised_queue_size: UInt32,
    pub filter_result: ExtensionObject,
}

impl MessageInfo for MonitoredItemModifyResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::MonitoredItemModifyResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<MonitoredItemModifyResult> for MonitoredItemModifyResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += self.revised_sampling_interval.byte_len();
        size += self.revised_queue_size.byte_len();
        size += self.filter_result.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += self.revised_sampling_interval.encode(stream)?;
        size += self.revised_queue_size.encode(stream)?;
        size += self.filter_result.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let revised_sampling_interval = Double::decode(stream)?;
        let revised_queue_size = UInt32::decode(stream)?;
        let filter_result = ExtensionObject::decode(stream)?;
        Ok(MonitoredItemModifyResult {
            status_code,
            revised_sampling_interval,
            revised_queue_size,
            filter_result,
        })
    }
}
