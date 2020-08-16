use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 44 (decimal)
// 3 to 4	    Length = n
// 5	DFRT	IPMD	IPMA	DUPL	NOCP	BUFF	FORW	DROP
// 6	        Spare	DDPN	BDPN	EDRT
// 7 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ApplyAction {
    ie_type: u16,
    ie_len: u16,

    //Bit 1 – DROP (Drop): when set to "1", this indicates a request to drop the packets.
    //
    //Bit 2 – FORW (Forward): when set to "1", this indicates a request to forward the packets.
    //
    //Bit 3 – BUFF (Buffer): when set to "1", this indicates a request to buffer the packets.
    //
    //Bit 4 – NOCP (Notify the CP function): when set to "1", this indicates a request to notify
    //the CP function about the arrival of a first downlink packet being buffered.
    //
    //Bit 5 – DUPL (Duplicate): when set to "1", this indicates a request to duplicate the packets.
    //
    //Bit 6 – IPMA (IP Multicast Accept): when set to "1", this indicates a request to accept UE
    //requests to join an IP multicast group.
    //
    //Bit 7 – IPMD (IP Multicast Deny): when set to "1", this indicates a request to deny UE
    //requests to join an IP multicast group.
    //
    //Bit 8 – DFRT (Duplicate for Redundant Transmission): when set to "1", this indicates a
    //request to duplicate the packets for redundant transmission.
    action: u8, //M

    //Bit 1 – EDRT (Eliminate Duplicate Packets for Redundant Transmission): when set to "1", this
    //indicates a request to eliminate duplicate packets used for redundant transmission.
    //
    //Bit 2 – BDPN (Buffered Downlink Packet Notification): when set to "1", this indicates a
    //request to notify the CP function about the first buffered DL packet for downlink data
    //delivery status notification.
    //
    //Bit 3 – DDPN (Discarded Downlink Packet Notification): when set to "1", this indicates a
    //request to notify the CP function about the first discarded DL packet for downlink data
    //delivery status notification if the DL Buffering Duration or DL Buffering Suggested Packet
    //Count is exceeded.
    //
    //Bit 4 to 8 – Spare, for future use and seto to "0".
    mask: u8, //M
}

pub type _ApplyAction = Result<ApplyAction, PFCPError>;

impl ApplyAction {
    pub fn decode(buf: &[u8], len: u16) -> _ApplyAction {
        let mut element = ApplyAction {
            ie_type: ie_type::APPLY_ACTION,
            ie_len: len,
            ..Default::default()
        };
        element.action = buf[0];
        element.mask = buf[1] & 0b0000_0111;
        Ok(element)
    }

    pub fn encode(self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.action);
        element_vec.push(self.mask);
        element_vec
    }
}
