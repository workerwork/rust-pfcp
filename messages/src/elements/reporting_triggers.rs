use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//                      Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 37 (decimal)
// 3 to 4	    Length = n
// 5	LIUSA	DROTH	STOPT	START	QUHTI	TIMTH	VOLTH	PERIO
// 6	QUVTI	IPMJL	EVEQU	EVETH	MACAR	ENVCL	TIMQU	VOLQU
// 7	Spare	Spare	Spare	Spare	Spare	Spare	Spare	REEMR
// 8 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ReportingTriggers {
    ie_type: u16,
    ie_len: u16,
    //Octet 5
    periodic_reporting: bool,
    volume_threshold: bool,
    time_threshold: bool,
    quota_holding_time: bool,
    start_of_traffic: bool,
    stop_of_traffic: bool,
    dropper_dl_traffic_threshold: bool,
    linked_usage_reporting: bool,
    //Octet 6
    volume_quota: bool,
    time_quota: bool,
    envelope_closure: bool,
    mac_addresses_reporting: bool,
    event_threshold: bool,
    event_quota: bool,
    ip_multicast_join_or_leave: bool,
    quota_validity_time: bool,
    //Octet 7
    report_the_end_marker_reception: bool,
}

impl ReportingTriggers {
    pub fn decode(buf: &[u8], len: u16) -> Result<ReportingTriggers, PFCPError> {
        let mut element = ReportingTriggers {
            ie_type: ie_type::REPORTING_TRIGGERS,
            ie_len: len,
            ..Default::default()
        };
        let mask = buf[0];
        if mask & 0b0000_0001 != 0 {
            element.periodic_reporting = true;
        }
        if mask & 0b0000_0010 != 0 {
            element.volume_threshold = true;
        }
        if mask & 0b0000_0100 != 0 {
            element.time_threshold = true;
        }
        if mask & 0b0000_1000 != 0 {
            element.quota_holding_time = true;
        }
        if mask & 0b0001_0000 != 0 {
            element.start_of_traffic = true;
        }
        if mask & 0b0010_0000 != 0 {
            element.stop_of_traffic = true;
        }
        if mask & 0b0100_0000 != 0 {
            element.dropper_dl_traffic_threshold = true;
        }
        if mask & 0b1000_0000 != 0 {
            element.linked_usage_reporting = true;
        }

        let mask = buf[1];
        if mask & 0b0000_0001 != 0 {
            element.volume_quota = true;
        }
        if mask & 0b0000_0010 != 0 {
            element.time_quota = true;
        }
        if mask & 0b0000_0100 != 0 {
            element.envelope_closure = true;
        }
        if mask & 0b0000_1000 != 0 {
            element.mac_addresses_reporting = true;
        }
        if mask & 0b0001_0000 != 0 {
            element.event_threshold = true;
        }
        if mask & 0b0010_0000 != 0 {
            element.event_quota = true;
        }
        if mask & 0b0100_0000 != 0 {
            element.ip_multicast_join_or_leave = true;
        }
        if mask & 0b1000_0000 != 0 {
            element.quota_validity_time = true;
        }

        let mask = buf[2];
        if mask & 0b0000_0001 != 0 {
            element.report_the_end_marker_reception = true;
        }
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());

        let mut mask: u8 = 0b0000_0000;
        if self.periodic_reporting {
            mask |= 0b0000_0001;
        }
        if self.volume_threshold {
            mask |= 0b0000_0010;
        }
        if self.time_threshold {
            mask |= 0b0000_0100;
        }
        if self.quota_holding_time {
            mask |= 0b0000_1000;
        }
        if self.start_of_traffic {
            mask |= 0b0001_0000;
        }
        if self.stop_of_traffic {
            mask |= 0b0010_0000;
        }
        if self.dropper_dl_traffic_threshold {
            mask |= 0b0100_0000;
        }
        if self.linked_usage_reporting {
            mask |= 0b1000_0000;
        }
        element_vec.push(self.mask);

        let mut mask: u8 = 0b0000_0000;
        if self.volume_quota {
            mask |= 0b0000_0001;
        }
        if self.time_quota {
            mask |= 0b0000_0010;
        }
        if self.envelope_closure {
            mask |= 0b0000_0100;
        }
        if self.mac_addresses_reporting {
            mask |= 0b0000_1000;
        }
        if self.event_threshold {
            mask |= 0b0001_0000;
        }
        if self.event_quota {
            mask |= 0b0010_0000;
        }
        if self.ip_multicast_join_or_leave {
            }
            }
        }
        if self.quota_validity_time {
            mask |= 0b1000_0000;
        }
        element_vec.push(self.mask);

        let mut mask: u8 = 0b0000_0000;
        if self.report_the_end_marker_reception {
            mask |= 0b0000_0001;
        }
        element_vec.push(self.mask);
        element_vec
    }
}
