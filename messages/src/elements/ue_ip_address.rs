use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 93 (decimal)
// 3 to 4	    Length = n
// 5  Spare IP6PL CHV6 CHV4	IPv6D S/D V4 V6
// m to (m+3)	IPv4 address
// p to (p+15)	IPv6 address
// r	        IPv6 Prefix Delegation Bits
// s	        IPv6 Prefix Length
// k to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct UEIPAddress {
    ie_type: u16,
    ie_len: u16,

    //Bit 1 – V6: If this bit is set to "1", then the CHV6 bit shall not be set and the IPv6
    //address field shall be present in the UE IP Address, otherwise the IPv6 address field shall
    //not be present.
    //
    //Bit 2 – V4: If this bit is set to "1", then the CHV4 bit sshall not be set and the IPv4
    //address field shall be present in the UE IP Address, otherwise the IPv4 address field shall
    //not be present.
    //
    //Bit 3 – S/D: This bit is only applicable to the UE IP Address IE in the PDI IE. It shall be
    //set to "0" and ignored by the receiver in IEs other than PDI IE. In the PDI IE, if this bit
    //is set to "0", this indicates a Source IP address; if this bit is set to "1", this indicates
    //a Destination IP address.
    //
    //Bit 4 – IPv6D: This bit is only applicable to the UE IP address IE in the PDI IE and when the
    //V6 bit or CHV6 bit is set to "1". If this bit is set to "1", then the IPv6 Prefix Delegation
    //Bits field shall be present, otherwise the UP function shall consider IPv6 prefix is default
    //"/64".
    //
    //Bit 5 – CHV4 (CHOOSE IPV4): If this bit is set to "1", then the V4 bit shall not be set, the
    //IPv4 address shall not be present and the UP function shall assign an IPv4 address. This bit
    //shall only be set by the CP function.
    //
    //Bit 6 – CHV6 (CHOOSE IPV6): If this bit is set to "1", then the V6 bit shall not be set, the
    //IPv6 address shall not be present and the UP function shall assign an IPv6 address. This bit
    //shall only be set by the CP function.
    //
    //Bit 7 – IP6PL (IPv6 Prefix Length): this bit is only applicable when the V6 bit or CHV6 bit
    //is set to "1" and the "IPv6D" bit is set to "0", for an IPv6 prefix other than default /64.
    //If this bit is set to "1", then the IPv6 Prefix Length field shall be present.
    //
    //Bit 8 Spare, for future use and set to "0".
    mask: u8, //M

    ipv4_addr: Option<Vec<u8>>,         //C 4bytes
    ipv6_addr: Option<Vec<u8>>,         //C 16bytes
    iPv6_prefix_delegation: Option<u8>, //C
    ipv6_prefix_length: Option<u8>,     //C
}

impl UEIPAddress {
    pub fn decode(mut buf: &mut[u8], len: u16) -> Result<UEIPAddress, PFCPError> {
        let mut element = UEIPAddress {
            ie_type: ie_type::UE_IP_ADDRESS,
            ie_len: len,
            ..Default::default()
        };
        element.mask = buf[0];
        buf = &mut buf[1..];
        if element.mask & 0b0000_0010 != 0 {
            element.ipv4_addr = Some(buf[0..=3].to_vec());
            buf = &mut buf[4..];
        }
        if element.mask & 0b0010_0001 != 0 {
            element.ipv6_addr = Some(buf[0..=15].to_vec());
            buf = &mut buf[16..];
        }
        if element.mask & 0b0000_1000 != 0 {
            element.iPv6_prefix_delegation = Some(buf[0]);
            buf = &mut buf[1..];
        }
        if element.mask & 0b0100_0000 != 0 {
            element.ipv6_prefix_length = Some(buf[0]);
        }
        Ok(element)
    }

    pub fn encode(self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.mask);
        if let Some(mut ipv4_addr) = self.ipv4_addr {
            element_vec.append(&mut ipv4_addr);
        }
        if let Some(mut ipv6_addr) = self.ipv6_addr {
            element_vec.append(&mut ipv6_addr);
        }
        if let Some(iPv6_prefix_delegation) = self.iPv6_prefix_delegation {
            element_vec.push(iPv6_prefix_delegation)
        }
        if let Some(ipv6_prefix_length) = self.ipv6_prefix_length {
            element_vec.push(ipv6_prefix_length)
        }
        element_vec
    }
}
