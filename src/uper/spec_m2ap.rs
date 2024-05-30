pub const ID_ACTIVE_MBMS_SESSION_LIST: u16 = 42;

pub const ID_ADDITIONAL_CONFIG_PARAMETERS: u16 = 54;

pub const ID_ALTERNATIVE_TNL_INFORMATION: u16 = 38;

pub const ID_CAUSE: u16 = 9;

pub const ID_COMMON_SUBFRAME_ALLOCATION_PERIOD: u16 = 24;

pub const ID_CRITICALITY_DIAGNOSTICS: u16 = 8;

pub const ID_ENB_MBMS_CONFIGURATION_DATA_CONFIG_UPDATE_ITEM: u16 = 27;

pub const ID_ENB_MBMS_CONFIGURATION_DATA_ITEM: u16 = 16;

pub const ID_ENB_MBMS_CONFIGURATION_DATA_LIST: u16 = 15;

pub const ID_ENB_MBMS_CONFIGURATION_DATA_LIST_CONFIG_UPDATE: u16 = 26;

pub const ID_ENB_MBMS_M2AP_ID: u16 = 1;

pub const ID_EN_BNAME: u16 = 14;

pub const ID_GLOBAL_ENB_ID: u16 = 13;

pub const ID_GLOBAL_MCE_ID: u16 = 17;

pub const ID_MBMS_COUNTING_REQUEST_SESSION: u16 = 32;

pub const ID_MBMS_COUNTING_REQUEST_SESSION_ITEM: u16 = 33;

pub const ID_MBMS_COUNTING_RESULT_ITEM: u16 = 35;

pub const ID_MBMS_COUNTING_RESULT_LIST: u16 = 34;

pub const ID_MBMS_SERVICE_AREA: u16 = 6;

pub const ID_MBMS_SERVICE_ASSOCIATED_LOGICAL_M2_CONNECTION_ITEM: u16 = 28;

pub const ID_MBMS_SERVICE_ASSOCIATED_LOGICAL_M2_CONNECTION_LIST_RES_ACK: u16 = 31;

pub const ID_MBMS_SESSION_ID: u16 = 3;

pub const ID_MBMS_SUSPENSION_NOTIFICATION_ITEM: u16 = 44;

pub const ID_MBMS_SUSPENSION_NOTIFICATION_LIST: u16 = 43;

pub const ID_MBSFN_AREA_CONFIGURATION_LIST: u16 = 10;

pub const ID_MBSFN_AREA_ID: u16 = 29;

pub const ID_MBSFN_SUBFRAME_CONFIGURATION_ITEM: u16 = 23;

pub const ID_MBSFN_SUBFRAME_CONFIGURATION_LIST: u16 = 22;

pub const ID_MCCH_UPDATE_TIME: u16 = 25;

pub const ID_MCC_HRELATED_BCCH_CONFIG_PER_MBSFN_AREA: u16 = 19;

pub const ID_MCC_HRELATED_BCCH_CONFIG_PER_MBSFN_AREA_ITEM: u16 = 20;

pub const ID_MCC_HRELATED_BCCH_EXT_CONFIG_PER_MBSFN_AREA: u16 = 52;

pub const ID_MCC_HRELATED_BCCH_EXT_CONFIG_PER_MBSFN_AREA_ITEM: u16 = 51;

pub const ID_MCE_MBMS_M2AP_ID: u16 = 0;

pub const ID_MC_ENAME: u16 = 18;

pub const ID_MCH_SCHEDULING_PERIOD_EXTENDED: u16 = 37;

pub const ID_MCH_SCHEDULING_PERIOD_EXTENDED2: u16 = 48;

pub const ID_MODIFICATION_PERIOD_EXTENDED: u16 = 46;

pub const ID_MODULATION_CODING_SCHEME2: u16 = 36;

pub const ID_OVERLOAD_STATUS_PER_PMCH_LIST: u16 = 39;

pub const ID_PMCH_CONFIGURATION_ITEM: u16 = 12;

pub const ID_PMCH_CONFIGURATION_LIST: u16 = 11;

pub const ID_PMCH_OVERLOAD_STATUS: u16 = 41;

pub const ID_REPETITION_PERIOD_EXTENDED: u16 = 47;

pub const ID_RESET_TYPE: u16 = 30;

pub const ID_SC_PTM_INFORMATION: u16 = 45;

pub const ID_SUBCARRIER_SPACING_MBMS: u16 = 49;

pub const ID_SUBFRAME_ALLOCATION_EXTENDED: u16 = 50;

pub const ID_SUBFRAME_ALLOCATION_FURTHER_EXTENSION: u16 = 53;

pub const ID_TMGI: u16 = 2;

pub const ID_TNL_INFORMATION: u16 = 7;

pub const ID_TIME_TO_WAIT: u16 = 21;

pub const ID_E_NB_CONFIGURATION_UPDATE: u8 = 6;

pub const ID_ERROR_INDICATION: u8 = 3;

pub const ID_M2_SETUP: u8 = 5;

pub const ID_M_CE_CONFIGURATION_UPDATE: u8 = 7;

pub const ID_MBMS_OVERLOAD_NOTIFICATION: u8 = 12;

pub const ID_MBMS_SCHEDULING_INFORMATION: u8 = 2;

pub const ID_MBMS_SERVICE_COUNTING: u8 = 10;

pub const ID_MBMS_SERVICE_COUNTING_RESULTS_REPORT: u8 = 11;

pub const ID_PRIVATE_MESSAGE: u8 = 8;

pub const ID_RESET: u8 = 4;

pub const ID_SESSION_START: u8 = 0;

pub const ID_SESSION_STOP: u8 = 1;

pub const ID_SESSION_UPDATE: u8 = 9;

pub const MAX_NR_OF_INDIVIDUAL_M2_CONNECTIONS_TO_RESET: i64 = 256;

pub const MAX_PRIVATE_I_ES: i64 = 65535;

pub const MAX_PROTOCOL_EXTENSIONS: i64 = 65535;

pub const MAX_PROTOCOL_I_ES: i64 = 65535;

pub const MAXNOOF_CELLS: i64 = 256;

pub const MAXNOOF_CELLSFOR_MBMS: i64 = 4096;

pub const MAXNOOF_COUNTING_SERVICE: i64 = 16;

pub const MAXNOOF_MBMS_SERVICE_AREAS_PER_CELL: i64 = 256;

pub const MAXNOOF_MBSFN_ALLOCATIONS: i64 = 8;

pub const MAXNOOF_MBSF_NAREAS: i64 = 256;

pub const MAXNOOF_PMC_HSPER_MBSF_NAREA: i64 = 15;

pub const MAXNOOF_SESSIONS_PER_PMCH: i64 = 29;

