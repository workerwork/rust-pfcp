use super::super::PFCPError;
use super::ie_type;

use super::source_interface::SourceInterface;
use super::f_teid::FTEID;
use super::network_instance::NetworkInstance;
use super::ue_ip_address::UEIPAddress;
use super::sdf_filter::SDFFilter;
use super::qfi::QFI;
use super::_3gpp_interface_type::_3GPPInterfaceType;

#[derive(Debug, Default)]
pub struct PDI {
    ie_type: u16,
    ie_len: u16,

    //This IE shall identify the source interface of the incoming packet.
    source_interface: SourceInterface, //M

    //This IE shall not be present if Traffic Endpoint ID is present.
    //If present, this IE shall identify the local F-TEID to match for an incoming packet.
    //The CP function shall set the CHOOSE (CH) bit to 1 if the CP function requests
    //the UP function to assign a local F-TEID to the PDR.
    f_teid: Option<FTEID>, //O

    //This IE shall not be present if Traffic Endpoint ID is present. It shall be present if the CP
    //function requests the UP function to allocate a UE IP address/prefix and the Traffic Endpoint
    //ID is not present.
    //If present, this IE shall identify the Network instance to match for the incoming packet.
    network_instance: Option<NetworkInstance>, //O

    //This IE shall not be present if Traffic Endpoint ID is present.
    //If present, this IE shall identify the source or destination IP address to match for the
    //incoming packet.
    //The CP function shall set the CHOOSE IPV4 (CHV4) and/or the CHOOSE IPV6 (CHV6) bits to 1 if
    //the UP function supports the allocation of UE IP address/ prefix and the CP function requests
    //the UP function to assign a UE IP address/prefix to the PDR.
    //In the 5GC, several IEs with the same IE type may be present to represent multiple UE IP
    //addresses, if the UPF indicated support of the IP6PL feature.
    ue_ip_address: Option<UEIPAddress>, //O

    //If present, this IE shall identify the SDF filter to match for the incoming packet. Several
    //IEs with the same IE type may be present to provision a list of SDF Filters. The full set of
    //applicable SDF filters, if any, shall be provided during the creation or the modification of
    //the PDI.
    sdf_filter: Option<SDFFilter>, //O

    //This IE shall not be present if Traffic Endpoint ID is present and the QFI(s) are included in
    //the Traffic Endpoint.
    //If present, this IE shall identify the QoS Flow Identifier to match for the incoming packet.
    //Several IEs with the same IE type may be present to provision a list of QFIs. When present,
    //the full set of applicable QFIs shall be provided during the creation or the modification of
    //the PDI.
    qfi: Option<QFI>, //O

    //This IE may be present to indicate the 3GPP interface type of the source interface, if
    //required by functionalities in the UP Function, e.g. for performance measurements.
    _3gpp_interface_type: Option<_3GPPInterfaceType>, //O
}

impl PDI {
    pub fn decode(buf: &[u8], len: u16) -> Result<PDI, PFCPError> {
        let mut element = PDI {
            ie_type: ie_type::PDI,
            ie_len: len,
            ..Default::default()
        };
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                ie_type::SOURCE_INTERFACE => {
                    element.source_interface = SourceInterface::decode(buf, elen)?;
                }
                ie_type::F_TEID => {
                    element.f_teid = Some(FTEID::decode(buf, elen)?);
                }
                ie_type::NETWORK_INSTANCE => {
                    element.network_instance = Some(NetworkInstance::decode(buf, elen)?);
                }
                ie_type::UE_IP_ADDRESS => {
                    element.ue_ip_address = Some(UEIPAddress::decode(buf, elen)?);
                }
                ie_type::SDF_FILTER => {
                    element.sdf_filter = Some(SDFFilter::decode(buf, elen)?);
                }
                ie_type::QFI => {
                    element.qfi = Some(QFI::decode(buf, elen)?);
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
        element_vec.append(&mut self.source_interface.encode());
        if let Some(f_teid) = self.f_teid {
            element_vec.append(&mut f_teid.encode());
        }
        if let Some(network_instance) = self.network_instance {
            element_vec.append(&mut network_instance.encode());
        }
        if let Some(ue_ip_address) = self.ue_ip_address {
            element_vec.append(&mut ue_ip_address.encode());
        }
        if let Some(sdf_filter) = self.sdf_filter {
            element_vec.append(&mut sdf_filter.encode());
        }
        if let Some(qfi) = self.qfi {
            element_vec.append(&mut qfi.encode());
        }
        if let Some(_3gpp_interface_type) = self._3gpp_interface_type {
            element_vec.append(&mut _3gpp_interface_type.encode());
        }
        element_vec
    }
}
