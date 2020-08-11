use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
// 	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 62 (decimal)
// 3 to 4	    Length = n
// 5	Spare	Spare	Spare	Spare	Spare	EVENT	VOLUM	DURAT
// 6 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct MeasurementMethod {
    ie_type: u16,
    ie_len: u16,
    event: u8,
    volum: u8,
    durat: u8,
}

impl MeasurementMethod {
    pub fn decode(buf: &mut [u8], len: u16) -> Result<MeasurementMethod, PFCPError> {
        let mut element = MeasurementMethod {
            ie_type: ie_type::MEASUREMENT_METHOD,
            ie_len: len,
            ..Default::default()
        };
        element.event = buf[0] & 0b0000_0100;
        element.volum = buf[0] & 0b0000_0010;
        element.durat = buf[0] & 0b0000_0001;
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.event | self.volum | self.durat);
        element_vec
    }
}
