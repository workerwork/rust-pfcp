use super::super::PFCPError;
use super::ie_type;

use super::measurement_method::MeasurementMethod;
use super::measurement_period::MeasurementPeriod;
use super::reporting_triggers::ReportingTriggers;
use super::urr_id::URRID;

#[derive(Debug, Default)]
pub struct CreateURR {
    ie_type: u16,
    ie_len: u16,

    //This IE shall uniquely identify the URR among all the URRs configured for this PFCP session.
    urr_id: URRID, //M

    //This IE shall indicate the method for measuring the network resources usage, i.e. whether the
    //data volume, duration (i.e. time), combined volume/duration, or event shall be measured.
    measurement_method: MeasurementMethod, //M

    //This IE shall indicate the trigger(s) for reporting network resources usage to the CP
    //function, e.g. periodic reporting or reporting upon reaching a threshold, or envelope
    //closure, or when an SMF instructs an UPF to report the reception of the End Marker packet
    //from the old I-UPF during a Service Request procedure (see clauses 4.2.3.2 and 4.23.4.3 in
    //3GPP TS 23.502 [29]).
    reporting_triggers: ReportingTriggers, //M

    //This IE shall be present if periodic reporting is required. When present, it shall indicate
    //the period for generating and reporting usage reports.
    measurement_period: Option<MeasurementPeriod>, //C

                                                   //
}

pub type _CreateURR = Result<CreateURR, PFCPError>;

impl CreateURR {
    pub fn decode(mut buf: &mut [u8], len: u16) -> _CreateURR {
        let mut element = CreateURR {
            ie_type: ie_type::CREATE_URR,
            ie_len: len,
            ..Default::default()
        };
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                ie_type::URR_ID => {
                    element.urr_id = URRID::decode(buf, elen)?;
                }
                ie_type::MEASUREMENT_METHOD => {
                    element.measurement_method = MeasurementMethod::decode(buf, elen)?;
                }
                ie_type::REPORTING_TRIGGERS => {
                    element.reporting_triggers = ReportingTriggers::decode(buf, elen)?;
                }
                ie_type::MEASUREMENT_PERIOD => {
                    element.measurement_period = Some(MeasurementPeriod::decode(buf, elen)?);
                }
                _ => (),
            }
            buf = &mut buf[elen.into()..];
        }
        Ok(element)
    }

    pub fn encode(self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.urr_id.encode());
        element_vec.append(&mut self.measurement_method.encode());
        element_vec.append(&mut self.reporting_triggers.encode());
        /*if let Some(measurement_period) = self.measurement_period {
            element_vec.append(&mut measurement_period.encode());
        }*/
        self.measurement_period
            .map(|measurement_period| element_vec.append(&mut measurement_period.encode()));
        element_vec
    }
}
