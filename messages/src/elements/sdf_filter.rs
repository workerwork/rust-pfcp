use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits	
// Octets	8	7	6	5	4	3	2	1	
// 1 to 2	    Type = 23 (decimal)	
// 3 to 4	    Length = n	
// 5	        Spare	BID	FL	SPI	TTC	FD	
// 6	        Spare	
// m to (m+1)	Length of Flow Description	
// (m+2) to p	Flow Description	
// s to (s+1)	ToS Traffic Class	
// t to (t+3)	Security Parameter Index	
// v to (v+2)	Flow Label	
// w to (w+3)	SDF Filter ID	
// x to (n+4)	These octet(s) is/are present only if explicitly specified	
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct SDFFilter {
    ie_type: u16,
    ie_len: u16,

    //Bit 1 – FD (Flow Description): If this bit is set to "1", then the Length of Flow Description
    //and the Flow Description fields shall be present, otherwise they shall not be present.
    //
    //Bit 2 – TTC (ToS Traffic Class): If this bit is set to "1", then the ToS Traffic Class field
    //shall be present, otherwise the ToS Traffic Class field shall not be present.
    //
    //Bit 3 – SPI (Security Parameter Index): If this bit is set to "1", then the Security
    //Parameter Index field shall be present, otherwise the Security Parameter Index field shall
    //not be present.
    //
    //Bit 4 – FL (Flow Label): If this bit is set to "1", then the Flow Label field shall be
    //present, otherwise the Flow Label field shall not be present.
    //
    //Bit 5 – BID (Bidirectional SDF Filter): If this bit is set to "1", then the SDF Filter ID
    //shall be present, otherwise the SDF Filter ID shall not be present.
    //
    //Bit 6 to 8: Spare, for future use and set to "0".
    mask: u8,   //M
    
    spare: u8,  //M
    flow_description: Option<Vec<u8>>,  //C
    tos_traffic_class: Option<Vec<u8>>, //C 2bytes
    security_parameter_index: Option<Vec<u8>>,  //C 4bytes
    flow_label: Option<Vec<u8>>,    //C 3bytes
    sdf_filter_id: Option<Vec<u8>>, //C 4bytes
}

impl SDFFilter {
    pub fn decode(buf: &mut [u8], len: u16) -> Result<SDFFilter, PFCPError> {
        let mut element = SDFFilter {
            ie_type: ie_type::SDF_FILTER,
            ie_len: len,
            ..Default::default()
        };
        element.mask = buf[0];
        element.spare = buf[1];
        buf = &mut buf[2..];
        if element.mask & 0b0000_0001 != 0 {
            let l = buf[0] * 16 + buf[1];
            element.flow_description = Some(buf[2..=l+1].to_vec());
            buf = &mut buf[l+2..];
        }
        if element.mask & 0b0000_0010 != 0 {
            element.tos_traffic_class = Some(buf[0..=1].to_vec());
            buf = &mut buf[2..];
        }
        if element.mask & 0b0000_0100 != 0 {
            element.security_parameter_index = Some(buf[0..=3].to_vec());
            buf = &mut buf[4..];
        }
        if element.mask & 0b0000_1000 != 0 {
            element.flow_label = Some(buf[0..=2].to_vec());
            buf = &mut buf[3..];
        }
        if element.mask & 0b0001_0000 != 0 {
            element.sdf_filter_id = Some(buf[0..=3].to_vec());
        }
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.mask);
        element_vec.push(self.spare);
        if let Some(flow_description) = self.flow_description {
            element_vec.append(&mut flow_description);
        }
        if let Some(tos_traffic_class) = self.tos_traffic_class {
            element_vec.append(&mut tos_traffic_class);
        }
        if let Some(security_parameter_index) = self.security_parameter_index {
            element_vec.append(&mut security_parameter_index);
        }
        if let Some(flow_label) = self.flow_label {
            element_vec.append(&mut flow_label);
        }
        if let Some(sdf_filter_id) = self.sdf_filter_id {
            element_vec.append(&mut sdf_filter_id);
        }
        element_vec
    }
}

