use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 21 (decimal)
// 3 to 4	    Length = n
// 5 	        Spare	CHID	CH	V6	V4
// 6 to 9 	            TEID
// m to (m+3) 	    IPv4 address
// p to (p+15) 	    IPv6 address
// q 	            CHOOSE ID
// k to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct FTEID {
    ie_type: u16,
    ie_len: u16,

    //Bit 1 – V4: If this bit is set to "1" and the CH bit is not set, then the IPv4 address field
    //shall be present, otherwise the IPv4 address field shall not be present.
    //
    //Bit 2 – V6: If this bit is set to "1" and the CH bit is not set, then the IPv6 address field
    //shall be present, otherwise the IPv6 address field shall not be present.
    //
    //Bit 3 – CH (CHOOSE): If this bit is set to "1", then the TEID, IPv4 address and IPv6 address
    //fields shall not be present and the UP function shall assign an F-TEID with an IP4 or an IPv6
    //address if the V4 or V6 bit is set respectively.
    //This bit shall only be set by the CP function.
    //
    //Bit 4 – CHID (CHOOSE ID): If this bit is set to "1", then the UP function shall assign the
    //same F-TEID to the PDRs requested to be created in a PFCP Session Establishment Request or
    //PFCP Session Modification Request with the same CHOOSE ID value.
    //This bit may only be set to "1" if the CH bit it set to "1".
    //This bit shall only be set by the CP function.
    //
    //Bit 5 to 8: Spare, for future use and set to 0.
    mask: u8, //M

    teid: Option<Vec<u8>>,      //C 4bytes
    ipv4_addr: Option<Vec<u8>>, //C 4bytes
    ipv6_addr: Option<Vec<u8>>, //C 16bytes
    choose_id: Option<u8>,      //C
}

impl FTEID {
    pub fn decode(buf: &[u8], len: u16) -> Result<FTEID, PFCPError> {
        let mut element = FTEID {
            ie_type: ie_type::F_TEID,
            ie_len: len,
            ..Default::default()
        };
        element.mask = buf[0];
        buf = &mut buf[1..];
        if element.mask & 0b0000_0100 != 0 {
            element.choose_id = Some(buf[0]);
            buf = &mut buf[1..];
        } else {
            element.teid = Some(buf[0..=3].to_vec());
            buf = &mut buf[4..];
            if element.mask & 0b0000_0001 != 0 {
                element.ipv4_addr = Some(buf[0..=7].to_vec());
                buf = &mut buf[8..];
            }
            if element.mask & 0b0000_0010 != 0 {
                element.ipv6_addr = Some(buf[0..=15].to_vec());
                buf = &mut buf[16..];
            }
            if element.mask & 0b0000_0100 != 0 {
                element.choose_id = Some(buf[0]);
            }
        }
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.mask);
        if let Some(teid) = self.teid {
            element_vec.append(&mut teid);
        }
        if let Some(ipv4_addr) = self.ipv4_addr {
            element_vec.append(&mut ipv4_addr);
        }
        if let Some(ipv6_addr) = self.ipv6_addr {
            element_vec.append(&mut ipv6_addr);
        }
        if let Some(choose_id) = self.choose_id {
            element_vec.push(choose_id);
        }
        element_vec
    }
}
