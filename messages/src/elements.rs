//#![allow(dead_code)]

pub mod ie_type {
    //Grouped IE, extendable
    pub const CREATE_PDR: u16 = 1;
    pub const PDI: u16 = 2;
    pub const CREATE_FAR: u16 = 3;
    pub const FORWARDING_PARAMETERS: u16 = 4;
    pub const DUPLICATING_PARAMETERS: u16 = 5;
    pub const CREATE_URR: u16 = 6;
    pub const CREATE_QER: u16 = 7;
    pub const CREATED_PDR: u16 = 8;
    pub const UPDATE_PDR: u16 = 9;
    pub const UPDATE_FAR: u16 = 10;
    pub const UPDATE_FORWARDING_PARAMETERS: u16 = 11;
    pub const UPDATE_BAR: u16 = 12;
    pub const UPDATE_URR: u16 = 13;
    pub const UPDATE_QER: u16 = 14;
    pub const REMOVE_PDR: u16 = 15;
    pub const REMOVE_FAR: u16 = 16;
    pub const REMOVE_URR: u16 = 17;
    pub const REMOVE_QER: u16 = 18;

    pub const CAUSE: u16 = 19;
    pub const SOURCE_INTERFACE: u16 = 20;
    pub const F_TEID: u16 = 21;
    pub const NETWORK_INSTANCE: u16 = 22;
    pub const SDF_FILTER: u16 = 23;
    pub const APPLICATION_ID: u16 = 24;
    pub const GATE_STATUS: u16 = 25;
    pub const MBR: u16 = 26;
    pub const GBR: u16 = 27;
    pub const QER_CORRELATION_ID: u16 = 28;
    pub const PRECEDENCE: u16 = 29;
    pub const TRANSPORT_LEVEL_MARKING: u16 = 30;
    pub const VOLUME_THRESHOLD: u16 = 31;
    pub const TIME_THRESHOLD: u16 = 32;
    pub const MONITORING_TIME: u16 = 33;
    pub const SUBSEQUENT_VOLUME_THRESHOLD: u16 = 34;
    pub const SUBSEQUENT_TIME_THRESHOLD: u16 = 35;
    pub const INACTIVITY_DETECTION_TIME: u16 = 36;
    pub const REPORTING_TRIGGERS: u16 = 37;
    pub const REDIRECT_INFORMATION: u16 = 38;
    pub const REPORT_TYPE: u16 = 39;
    pub const OFFENDING_IE: u16 = 40;
    pub const FORWARDING_POLICY: u16 = 41;
    pub const DESTINATION_INTERFACE: u16 = 42;
    pub const UP_FUNCTION_FEATURES: u16 = 43;
    pub const APPLY_ACTION: u16 = 44;
    pub const DOWNLINK_DATA_SERVICE_INFORMATION: u16 = 45;
    pub const DOWNLINK_DATA_NOTIFICATION_DELAY: u16 = 46;
    pub const DL_BUFFERING_DURATION: u16 = 47;
    pub const DL_BUFFERING_SUGGESTED_PACKET_COUNT: u16 = 48;
    pub const PFCP_SM_REQ_FLAGS: u16 = 49;
    pub const PFCP_SR_RSP_FLAGS: u16 = 50;
    pub const SEQUENCE_NUMBER: u16 = 52;
    pub const METRIC: u16 = 53;
    pub const TIMER: u16 = 55;
    pub const PDR_ID: u16 = 56;
    pub const F_SEID: u16 = 57;
    pub const NODE_ID: u16 = 60;
    pub const PFD_CONTENTS: u16 = 61;
    pub const MEASUREMENT_METHOD: u16 = 62;
    pub const USAGE_REPORT_TRIGGER: u16 = 63;
    pub const MEASUREMENT_PERIOD: u16 = 64;
    pub const FULLY_QUALIFIED_PDN_CONNECTION_SET_IDENTIFIER: u16 = 65;
    pub const VOLUME_MEASUREMENT: u16 = 66;
    pub const DURATION_MEASUREMENT: u16 = 67;
    pub const TIME_OF_FIRST_PACKET: u16 = 69;
    pub const TIME_OF_LAST_PACKET: u16 = 70;
    pub const QUOTA_HOLDING_TIME: u16 = 71;
    pub const DROPPED_DL_TRAFFIC_THRESHOLD: u16 = 72;
    pub const VOLUME_QUOTA: u16 = 73;
    pub const TIME_QUOTA: u16 = 74;
    pub const START_TIME: u16 = 75;
    pub const END_TIME: u16 = 76;
    pub const URR_ID: u16 = 81;
    pub const LINKED_URR_ID: u16 = 82;
    pub const OUTER_HEADER_CREATION: u16 = 84;
    pub const BAR_ID: u16 = 88;
    pub const CP_FUNCTION_FEATURES: u16 = 89;
    pub const USAGE_INFORMATION: u16 = 90;
    pub const APPLICATION_INSTANCE_ID: u16 = 91;
    pub const FLOW_INFORMATION: u16 = 92;
    pub const UE_IP_ADDRESS: u16 = 93;
    pub const PACKET_RATE: u16 = 94;
    pub const OUTER_HEADER_REMOVAL: u16 = 95;
    pub const RECOVERY_TIME_STAMP: u16 = 96;
    pub const DL_FLOW_LEVEL_MARKING: u16 = 97;
    pub const HEADER_ENRICHMENT: u16 = 98;
    pub const MEASUREMENT_INFORMATION: u16 = 100;
    pub const NODE_REPORT_TYPE: u16 = 101;
    pub const REMOTE_GTPU_PEER: u16 = 103;
    pub const UR_SEQN: u16 = 104;
    pub const ACTIVATE_PREDEFINED_RULES: u16 = 106;
    pub const DEACTIVATE_PREDEFINED_RULES: u16 = 107;
    pub const FAR_ID: u16 = 108;
    pub const QER_ID: u16 = 109;
    pub const OCI_FLAGS: u16 = 110;
    pub const PFCP_ASSOCIATION_RELEASE_REQUEST: u16 = 111;
    pub const GRACEFUL_RELEASE_PERIOD: u16 = 112;
    pub const PDN_TYPE: u16 = 113;
    pub const FAILED_RULE_ID: u16 = 114;
    pub const TIME_QUOTA_MECHANISM: u16 = 115;
    pub const USER_PLANE_IP_RESOURCE_INFORMATION: u16 = 116;
    pub const USER_PLANE_INACTIVITY_TIMER: u16 = 117;
    pub const MULTIPLIER: u16 = 119;
    pub const AGGREGATED_URR_ID: u16 = 120;
    pub const SUBSEQUENT_VOLUME_QUOTA: u16 = 121;
    pub const SUBSEQUENT_TIME_QUOTA: u16 = 122;
    pub const RQI: u16 = 123;
    pub const QFI: u16 = 124;
    pub const QUERY_URR_REFERENCE: u16 = 125;
    pub const ADDITIONAL_USAGE_REPORTS_INFORMATION: u16 = 126;
    pub const TRAFFIC_ENDPOINT_ID: u16 = 131;
    pub const MAC_ADDRESS: u16 = 133;
    pub const C_TAG: u16 = 134;
    pub const S_TAG: u16 = 135;
    pub const ETHER_TYPE: u16 = 136;
    pub const PROXYING: u16 = 137;
    pub const ETHERNET_FILTER_ID: u16 = 138;
    pub const ETHERNET_FILTER_PROPERTIES: u16 = 139;
    pub const SUGGESTED_BUFFERING_PACKETS_COUNT: u16 = 140;
    pub const USER_ID: u16 = 141;
    pub const ETHERNET_PDU_SESSION_INFORMATION: u16 = 142;
    pub const MAC_ADDRESSES_DETECTED: u16 = 144;
    pub const MAC_ADDRESSES_REMOVED: u16 = 145;
    pub const ETHERNET_INACTIVITY_TIMER: u16 = 146;
    pub const EVENT_QUOTA: u16 = 148;
    pub const EVENT_THRESHOLD: u16 = 149;
    pub const SUBSEQUENT_EVENT_QUOTA: u16 = 150;
    pub const SUBSEQUENT_EVENT_THRESHOLD: u16 = 151;
    pub const TRACE_INFORMATION: u16 = 152;
    pub const FRAMED_ROUTE: u16 = 153;
    pub const FRAMED_ROUTING: u16 = 154;
    pub const FRAMED_IPV6_ROUTE: u16 = 155;
    pub const EVENT_TIME_STAMP: u16 = 156;
    pub const AVERAGING_WINDOW: u16 = 157;
    pub const PAGING_POLICY_INDICATOR: u16 = 158;
    pub const APN_DNN: u16 = 159;
    pub const _3GPP_INTERFACE_TYPE: u16 = 160;
    pub const PFCP_SR_REQ_FLAGS: u16 = 161;
    pub const PFCP_AU_REQ_FLAGS: u16 = 162;
    pub const ACTIVATION_TIME: u16 = 163;
    pub const DEACTIVATION_TIME: u16 = 164;
    pub const MAR_ID: u16 = 170;
    pub const STEERING_FUNCTIONALITY: u16 = 171;
    pub const STEERING_MODE: u16 = 172;
    pub const WEIGHT: u16 = 173;
    pub const PRIORITY: u16 = 174;
    pub const UE_IP_ADDRESS_POOL_IDENTITY: u16 = 177;
    pub const ALTERNATIVE_SMF_IP_ADDRESS: u16 = 178;
    pub const PACKET_REPLICATION_AND_DETECTION_CARRY_ON_INFORMATION: u16 = 179;
    pub const SMF_SET_ID: u16 = 180;
    pub const QUOTA_VALIDITY_TIME: u16 = 181;
}

pub mod _3gpp_interface_type;
pub mod apply_action;
pub mod cause;
pub mod create_far;
pub mod create_pdr;
pub mod create_qer;
pub mod create_urr;
pub mod destination_interface;
pub mod f_seid;
pub mod f_teid;
pub mod far_id;
pub mod forwarding_parameters;
pub mod gate_status;
pub mod mbr;
pub mod measurement_method;
pub mod measurement_period;
pub mod network_instance;
pub mod node_id;
pub mod outer_header_creation;
pub mod outer_header_removal;
pub mod pdi;
pub mod pdn_type;
pub mod pdr_id;
pub mod precedence;
pub mod qer_id;
pub mod qfi;
pub mod recovery_time_stamp;
pub mod reporting_triggers;
pub mod sdf_filter;
pub mod source_interface;
pub mod transport_level_marking;
pub mod ue_ip_address;
pub mod urr_id;