pub const MAXNOOFERRORS: i64 = 256;

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "29")]
pub struct Active_MBMS_Session_List(pub Vec<Active_MBMS_Session_List_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct AdditionalConfigParameters {
    pub pmch_bandwidth: PMCH_Bandwidth,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AdditionalConfigParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1535")]
pub struct AllocatedSubframesEnd(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct AllocationAndRetentionPriority {
    pub priority_level: PriorityLevel,
    pub pre_emption_capability: Pre_emptionCapability,
    pub pre_emption_vulnerability: Pre_emptionVulnerability,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AllocationAndRetentionPriorityIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "10000000000")]
pub struct BitRate(pub u64);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = true)]
pub enum Cause {
    #[asn(key = 0, extended = false)]
    RadioNetwork(CauseRadioNetwork),
    #[asn(key = 1, extended = false)]
    Transport(CauseTransport),
    #[asn(key = 2, extended = false)]
    NAS(CauseNAS),
    #[asn(key = 3, extended = false)]
    Protocol(CauseProtocol),
    #[asn(key = 4, extended = false)]
    Misc(CauseMisc),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CauseMisc(pub u8);
impl CauseMisc {
    pub const CONTROL_PROCESSING_OVERLOAD: u8 = 0u8;
    pub const HARDWARE_FAILURE: u8 = 1u8;
    pub const OM_INTERVENTION: u8 = 2u8;
    pub const UNSPECIFIED: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CauseNAS(pub u8);
impl CauseNAS {
    pub const UNSPECIFIED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct CauseProtocol(pub u8);
impl CauseProtocol {
    pub const TRANSFER_SYNTAX_ERROR: u8 = 0u8;
    pub const ABSTRACT_SYNTAX_ERROR_REJECT: u8 = 1u8;
    pub const ABSTRACT_SYNTAX_ERROR_IGNORE_AND_NOTIFY: u8 = 2u8;
    pub const MESSAGE_NOT_COMPATIBLE_WITH_RECEIVER_STATE: u8 = 3u8;
    pub const SEMANTIC_ERROR: u8 = 4u8;
    pub const ABSTRACT_SYNTAX_ERROR_FALSELY_CONSTRUCTED_MESSAGE: u8 = 5u8;
    pub const UNSPECIFIED: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct CauseRadioNetwork(pub u8);
impl CauseRadioNetwork {
    pub const UNKNOWN_OR_ALREADY_ALLOCATED_MCE_MBMS_M2AP_ID: u8 = 0u8;
    pub const UNKNOWN_OR_ALREADY_ALLOCATED_E_NB_MBMS_M2AP_ID: u8 = 1u8;
    pub const UNKNOWN_OR_INCONSISTENT_PAIR_OF_MBMS_M2AP_I_DS: u8 = 2u8;
    pub const RADIO_RESOURCES_NOT_AVAILABLE: u8 = 3u8;
    pub const INTERACTION_WITH_OTHER_PROCEDURE: u8 = 4u8;
    pub const UNSPECIFIED: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CauseTransport(pub u8);
impl CauseTransport {
    pub const TRANSPORT_RESOURCE_UNAVAILABLE: u8 = 0u8;
    pub const UNSPECIFIED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Cell_Information {
    pub ecgi: ECGI,
    pub cell_reservation_info: Cell_InformationCellReservationInfo,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Cell_InformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct Cell_Information_List(pub Vec<Cell_Information>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct Common_Subframe_Allocation_Period(pub u8);
impl Common_Subframe_Allocation_Period {
    pub const RF4: u8 = 0u8;
    pub const RF8: u8 = 1u8;
    pub const RF16: u8 = 2u8;
    pub const RF32: u8 = 3u8;
    pub const RF64: u8 = 4u8;
    pub const RF128: u8 = 5u8;
    pub const RF256: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct CountingResult(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Criticality(pub u8);
impl Criticality {
    pub const REJECT: u8 = 0u8;
    pub const IGNORE: u8 = 1u8;
    pub const NOTIFY: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct CriticalityDiagnostics {
    #[asn(optional_idx = 0)]
    pub procedure_code: Option<ProcedureCode>,
    #[asn(optional_idx = 1)]
    pub triggering_message: Option<TriggeringMessage>,
    #[asn(optional_idx = 2)]
    pub procedure_criticality: Option<Criticality>,
    #[asn(optional_idx = 3)]
    pub i_es_criticality_diagnostics: Option<CriticalityDiagnostics_IE_List>,
    #[asn(optional_idx = 4)]
    pub ie_extensions: Option<CriticalityDiagnosticsIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct CriticalityDiagnostics_IE_List(pub Vec<CriticalityDiagnostics_IE_List_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ECGI {
    pub plmn_identity: PLMN_Identity,
    pub eutra_ncell_identifier: EUTRANCellIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ECGIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "0", extensible = true)]
pub enum ENB_ID {
    #[asn(key = 0, extended = false)]
    Macro_eNB_ID(ENB_ID_macro_eNB_ID),
    #[asn(key = 0, extended = true)]
    Short_Macro_eNB_ID(ENB_ID_short_Macro_eNB_ID),
    #[asn(key = 1, extended = true)]
    Long_Macro_eNB_ID(ENB_ID_long_Macro_eNB_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ENB_MBMS_Configuration_data_ConfigUpdate_Item {
    #[asn(key = 0, extended = false)]
    MBMSConfigData(ENB_MBMS_Configuration_data_Item),
    #[asn(key = 1, extended = false)]
    ECGI(ECGI),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ENB_MBMS_Configuration_data_Item {
    pub ecgi: ECGI,
    pub mbsfn_synchronisation_area: MBSFN_SynchronisationArea_ID,
    pub mbms_service_area_list: MBMS_Service_Area_ID_List,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ENB_MBMS_Configuration_data_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct ENB_MBMS_Configuration_data_List(pub Vec<ENB_MBMS_Configuration_data_List_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct ENB_MBMS_Configuration_data_List_ConfigUpdate(
    pub Vec<ENB_MBMS_Configuration_data_List_ConfigUpdate_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ENB_MBMS_M2AP_ID(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationUpdate {
    pub protocol_i_es: ENBConfigurationUpdateProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationUpdateAcknowledge {
    pub protocol_i_es: ENBConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ENBConfigurationUpdateFailure {
    pub protocol_i_es: ENBConfigurationUpdateFailureProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "PrintableString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct ENBname(pub String);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct EUTRANCellIdentifier(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GBR_QosInformation {
    pub mbms_e_rab_maximum_bitrate_dl: BitRate,
    pub mbms_e_rab_guaranteed_bitrate_dl: BitRate,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GBR_QosInformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct GTP_TEID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalENB_ID {
    pub plmn_identity: PLMN_Identity,
    pub enb_id: ENB_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalENB_IDIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GlobalMCE_ID {
    pub plmn_identity: PLMN_Identity,
    pub mce_id: MCE_ID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<GlobalMCE_IDIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "4",
    sz_ub = "16"
)]
pub struct IPAddress(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitiatingMessage {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: InitiatingMessageValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "28")]
pub struct LCID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum M2AP_PDU {
    #[asn(key = 0, extended = false)]
    InitiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    SuccessfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    UnsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct M2SetupFailure {
    pub protocol_i_es: M2SetupFailureProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct M2SetupRequest {
    pub protocol_i_es: M2SetupRequestProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct M2SetupResponse {
    pub protocol_i_es: M2SetupResponseProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "4096"
)]
pub struct MBMS_Cell_List(pub Vec<ECGI>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct MBMS_Counting_Request_Session(pub Vec<MBMS_Counting_Request_Session_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMS_Counting_Request_SessionIE {
    pub tmgi: TMGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBMS_Counting_Request_SessionIEIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMS_Counting_Result {
    pub tmgi: TMGI,
    pub counting_result: CountingResult,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBMS_Counting_ResultIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct MBMS_Counting_Result_List(pub Vec<MBMS_Counting_Result_List_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MBMS_E_RAB_QoS_Parameters {
    pub qci: QCI,
    #[asn(optional_idx = 0)]
    pub gbr_qos_information: Option<GBR_QosInformation>,
    pub allocation_and_retention_priority: AllocationAndRetentionPriority,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MBMS_E_RAB_QoS_ParametersIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct MBMS_Service_Area(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct MBMS_Service_Area_ID_List(pub Vec<MBMS_Service_Area>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct MBMS_Service_associatedLogicalM2_ConnectionItem {
    #[asn(optional_idx = 0)]
    pub enb_mbms_m2ap_id: Option<ENB_MBMS_M2AP_ID>,
    #[asn(optional_idx = 1)]
    pub mce_mbms_m2ap_id: Option<MCE_MBMS_M2AP_ID>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<MBMS_Service_associatedLogicalM2_ConnectionItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct MBMS_Service_associatedLogicalM2_ConnectionListRes(
    pub Vec<MBMS_Service_associatedLogicalM2_ConnectionListRes_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct MBMS_Service_associatedLogicalM2_ConnectionListResAck(
    pub Vec<MBMS_Service_associatedLogicalM2_ConnectionListResAck_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct MBMS_Session_ID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMS_Suspension_Notification_Item {
    pub sfn: SFN,
    pub mbms_sessions_to_be_suspended_list: MBMSsessionsToBeSuspendedListPerPMCH_Item,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBMS_Suspension_Notification_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct MBMS_Suspension_Notification_List(pub Vec<MBMS_Suspension_Notification_List_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "29")]
pub struct MBMSsessionListPerPMCH_Item(pub Vec<MBMSsessionListPerPMCH_Item_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "29")]
pub struct MBMSsessionsToBeSuspendedListPerPMCH_Item(
    pub Vec<MBMSsessionsToBeSuspendedListPerPMCH_Item_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct MBSFN_Area_Configuration_List(pub Vec<MBSFN_Area_Configuration_List_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct MBSFN_Area_ID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBSFN_Subframe_Configuration {
    pub radioframe_allocation_period: MBSFN_Subframe_ConfigurationRadioframeAllocationPeriod,
    pub radioframe_allocation_offset: MBSFN_Subframe_ConfigurationRadioframeAllocationOffset,
    pub subframe_allocation: MBSFN_Subframe_ConfigurationSubframeAllocation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBSFN_Subframe_ConfigurationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct MBSFN_Subframe_ConfigurationList(pub Vec<MBSFN_Subframe_ConfigurationList_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct MBSFN_SynchronisationArea_ID(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct MCCH_Update_Time(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea(pub Vec<MCCHrelatedBCCH_ConfigPerMBSFNArea_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_Item {
    pub mbsfn_area: MBSFN_Area_ID,
    pub pdcch_length: MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemPdcchLength,
    pub repetition_period: MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemRepetitionPeriod,
    pub offset: MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemOffset,
    pub modification_period: MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemModificationPeriod,
    pub subframe_allocation_info: MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemSubframeAllocationInfo,
    pub modulation_and_coding_scheme:
        MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemModulationAndCodingScheme,
    #[asn(optional_idx = 0)]
    pub cell_information_list: Option<Cell_Information_List>,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea(
    pub Vec<MCCHrelatedBCCH_ExtConfigPerMBSFNArea_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_Item {
    pub mbsfn_area: MBSFN_Area_ID,
    pub repetition_period_expanded:
        MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemRepetitionPeriodExpanded,
    pub offset: MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemOffset,
    pub modification_period_expanded:
        MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemModificationPeriodExpanded,
    pub subframe_allocation_info_expanded:
        MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemSubframeAllocationInfoExpanded,
    pub modulation_and_coding_scheme:
        MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemModulationAndCodingScheme,
    pub subcarrier_spacing_mbms_expanded:
        MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemSubcarrier_SpacingMBMSExpanded,
    #[asn(optional_idx = 0)]
    pub time_separation: Option<MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemTimeSeparation>,
    #[asn(optional_idx = 1)]
    pub cell_information_list: Option<Cell_Information_List>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct MCE_ID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct MCE_MBMS_M2AP_ID(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MCEConfigurationUpdate {
    pub protocol_i_es: MCEConfigurationUpdateProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MCEConfigurationUpdateAcknowledge {
    pub protocol_i_es: MCEConfigurationUpdateAcknowledgeProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MCEConfigurationUpdateFailure {
    pub protocol_i_es: MCEConfigurationUpdateFailureProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "PrintableString",
    sz_extensible = true,
    sz_lb = "1",
    sz_ub = "150"
)]
pub struct MCEname(pub String);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct MCH_Scheduling_Period(pub u8);
impl MCH_Scheduling_Period {
    pub const RF8: u8 = 0u8;
    pub const RF16: u8 = 1u8;
    pub const RF32: u8 = 2u8;
    pub const RF64: u8 = 3u8;
    pub const RF128: u8 = 4u8;
    pub const RF256: u8 = 5u8;
    pub const RF512: u8 = 6u8;
    pub const RF1024: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct MCH_Scheduling_PeriodExtended(pub u8);
impl MCH_Scheduling_PeriodExtended {
    pub const RF4: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MCH_Scheduling_PeriodExtended2(pub u8);
impl MCH_Scheduling_PeriodExtended2 {
    pub const RF1: u8 = 0u8;
    pub const RF2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MbmsOverloadNotification {
    pub protocol_i_es: MbmsOverloadNotificationProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MbmsSchedulingInformation {
    pub protocol_i_es: MbmsSchedulingInformationProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MbmsSchedulingInformationResponse {
    pub protocol_i_es: MbmsSchedulingInformationResponseProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MbmsServiceCountingFailure {
    pub protocol_i_es: MbmsServiceCountingFailureProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MbmsServiceCountingRequest {
    pub protocol_i_es: MbmsServiceCountingRequestProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MbmsServiceCountingResponse {
    pub protocol_i_es: MbmsServiceCountingResponseProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MbmsServiceCountingResultsReport {
    pub protocol_i_es: MbmsServiceCountingResultsReportProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "8")]
pub struct Modification_PeriodExtended(pub u8);
impl Modification_PeriodExtended {
    pub const RF1: u8 = 0u8;
    pub const RF2: u8 = 1u8;
    pub const RF4: u8 = 2u8;
    pub const RF8: u8 = 3u8;
    pub const RF16: u8 = 4u8;
    pub const RF32: u8 = 5u8;
    pub const RF64: u8 = 6u8;
    pub const RF128: u8 = 7u8;
    pub const RF256: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "27")]
pub struct Modulation_Coding_Scheme2(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct Overload_Status_Per_PMCH_List(pub Vec<Overload_Status_Per_PMCH_List_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct PLMN_Identity(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct PMCH_Bandwidth(pub u8);
impl PMCH_Bandwidth {
    pub const N40: u8 = 0u8;
    pub const N35: u8 = 1u8;
    pub const N30: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PMCH_Configuration {
    pub allocated_subframes_end: AllocatedSubframesEnd,
    pub data_mcs: PMCH_ConfigurationDataMCS,
    pub mch_scheduling_period: MCH_Scheduling_Period,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PMCH_ConfigurationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PMCH_Configuration_Item {
    pub pmch_configuration: PMCH_Configuration,
    pub mbms_session_list: MBMSsessionListPerPMCH_Item,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PMCH_Configuration_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "15")]
pub struct PMCH_Configuration_List(pub Vec<PMCH_Configuration_List_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PMCH_Overload_Status(pub u8);
impl PMCH_Overload_Status {
    pub const NORMAL: u8 = 0u8;
    pub const OVERLOAD: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Pre_emptionCapability(pub u8);
impl Pre_emptionCapability {
    pub const SHALL_NOT_TRIGGER_PRE_EMPTION: u8 = 0u8;
    pub const MAY_TRIGGER_PRE_EMPTION: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Pre_emptionVulnerability(pub u8);
impl Pre_emptionVulnerability {
    pub const NOT_PRE_EMPTABLE: u8 = 0u8;
    pub const PRE_EMPTABLE: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Presence(pub u8);
impl Presence {
    pub const OPTIONAL: u8 = 0u8;
    pub const CONDITIONAL: u8 = 1u8;
    pub const MANDATORY: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct PriorityLevel(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum PrivateIE_ID {
    #[asn(key = 0, extended = false)]
    Local(PrivateIE_ID_local),
    #[asn(key = 1, extended = false)]
    Global(PrivateIE_ID_global),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PrivateMessage {
    pub private_i_es: PrivateMessagePrivateIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ProcedureCode(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ProtocolIE_ID(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct QCI(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct Repetition_PeriodExtended(pub u8);
impl Repetition_PeriodExtended {
    pub const RF1: u8 = 0u8;
    pub const RF2: u8 = 1u8;
    pub const RF4: u8 = 2u8;
    pub const RF8: u8 = 3u8;
    pub const RF16: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Reset {
    pub protocol_i_es: ResetProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ResetAcknowledge {
    pub protocol_i_es: ResetAcknowledgeProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ResetAll(pub u8);
impl ResetAll {
    pub const RESET_ALL: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ResetType {
    #[asn(key = 0, extended = false)]
    M2_Interface(ResetAll),
    #[asn(key = 1, extended = false)]
    PartOfM2_Interface(MBMS_Service_associatedLogicalM2_ConnectionListRes),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SC_PTM_Information {
    pub mbms_cell_list: MBMS_Cell_List,
    pub mbms_e_rab_qo_s_parameters: MBMS_E_RAB_QoS_Parameters,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SC_PTM_InformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct SFN(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SessionStartFailure {
    pub protocol_i_es: SessionStartFailureProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SessionStartRequest {
    pub protocol_i_es: SessionStartRequestProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SessionStartResponse {
    pub protocol_i_es: SessionStartResponseProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SessionStopRequest {
    pub protocol_i_es: SessionStopRequestProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SessionStopResponse {
    pub protocol_i_es: SessionStopResponseProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SessionUpdateFailure {
    pub protocol_i_es: SessionUpdateFailureProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SessionUpdateRequest {
    pub protocol_i_es: SessionUpdateRequestProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SessionUpdateResponse {
    pub protocol_i_es: SessionUpdateResponseProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Subcarrier_SpacingMBMS(pub u8);
impl Subcarrier_SpacingMBMS {
    pub const KHZ_7DOT5: u8 = 0u8;
    pub const KHZ_1DOT25: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum SubframeAllocationExtended {
    #[asn(key = 0, extended = false)]
    OneFrameExtension(SubframeAllocationExtended_oneFrameExtension),
    #[asn(key = 1, extended = false)]
    FourFrameExtension(SubframeAllocationExtended_fourFrameExtension),
    #[asn(key = 2, extended = false)]
    Choice_extension(SubframeAllocationExtended_choice_extension),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum SubframeAllocationFurtherExtension {
    #[asn(key = 0, extended = false)]
    OneFrameFurtherExtension(SubframeAllocationFurtherExtension_oneFrameFurtherExtension),
    #[asn(key = 1, extended = false)]
    FourFrameFurtherExtension(SubframeAllocationFurtherExtension_fourFrameFurtherExtension),
    #[asn(key = 2, extended = false)]
    Choice_extension(SubframeAllocationFurtherExtension_choice_extension),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: SuccessfulOutcomeValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TMGI {
    pub plm_nidentity: PLMN_Identity,
    pub service_id: TMGIServiceID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TMGIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TNL_Information {
    pub ipmc_address: IPAddress,
    pub ip_source_address: IPAddress,
    pub gtp_teid: GTP_TEID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TNL_InformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct TimeToWait(pub u8);
impl TimeToWait {
    pub const V1S: u8 = 0u8;
    pub const V2S: u8 = 1u8;
    pub const V5S: u8 = 2u8;
    pub const V10S: u8 = 3u8;
    pub const V20S: u8 = 4u8;
    pub const V60S: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct TriggeringMessage(pub u8);
impl TriggeringMessage {
    pub const INITIATING_MESSAGE: u8 = 0u8;
    pub const SUCCESSFUL_OUTCOME: u8 = 1u8;
    pub const UNSUCCESSFUL_OUTCOME: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct TypeOfError(pub u8);
impl TypeOfError {
    pub const NOT_UNDERSTOOD: u8 = 0u8;
    pub const MISSING: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub value: UnsuccessfulOutcomeValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum Active_MBMS_Session_List_Entry_EntryValue {
    #[asn(key = 2)]
    Id_TMGI(TMGI),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Active_MBMS_Session_List_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Active_MBMS_Session_List_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Active_MBMS_Session_List_Entry(pub Vec<Active_MBMS_Session_List_Entry_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AdditionalConfigParametersIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AdditionalConfigParametersIE_Extensions(
    pub Vec<AdditionalConfigParametersIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AllocationAndRetentionPriorityIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AllocationAndRetentionPriorityIE_Extensions(
    pub Vec<AllocationAndRetentionPriorityIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Cell_InformationCellReservationInfo(pub u8);
impl Cell_InformationCellReservationInfo {
    pub const RESERVED_CELL: u8 = 0u8;
    pub const NON_RESERVED_CELL: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Cell_InformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Cell_InformationIE_Extensions(pub Vec<Cell_InformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnosticsIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnosticsIE_Extensions(pub Vec<CriticalityDiagnosticsIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CriticalityDiagnostics_IE_List_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct CriticalityDiagnostics_IE_List_EntryIE_Extensions(
    pub Vec<CriticalityDiagnostics_IE_List_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CriticalityDiagnostics_IE_List_Entry {
    pub ie_criticality: Criticality,
    pub ie_id: ProtocolIE_ID,
    pub type_of_error: TypeOfError,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<CriticalityDiagnostics_IE_List_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECGIIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ECGIIE_Extensions(pub Vec<ECGIIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "20", sz_ub = "20")]
pub struct ENB_ID_macro_eNB_ID(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "18", sz_ub = "18")]
pub struct ENB_ID_short_Macro_eNB_ID(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "21", sz_ub = "21")]
pub struct ENB_ID_long_Macro_eNB_ID(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENB_MBMS_Configuration_data_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ENB_MBMS_Configuration_data_ItemIE_Extensions(
    pub Vec<ENB_MBMS_Configuration_data_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ENB_MBMS_Configuration_data_List_EntryValue {
    #[asn(key = 16)]
    Id_ENB_MBMS_Configuration_data_Item(ENB_MBMS_Configuration_data_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENB_MBMS_Configuration_data_List_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENB_MBMS_Configuration_data_List_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ENB_MBMS_Configuration_data_List_ConfigUpdate_EntryValue {
    #[asn(key = 27)]
    Id_ENB_MBMS_Configuration_data_ConfigUpdate_Item(ENB_MBMS_Configuration_data_ConfigUpdate_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENB_MBMS_Configuration_data_List_ConfigUpdate_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENB_MBMS_Configuration_data_List_ConfigUpdate_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationUpdateProtocolIEs_EntryValue {
    #[asn(key = 26)]
    Id_ENB_MBMS_Configuration_data_List_ConfigUpdate(ENB_MBMS_Configuration_data_List_ConfigUpdate),
    #[asn(key = 14)]
    Id_ENBname(ENBname),
    #[asn(key = 13)]
    Id_GlobalENB_ID(GlobalENB_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationUpdateProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationUpdateProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationUpdateProtocolIEs(pub Vec<ENBConfigurationUpdateProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationUpdateAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 19)]
    Id_MCCHrelatedBCCH_ConfigPerMBSFNArea(MCCHrelatedBCCH_ConfigPerMBSFNArea),
    #[asn(key = 52)]
    Id_MCCHrelatedBCCH_ExtConfigPerMBSFNArea(MCCHrelatedBCCH_ExtConfigPerMBSFNArea),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationUpdateAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationUpdateAcknowledgeProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationUpdateAcknowledgeProtocolIEs(
    pub Vec<ENBConfigurationUpdateAcknowledgeProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ENBConfigurationUpdateFailureProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_Cause(Cause),
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 21)]
    Id_TimeToWait(TimeToWait),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ENBConfigurationUpdateFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ENBConfigurationUpdateFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ENBConfigurationUpdateFailureProtocolIEs(
    pub Vec<ENBConfigurationUpdateFailureProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ErrorIndicationProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_Cause(Cause),
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 1)]
    Id_ENB_MBMS_M2AP_ID(ENB_MBMS_M2AP_ID),
    #[asn(key = 0)]
    Id_MCE_MBMS_M2AP_ID(MCE_MBMS_M2AP_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ErrorIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ErrorIndicationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ErrorIndicationProtocolIEs(pub Vec<ErrorIndicationProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GBR_QosInformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GBR_QosInformationIE_Extensions(pub Vec<GBR_QosInformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalENB_IDIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalENB_IDIE_Extensions(pub Vec<GlobalENB_IDIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct GlobalMCE_IDIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct GlobalMCE_IDIE_Extensions(pub Vec<GlobalMCE_IDIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InitiatingMessageValue {
    #[asn(key = 6)]
    Id_eNBConfigurationUpdate(ENBConfigurationUpdate),
    #[asn(key = 3)]
    Id_errorIndication(ErrorIndication),
    #[asn(key = 5)]
    Id_m2Setup(M2SetupRequest),
    #[asn(key = 7)]
    Id_mCEConfigurationUpdate(MCEConfigurationUpdate),
    #[asn(key = 12)]
    Id_mbmsOverloadNotification(MbmsOverloadNotification),
    #[asn(key = 2)]
    Id_mbmsSchedulingInformation(MbmsSchedulingInformation),
    #[asn(key = 10)]
    Id_mbmsServiceCounting(MbmsServiceCountingRequest),
    #[asn(key = 11)]
    Id_mbmsServiceCountingResultsReport(MbmsServiceCountingResultsReport),
    #[asn(key = 8)]
    Id_privateMessage(PrivateMessage),
    #[asn(key = 4)]
    Id_reset(Reset),
    #[asn(key = 0)]
    Id_sessionStart(SessionStartRequest),
    #[asn(key = 1)]
    Id_sessionStop(SessionStopRequest),
    #[asn(key = 9)]
    Id_sessionUpdate(SessionUpdateRequest),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum M2SetupFailureProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_Cause(Cause),
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 21)]
    Id_TimeToWait(TimeToWait),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M2SetupFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: M2SetupFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct M2SetupFailureProtocolIEs(pub Vec<M2SetupFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum M2SetupRequestProtocolIEs_EntryValue {
    #[asn(key = 15)]
    Id_ENB_MBMS_Configuration_data_List(ENB_MBMS_Configuration_data_List),
    #[asn(key = 14)]
    Id_ENBname(ENBname),
    #[asn(key = 13)]
    Id_GlobalENB_ID(GlobalENB_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M2SetupRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: M2SetupRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct M2SetupRequestProtocolIEs(pub Vec<M2SetupRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum M2SetupResponseProtocolIEs_EntryValue {
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 17)]
    Id_GlobalMCE_ID(GlobalMCE_ID),
    #[asn(key = 19)]
    Id_MCCHrelatedBCCH_ConfigPerMBSFNArea(MCCHrelatedBCCH_ConfigPerMBSFNArea),
    #[asn(key = 52)]
    Id_MCCHrelatedBCCH_ExtConfigPerMBSFNArea(MCCHrelatedBCCH_ExtConfigPerMBSFNArea),
    #[asn(key = 18)]
    Id_MCEname(MCEname),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct M2SetupResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: M2SetupResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct M2SetupResponseProtocolIEs(pub Vec<M2SetupResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMS_Counting_Request_Session_Entry_EntryValue {
    #[asn(key = 33)]
    Id_MBMS_Counting_Request_Session_Item(MBMS_Counting_Request_SessionIE),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_Counting_Request_Session_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMS_Counting_Request_Session_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMS_Counting_Request_Session_Entry(pub Vec<MBMS_Counting_Request_Session_Entry_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_Counting_Request_SessionIEIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMS_Counting_Request_SessionIEIE_Extensions(
    pub Vec<MBMS_Counting_Request_SessionIEIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_Counting_ResultIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMS_Counting_ResultIE_Extensions(pub Vec<MBMS_Counting_ResultIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMS_Counting_Result_List_Entry_EntryValue {
    #[asn(key = 35)]
    Id_MBMS_Counting_Result_Item(MBMS_Counting_Result),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_Counting_Result_List_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMS_Counting_Result_List_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBMS_Counting_Result_List_Entry(pub Vec<MBMS_Counting_Result_List_Entry_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_E_RAB_QoS_ParametersIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMS_E_RAB_QoS_ParametersIE_Extensions(
    pub Vec<MBMS_E_RAB_QoS_ParametersIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_Service_associatedLogicalM2_ConnectionItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMS_Service_associatedLogicalM2_ConnectionItemIE_Extensions(
    pub Vec<MBMS_Service_associatedLogicalM2_ConnectionItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMS_Service_associatedLogicalM2_ConnectionListRes_EntryValue {
    #[asn(key = 28)]
    Id_MBMS_Service_associatedLogicalM2_ConnectionItem(
        MBMS_Service_associatedLogicalM2_ConnectionItem,
    ),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_Service_associatedLogicalM2_ConnectionListRes_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMS_Service_associatedLogicalM2_ConnectionListRes_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMS_Service_associatedLogicalM2_ConnectionListResAck_EntryValue {
    #[asn(key = 28)]
    Id_MBMS_Service_associatedLogicalM2_ConnectionItem(
        MBMS_Service_associatedLogicalM2_ConnectionItem,
    ),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_Service_associatedLogicalM2_ConnectionListResAck_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMS_Service_associatedLogicalM2_ConnectionListResAck_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_Suspension_Notification_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMS_Suspension_Notification_ItemIE_Extensions(
    pub Vec<MBMS_Suspension_Notification_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBMS_Suspension_Notification_List_EntryValue {
    #[asn(key = 44)]
    Id_MBMS_Suspension_Notification_Item(MBMS_Suspension_Notification_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMS_Suspension_Notification_List_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBMS_Suspension_Notification_List_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSsessionListPerPMCH_Item_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSsessionListPerPMCH_Item_EntryIE_Extensions(
    pub Vec<MBMSsessionListPerPMCH_Item_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSsessionListPerPMCH_Item_Entry {
    pub tmgi: TMGI,
    pub lcid: LCID,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBMSsessionListPerPMCH_Item_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBMSsessionsToBeSuspendedListPerPMCH_Item_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBMSsessionsToBeSuspendedListPerPMCH_Item_EntryIE_Extensions(
    pub Vec<MBMSsessionsToBeSuspendedListPerPMCH_Item_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBMSsessionsToBeSuspendedListPerPMCH_Item_Entry {
    pub tmgi: TMGI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MBMSsessionsToBeSuspendedListPerPMCH_Item_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBSFN_Area_Configuration_List_Entry_EntryValue {
    #[asn(key = 24)]
    Id_Common_Subframe_Allocation_Period(Common_Subframe_Allocation_Period),
    #[asn(key = 43)]
    Id_MBMS_Suspension_Notification_List(MBMS_Suspension_Notification_List),
    #[asn(key = 29)]
    Id_MBSFN_Area_ID(MBSFN_Area_ID),
    #[asn(key = 22)]
    Id_MBSFN_Subframe_Configuration_List(MBSFN_Subframe_ConfigurationList),
    #[asn(key = 11)]
    Id_PMCH_Configuration_List(PMCH_Configuration_List),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBSFN_Area_Configuration_List_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBSFN_Area_Configuration_List_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MBSFN_Area_Configuration_List_Entry(pub Vec<MBSFN_Area_Configuration_List_Entry_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct MBSFN_Subframe_ConfigurationRadioframeAllocationPeriod(pub u8);
impl MBSFN_Subframe_ConfigurationRadioframeAllocationPeriod {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct MBSFN_Subframe_ConfigurationRadioframeAllocationOffset(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct MBSFN_Subframe_ConfigurationSubframeAllocation_oneFrame(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "24", sz_ub = "24")]
pub struct MBSFN_Subframe_ConfigurationSubframeAllocation_fourFrames(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum MBSFN_Subframe_ConfigurationSubframeAllocation {
    #[asn(key = 0, extended = false)]
    OneFrame(MBSFN_Subframe_ConfigurationSubframeAllocation_oneFrame),
    #[asn(key = 1, extended = false)]
    FourFrames(MBSFN_Subframe_ConfigurationSubframeAllocation_fourFrames),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBSFN_Subframe_ConfigurationIE_Extensions_EntryExtensionValue {
    #[asn(key = 50)]
    Id_SubframeAllocationExtended(SubframeAllocationExtended),
    #[asn(key = 53)]
    Id_SubframeAllocationFurtherExtension(SubframeAllocationFurtherExtension),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBSFN_Subframe_ConfigurationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub extension_value: MBSFN_Subframe_ConfigurationIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MBSFN_Subframe_ConfigurationIE_Extensions(
    pub Vec<MBSFN_Subframe_ConfigurationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MBSFN_Subframe_ConfigurationList_EntryValue {
    #[asn(key = 23)]
    Id_MBSFN_Subframe_Configuration_Item(MBSFN_Subframe_Configuration),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBSFN_Subframe_ConfigurationList_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MBSFN_Subframe_ConfigurationList_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MCCHrelatedBCCH_ConfigPerMBSFNArea_EntryValue {
    #[asn(key = 20)]
    Id_MCCHrelatedBCCH_ConfigPerMBSFNArea_Item(MCCHrelatedBCCH_ConfigPerMBSFNArea_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MCCHrelatedBCCH_ConfigPerMBSFNArea_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemPdcchLength(pub u8);
impl MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemPdcchLength {
    pub const S1: u8 = 0u8;
    pub const S2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemRepetitionPeriod(pub u8);
impl MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemRepetitionPeriod {
    pub const RF32: u8 = 0u8;
    pub const RF64: u8 = 1u8;
    pub const RF128: u8 = 2u8;
    pub const RF256: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "10")]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemOffset(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemModificationPeriod(pub u8);
impl MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemModificationPeriod {
    pub const RF512: u8 = 0u8;
    pub const RF1024: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemSubframeAllocationInfo(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemModulationAndCodingScheme(pub u8);
impl MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemModulationAndCodingScheme {
    pub const N2: u8 = 0u8;
    pub const N7: u8 = 1u8;
    pub const N13: u8 = 2u8;
    pub const N19: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 46)]
    Id_Modification_PeriodExtended(Modification_PeriodExtended),
    #[asn(key = 47)]
    Id_Repetition_PeriodExtended(Repetition_PeriodExtended),
    #[asn(key = 49)]
    Id_Subcarrier_SpacingMBMS(Subcarrier_SpacingMBMS),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub extension_value: MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemIE_Extensions(
    pub Vec<MCCHrelatedBCCH_ConfigPerMBSFNArea_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MCCHrelatedBCCH_ExtConfigPerMBSFNArea_EntryValue {
    #[asn(key = 51)]
    Id_MCCHrelatedBCCH_ExtConfigPerMBSFNArea_Item(MCCHrelatedBCCH_ExtConfigPerMBSFNArea_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MCCHrelatedBCCH_ExtConfigPerMBSFNArea_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "8")]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemRepetitionPeriodExpanded(pub u8);
impl MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemRepetitionPeriodExpanded {
    pub const RF1: u8 = 0u8;
    pub const RF2: u8 = 1u8;
    pub const RF4: u8 = 2u8;
    pub const RF8: u8 = 3u8;
    pub const RF16: u8 = 4u8;
    pub const RF32: u8 = 5u8;
    pub const RF64: u8 = 6u8;
    pub const RF128: u8 = 7u8;
    pub const RF256: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "10")]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemOffset(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "10")]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemModificationPeriodExpanded(pub u8);
impl MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemModificationPeriodExpanded {
    pub const RF1: u8 = 0u8;
    pub const RF2: u8 = 1u8;
    pub const RF4: u8 = 2u8;
    pub const RF8: u8 = 3u8;
    pub const RF16: u8 = 4u8;
    pub const RF32: u8 = 5u8;
    pub const RF64: u8 = 6u8;
    pub const RF128: u8 = 7u8;
    pub const RF256: u8 = 8u8;
    pub const RF512: u8 = 9u8;
    pub const RF1024: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemSubframeAllocationInfoExpanded(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemModulationAndCodingScheme(pub u8);
impl MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemModulationAndCodingScheme {
    pub const N2: u8 = 0u8;
    pub const N7: u8 = 1u8;
    pub const N13: u8 = 2u8;
    pub const N19: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemSubcarrier_SpacingMBMSExpanded(pub u8);
impl MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemSubcarrier_SpacingMBMSExpanded {
    pub const KHZ_7DOT5: u8 = 0u8;
    pub const KHZ_2DOT5: u8 = 1u8;
    pub const KHZ_1DOT25: u8 = 2u8;
    pub const KHZ_0DOT37: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemTimeSeparation(pub u8);
impl MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemTimeSeparation {
    pub const SL2: u8 = 0u8;
    pub const SL4: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 54)]
    Id_AdditionalConfigParameters(AdditionalConfigParameters),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub extension_value:
        MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemIE_Extensions(
    pub Vec<MCCHrelatedBCCH_ExtConfigPerMBSFNArea_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MCEConfigurationUpdateProtocolIEs_EntryValue {
    #[asn(key = 17)]
    Id_GlobalMCE_ID(GlobalMCE_ID),
    #[asn(key = 19)]
    Id_MCCHrelatedBCCH_ConfigPerMBSFNArea(MCCHrelatedBCCH_ConfigPerMBSFNArea),
    #[asn(key = 52)]
    Id_MCCHrelatedBCCH_ExtConfigPerMBSFNArea(MCCHrelatedBCCH_ExtConfigPerMBSFNArea),
    #[asn(key = 18)]
    Id_MCEname(MCEname),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MCEConfigurationUpdateProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MCEConfigurationUpdateProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MCEConfigurationUpdateProtocolIEs(pub Vec<MCEConfigurationUpdateProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MCEConfigurationUpdateAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MCEConfigurationUpdateAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MCEConfigurationUpdateAcknowledgeProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MCEConfigurationUpdateAcknowledgeProtocolIEs(
    pub Vec<MCEConfigurationUpdateAcknowledgeProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MCEConfigurationUpdateFailureProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_Cause(Cause),
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 21)]
    Id_TimeToWait(TimeToWait),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MCEConfigurationUpdateFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MCEConfigurationUpdateFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MCEConfigurationUpdateFailureProtocolIEs(
    pub Vec<MCEConfigurationUpdateFailureProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MbmsOverloadNotificationProtocolIEs_EntryValue {
    #[asn(key = 29)]
    Id_MBSFN_Area_ID(MBSFN_Area_ID),
    #[asn(key = 39)]
    Id_Overload_Status_Per_PMCH_List(Overload_Status_Per_PMCH_List),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MbmsOverloadNotificationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MbmsOverloadNotificationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MbmsOverloadNotificationProtocolIEs(pub Vec<MbmsOverloadNotificationProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MbmsSchedulingInformationProtocolIEs_EntryValue {
    #[asn(key = 10)]
    Id_MBSFN_Area_Configuration_List(MBSFN_Area_Configuration_List),
    #[asn(key = 25)]
    Id_MCCH_Update_Time(MCCH_Update_Time),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MbmsSchedulingInformationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MbmsSchedulingInformationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MbmsSchedulingInformationProtocolIEs(
    pub Vec<MbmsSchedulingInformationProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MbmsSchedulingInformationResponseProtocolIEs_EntryValue {
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MbmsSchedulingInformationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MbmsSchedulingInformationResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MbmsSchedulingInformationResponseProtocolIEs(
    pub Vec<MbmsSchedulingInformationResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MbmsServiceCountingFailureProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_Cause(Cause),
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MbmsServiceCountingFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MbmsServiceCountingFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MbmsServiceCountingFailureProtocolIEs(
    pub Vec<MbmsServiceCountingFailureProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MbmsServiceCountingRequestProtocolIEs_EntryValue {
    #[asn(key = 32)]
    Id_MBMS_Counting_Request_Session(MBMS_Counting_Request_Session),
    #[asn(key = 29)]
    Id_MBSFN_Area_ID(MBSFN_Area_ID),
    #[asn(key = 25)]
    Id_MCCH_Update_Time(MCCH_Update_Time),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MbmsServiceCountingRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MbmsServiceCountingRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MbmsServiceCountingRequestProtocolIEs(
    pub Vec<MbmsServiceCountingRequestProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MbmsServiceCountingResponseProtocolIEs_EntryValue {
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MbmsServiceCountingResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MbmsServiceCountingResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MbmsServiceCountingResponseProtocolIEs(
    pub Vec<MbmsServiceCountingResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MbmsServiceCountingResultsReportProtocolIEs_EntryValue {
    #[asn(key = 34)]
    Id_MBMS_Counting_Result_List(MBMS_Counting_Result_List),
    #[asn(key = 29)]
    Id_MBSFN_Area_ID(MBSFN_Area_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MbmsServiceCountingResultsReportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MbmsServiceCountingResultsReportProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct MbmsServiceCountingResultsReportProtocolIEs(
    pub Vec<MbmsServiceCountingResultsReportProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum Overload_Status_Per_PMCH_List_Entry_EntryValue {
    #[asn(key = 42)]
    Id_Active_MBMS_Session_List(Active_MBMS_Session_List),
    #[asn(key = 41)]
    Id_PMCH_Overload_Status(PMCH_Overload_Status),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Overload_Status_Per_PMCH_List_Entry_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: Overload_Status_Per_PMCH_List_Entry_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct Overload_Status_Per_PMCH_List_Entry(pub Vec<Overload_Status_Per_PMCH_List_Entry_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "28")]
pub struct PMCH_ConfigurationDataMCS(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum PMCH_ConfigurationIE_Extensions_EntryExtensionValue {
    #[asn(key = 37)]
    Id_MCH_Scheduling_PeriodExtended(MCH_Scheduling_PeriodExtended),
    #[asn(key = 48)]
    Id_MCH_Scheduling_PeriodExtended2(MCH_Scheduling_PeriodExtended2),
    #[asn(key = 36)]
    Id_Modulation_Coding_Scheme2(Modulation_Coding_Scheme2),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PMCH_ConfigurationIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub extension_value: PMCH_ConfigurationIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PMCH_ConfigurationIE_Extensions(pub Vec<PMCH_ConfigurationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PMCH_Configuration_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PMCH_Configuration_ItemIE_Extensions(
    pub Vec<PMCH_Configuration_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum PMCH_Configuration_List_EntryValue {
    #[asn(key = 12)]
    Id_PMCH_Configuration_Item(PMCH_Configuration_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PMCH_Configuration_List_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: PMCH_Configuration_List_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct PrivateIE_ID_local(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OBJECT-IDENTIFIER")]
pub struct PrivateIE_ID_global(Vec<u32>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PrivateMessagePrivateIEs_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PrivateMessagePrivateIEs(pub Vec<PrivateMessagePrivateIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_Cause(Cause),
    #[asn(key = 30)]
    Id_ResetType(ResetType),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetProtocolIEs(pub Vec<ResetProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResetAcknowledgeProtocolIEs_EntryValue {
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 31)]
    Id_MBMS_Service_associatedLogicalM2_ConnectionListResAck(
        MBMS_Service_associatedLogicalM2_ConnectionListResAck,
    ),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResetAcknowledgeProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: ResetAcknowledgeProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct ResetAcknowledgeProtocolIEs(pub Vec<ResetAcknowledgeProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SC_PTM_InformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SC_PTM_InformationIE_Extensions(pub Vec<SC_PTM_InformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SessionStartFailureProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_Cause(Cause),
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 0)]
    Id_MCE_MBMS_M2AP_ID(MCE_MBMS_M2AP_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SessionStartFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SessionStartFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SessionStartFailureProtocolIEs(pub Vec<SessionStartFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SessionStartRequestProtocolIEs_EntryValue {
    #[asn(key = 38)]
    Id_Alternative_TNL_Information(TNL_Information),
    #[asn(key = 6)]
    Id_MBMS_Service_Area(MBMS_Service_Area),
    #[asn(key = 3)]
    Id_MBMS_Session_ID(MBMS_Session_ID),
    #[asn(key = 0)]
    Id_MCE_MBMS_M2AP_ID(MCE_MBMS_M2AP_ID),
    #[asn(key = 45)]
    Id_SC_PTM_Information(SC_PTM_Information),
    #[asn(key = 2)]
    Id_TMGI(TMGI),
    #[asn(key = 7)]
    Id_TNL_Information(TNL_Information),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SessionStartRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SessionStartRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SessionStartRequestProtocolIEs(pub Vec<SessionStartRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SessionStartResponseProtocolIEs_EntryValue {
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 1)]
    Id_ENB_MBMS_M2AP_ID(ENB_MBMS_M2AP_ID),
    #[asn(key = 0)]
    Id_MCE_MBMS_M2AP_ID(MCE_MBMS_M2AP_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SessionStartResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SessionStartResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SessionStartResponseProtocolIEs(pub Vec<SessionStartResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SessionStopRequestProtocolIEs_EntryValue {
    #[asn(key = 1)]
    Id_ENB_MBMS_M2AP_ID(ENB_MBMS_M2AP_ID),
    #[asn(key = 0)]
    Id_MCE_MBMS_M2AP_ID(MCE_MBMS_M2AP_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SessionStopRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SessionStopRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SessionStopRequestProtocolIEs(pub Vec<SessionStopRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SessionStopResponseProtocolIEs_EntryValue {
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 1)]
    Id_ENB_MBMS_M2AP_ID(ENB_MBMS_M2AP_ID),
    #[asn(key = 0)]
    Id_MCE_MBMS_M2AP_ID(MCE_MBMS_M2AP_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SessionStopResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SessionStopResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SessionStopResponseProtocolIEs(pub Vec<SessionStopResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SessionUpdateFailureProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_Cause(Cause),
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 1)]
    Id_ENB_MBMS_M2AP_ID(ENB_MBMS_M2AP_ID),
    #[asn(key = 0)]
    Id_MCE_MBMS_M2AP_ID(MCE_MBMS_M2AP_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SessionUpdateFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SessionUpdateFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SessionUpdateFailureProtocolIEs(pub Vec<SessionUpdateFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SessionUpdateRequestProtocolIEs_EntryValue {
    #[asn(key = 1)]
    Id_ENB_MBMS_M2AP_ID(ENB_MBMS_M2AP_ID),
    #[asn(key = 6)]
    Id_MBMS_Service_Area(MBMS_Service_Area),
    #[asn(key = 3)]
    Id_MBMS_Session_ID(MBMS_Session_ID),
    #[asn(key = 0)]
    Id_MCE_MBMS_M2AP_ID(MCE_MBMS_M2AP_ID),
    #[asn(key = 45)]
    Id_SC_PTM_Information(SC_PTM_Information),
    #[asn(key = 2)]
    Id_TMGI(TMGI),
    #[asn(key = 7)]
    Id_TNL_Information(TNL_Information),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SessionUpdateRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SessionUpdateRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SessionUpdateRequestProtocolIEs(pub Vec<SessionUpdateRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SessionUpdateResponseProtocolIEs_EntryValue {
    #[asn(key = 8)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 1)]
    Id_ENB_MBMS_M2AP_ID(ENB_MBMS_M2AP_ID),
    #[asn(key = 0)]
    Id_MCE_MBMS_M2AP_ID(MCE_MBMS_M2AP_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SessionUpdateResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: SessionUpdateResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct SessionUpdateResponseProtocolIEs(pub Vec<SessionUpdateResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct SubframeAllocationExtended_oneFrameExtension(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct SubframeAllocationExtended_fourFrameExtension(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SubframeAllocationExtended_choice_extension {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct SubframeAllocationFurtherExtension_oneFrameFurtherExtension(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct SubframeAllocationFurtherExtension_fourFrameFurtherExtension(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SubframeAllocationFurtherExtension_choice_extension {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomeValue {
    #[asn(key = 6)]
    Id_eNBConfigurationUpdate(ENBConfigurationUpdateAcknowledge),
    #[asn(key = 5)]
    Id_m2Setup(M2SetupResponse),
    #[asn(key = 7)]
    Id_mCEConfigurationUpdate(MCEConfigurationUpdateAcknowledge),
    #[asn(key = 2)]
    Id_mbmsSchedulingInformation(MbmsSchedulingInformationResponse),
    #[asn(key = 10)]
    Id_mbmsServiceCounting(MbmsServiceCountingResponse),
    #[asn(key = 4)]
    Id_reset(ResetAcknowledge),
    #[asn(key = 0)]
    Id_sessionStart(SessionStartResponse),
    #[asn(key = 1)]
    Id_sessionStop(SessionStopResponse),
    #[asn(key = 9)]
    Id_sessionUpdate(SessionUpdateResponse),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct TMGIServiceID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TMGIIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TMGIIE_Extensions(pub Vec<TMGIIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TNL_InformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TNL_InformationIE_Extensions(pub Vec<TNL_InformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomeValue {
    #[asn(key = 6)]
    Id_eNBConfigurationUpdate(ENBConfigurationUpdateFailure),
    #[asn(key = 5)]
    Id_m2Setup(M2SetupFailure),
    #[asn(key = 7)]
    Id_mCEConfigurationUpdate(MCEConfigurationUpdateFailure),
    #[asn(key = 10)]
    Id_mbmsServiceCounting(MbmsServiceCountingFailure),
    #[asn(key = 0)]
    Id_sessionStart(SessionStartFailure),
    #[asn(key = 9)]
    Id_sessionUpdate(SessionUpdateFailure),
}
