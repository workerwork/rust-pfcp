use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//                      Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 57 (decimal)
// 3 to 4	    Length = n
// 5	           Spare	        V4	V6
// 6 to 13	            SEID
// m to (m+3)	    IPv4 address
// p to (p+15)	    IPv6 address
// k to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct FSEID {
    ie_type: u16,
    ie_len: u16,

    //Bit 1 – V6: If this bit is set to "1", then IPv6 address field shall be present in the
    //F-SEID, otherwise the IPv6 address field is not present at all.
    //
    //Bit 2 – V4: If this bit is set to "1", then IPv4 address field shall be present in the
    //F-SEID, otherwise the IPv4 address field is not present at all.
    //
    //Bit 3 to 8 are spare and reserved for future use.
    //
    //At least one of V4 and V6 shall be set to "1", and both may be set to "1".
    mask: u8,                   //M
    seid: Vec<u8>,              //M 8bytes
    ipv4_addr: Option<Vec<u8>>, //C 4bytes
    ipv6_addr: Option<Vec<u8>>, //C 16bytes
}

impl FSEID {
    pub fn decode(mut buf: &mut [u8], len: u16) -> Result<FSEID, PFCPError> {
        let mut element = FSEID {
            ie_type: ie_type::F_SEID,
            ie_len: len,
            ..Default::default()
        };
        element.mask = buf[0];
        element.seid = buf[1..=8].to_vec();
        buf = &mut buf[9..];
        match element.mask & 0b0000_0011 {
            0b0000_0010 => {
                element.ipv4_addr = Some(buf[0..=3].to_vec());
            }
            0b0000_0001 => {
                element.ipv6_addr = Some(buf[0..=15].to_vec());
            }
            0b0000_0011 => {
                element.ipv4_addr = Some(buf[0..=3].to_vec());
                element.ipv6_addr = Some(buf[4..=19].to_vec());
            }
            _ => (),
        }
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.mask);
        element_vec.append(&mut self.seid);
        if let Some(mut ipv4_addr) = self.ipv4_addr {
            element_vec.append(&mut ipv4_addr);
        }
        if let Some(mut ipv6_addr) = self.ipv6_addr {
            element_vec.append(&mut ipv6_addr);
        }
        element_vec
    }
}
