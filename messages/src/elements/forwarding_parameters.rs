use super::super::PFCPError;
use super::ie_type;

#[derive(Debug, Default)]
pub struct ForwardingParameters {
    ie_type: u16,
    ie_len: u16,

    //This IE shall identify the destination interface of the outgoing packet.
    destination_interface: DestincationInterface, //M

    //When present, this IE shall identify the Network instance towards which to send the outgoing
    //packet.
    network_interface: NetworkInterface, //O

    //This IE shall be present if the UP function is required to mark the IP header with the DSCP
    //marking as defined by IETF RFC 2474 [22]. When present for EPC, it shall contain the value of
    //the DSCP in the TOS/Traffic Class field set based on the QCI, and optionally the ARP priority
    //level, of the associated EPS bearer, as described in clause 5.10 of 3GPP TS 23.214 [2]. When
    //present for 5GC, it shall contain the value of the DSCP in the TOS/Traffic Class field set
    //based on the 5QI, the Priority Level (if explicitly signalled), and optionally the ARP
    //priority level, of the associated QoS flow, as described in clause 5.8.2.7 of
    //3GPP TS 23.501 [28].
    transport_level_marking: TransportLevelMarking, //O

    //This IE may be present to indicate the 3GPP interface type of the destination interface, if
    //required by functionalities in the UP Function, e.g. for performance measurements.
    _3gpp_interface_type: _3GPPInterfaceType, //O
}

impl ForwardingParameters {
    pub fn decode(buf: &[u8], len: u16) -> Result<ForwardingParameters, PFCPError> {
        let mut element = ForwardingParameters {
            ie_type: ie_type::FORWARDING_PARAMETERS,
            ie_len: len,
            ..Default::default()
        };
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                ie_type::DESTINCATION_INTERFACE => {
                    element.destination_interface = DestincationInterface::decode(buf, elen)?;
                }
                ie_type::NETWORK_INTERFACE => {
                    element.network_interface = Some(NetworkInterface::decode(buf, elen)?);
                }
                ie_type::TRANSPORT_LEVEL_MARKING => {
                    element.transport_level_marking =
                        Some(TransportLevelMarking::decode(buf, elen)?);
                }
                ie_type::_3GPP_INTERFACE_TYPE => {
                    element._3gpp_interface_type = Some(_3GPPInterfaceType::decode(buf, elen)?);
                }
            }
            buf = &mut buf[elen.into()..];
        }
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.destination_interface.encode());
        if let Some(f_teid) = self.f_teid {
            element_vec.append(&mut f_teid.encode());
        }
        if let Some(network_interface) = self.network_interface {
            element_vec.append(&mut network_interface.encode());
        }
        if let Some(transport_level_marking) = self.transport_level_marking {
            element_vec.append(&mut transport_level_marking.encode());
        }
        if let Some(_3gpp_interface_type) = self._3gpp_interface_type {
            element_vec.append(&mut _3gpp_interface_type);
        }
        element_vec
    }
}
