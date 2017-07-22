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
pub struct SamplingIntervalDiagnosticsDataType {
    pub sampling_interval: Double,
    pub monitored_item_count: UInt32,
    pub max_monitored_item_count: UInt32,
    pub disabled_monitored_item_count: UInt32,
}

impl MessageInfo for SamplingIntervalDiagnosticsDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::SamplingIntervalDiagnosticsDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SamplingIntervalDiagnosticsDataType> for SamplingIntervalDiagnosticsDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.sampling_interval.byte_len();
        size += self.monitored_item_count.byte_len();
        size += self.max_monitored_item_count.byte_len();
        size += self.disabled_monitored_item_count.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.sampling_interval.encode(stream)?;
        size += self.monitored_item_count.encode(stream)?;
        size += self.max_monitored_item_count.encode(stream)?;
        size += self.disabled_monitored_item_count.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let sampling_interval = Double::decode(stream)?;
        let monitored_item_count = UInt32::decode(stream)?;
        let max_monitored_item_count = UInt32::decode(stream)?;
        let disabled_monitored_item_count = UInt32::decode(stream)?;
        Ok(SamplingIntervalDiagnosticsDataType {
            sampling_interval,
            monitored_item_count,
            max_monitored_item_count,
            disabled_monitored_item_count,
        })
    }
}
