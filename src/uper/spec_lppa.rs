pub const ID_ADD_OTDOA_CELLS: u16 = 18;

pub const ID_ASSISTANCE_INFORMATION: u16 = 22;

pub const ID_ASSISTANCE_INFORMATION_FAILURE_LIST: u16 = 24;

pub const ID_BROADCAST: u16 = 23;

pub const ID_CAUSE: u16 = 0;

pub const ID_CELL_PORTION_ID: u16 = 14;

pub const ID_CRITICALITY_DIAGNOSTICS: u16 = 1;

pub const ID_E_CID_MEASUREMENT_RESULT: u16 = 7;

pub const ID_E_SMLC_UE_MEASUREMENT_ID: u16 = 2;

pub const ID_INTER_RAT_MEASUREMENT_QUANTITIES: u16 = 15;

pub const ID_INTER_RAT_MEASUREMENT_QUANTITIES_ITEM: u16 = 16;

pub const ID_INTER_RAT_MEASUREMENT_RESULT: u16 = 17;

pub const ID_MEASUREMENT_PERIODICITY: u16 = 4;

pub const ID_MEASUREMENT_QUANTITIES: u16 = 5;

pub const ID_MEASUREMENT_QUANTITIES_ITEM: u16 = 11;

pub const ID_NR_CGI: u16 = 27;

pub const ID_OTDOA_INFORMATION_TYPE_GROUP: u16 = 9;

pub const ID_OTDOA_INFORMATION_TYPE_ITEM: u16 = 10;

pub const ID_OTDOA_CELLS: u16 = 8;

pub const ID_REPORT_CHARACTERISTICS: u16 = 3;

pub const ID_REQUESTED_SRS_TRANSMISSION_CHARACTERISTICS: u16 = 12;

pub const ID_RESULTS_PER_SSB_INDEX_ITEM: u16 = 26;

pub const ID_RESULTS_PER_SSB_INDEX_LIST: u16 = 25;

pub const ID_UL_CONFIGURATION: u16 = 13;

pub const ID_WLAN_MEASUREMENT_QUANTITIES: u16 = 19;

pub const ID_WLAN_MEASUREMENT_QUANTITIES_ITEM: u16 = 20;

pub const ID_WLAN_MEASUREMENT_RESULT: u16 = 21;

pub const ID_ASSISTANCE_INFORMATION_CONTROL: u8 = 9;

pub const ID_ASSISTANCE_INFORMATION_FEEDBACK: u8 = 10;

pub const ID_E_CID_MEASUREMENT_FAILURE_INDICATION: u8 = 3;

pub const ID_E_CID_MEASUREMENT_INITIATION: u8 = 2;

pub const ID_E_CID_MEASUREMENT_REPORT: u8 = 4;

pub const ID_E_CID_MEASUREMENT_TERMINATION: u8 = 5;

pub const ID_E_NB_UE_MEASUREMENT_ID: u16 = 6;

pub const ID_ERROR_INDICATION: u8 = 0;

pub const ID_O_TDOA_INFORMATION_EXCHANGE: u8 = 6;

pub const ID_PRIVATE_MESSAGE: u8 = 1;

pub const ID_U_TDOA_INFORMATION_EXCHANGE: u8 = 7;

pub const ID_U_TDOA_INFORMATION_UPDATE: u8 = 8;

pub const MAX_CELL_REPORT: i64 = 9;

pub const MAX_CELLINE_NB: i64 = 256;

pub const MAX_CELLINE_NB_EXT: i64 = 3840;

pub const MAX_GERAN_MEAS: i64 = 8;

pub const MAX_MBSFN_ALLOCATIONS: i64 = 8;

pub const MAX_N_RMEAS: i64 = 32;

pub const MAX_NO_MEAS: i64 = 63;

pub const MAX_NR_OF_ERRORS: i64 = 256;

pub const MAX_NR_OF_POS_SI_BS: i64 = 32;

pub const MAX_NR_OF_POS_S_IMESSAGE: i64 = 32;

pub const MAX_NR_OF_SEGMENTS: i64 = 64;

pub const MAX_PRIVATE_I_ES: i64 = 65535;

pub const MAX_PROTOCOL_EXTENSIONS: i64 = 65535;

pub const MAX_PROTOCOL_I_ES: i64 = 65535;

pub const MAX_RESULTS_PER_SSB_INDEX: i64 = 64;

pub const MAX_SERV_CELL: i64 = 5;

pub const MAX_UTRAN_MEAS: i64 = 8;

pub const MAX_WLA_NCHANNELS: i64 = 16;

pub const MAXNO_ASSIST_INFO_FAILURE_LIST_ITEMS: i64 = 32;

pub const MAXNO_FREQ_HOPPING_BANDS_MINUS_ONE: i64 = 7;

pub const MAXNO_OTDO_ATYPES: i64 = 63;

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "63")]
pub struct Add_OTDOACell_Information(pub Vec<OTDOACell_Information_Item>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "3840"
)]
pub struct Add_OTDOACells(pub Vec<Add_OTDOACells_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Assistance_Information {
    pub system_information: SystemInformation,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Assistance_InformationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AssistanceInformationControl {
    pub protocol_i_es: AssistanceInformationControlProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AssistanceInformationFailureList(pub Vec<AssistanceInformationFailureList_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AssistanceInformationFeedback {
    pub protocol_i_es: AssistanceInformationFeedbackProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct AssistanceInformationMetaData {
    #[asn(optional_idx = 0)]
    pub encrypted: Option<AssistanceInformationMetaDataEncrypted>,
    #[asn(optional_idx = 1)]
    pub gnssid: Option<AssistanceInformationMetaDataGNSSID>,
    #[asn(optional_idx = 2)]
    pub sbasid: Option<AssistanceInformationMetaDataSBASID>,
    #[asn(optional_idx = 3)]
    pub ie_extensions: Option<AssistanceInformationMetaDataIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023", extensible = true)]
pub struct BCCH(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct BSSID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum BitmapsforNPRS {
    #[asn(key = 0, extended = false)]
    Ten(BitmapsforNPRS_ten),
    #[asn(key = 1, extended = false)]
    Forty(BitmapsforNPRS_forty),
    #[asn(key = 0, extended = true)]
    Ten_tdd(BitmapsforNPRS_ten_tdd),
    #[asn(key = 1, extended = true)]
    Forty_tdd(BitmapsforNPRS_forty_tdd),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Broadcast(pub u8);
impl Broadcast {
    pub const START: u8 = 0u8;
    pub const STOP: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct BroadcastPeriodicity(pub u8);
impl BroadcastPeriodicity {
    pub const MS80: u8 = 0u8;
    pub const MS160: u8 = 1u8;
    pub const MS320: u8 = 2u8;
    pub const MS640: u8 = 3u8;
    pub const MS1280: u8 = 4u8;
    pub const MS2560: u8 = 5u8;
    pub const MS5120: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct CPLength(pub u8);
impl CPLength {
    pub const NORMAL: u8 = 0u8;
    pub const EXTENDED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum Cause {
    #[asn(key = 0, extended = false)]
    RadioNetwork(CauseRadioNetwork),
    #[asn(key = 1, extended = false)]
    Protocol(CauseProtocol),
    #[asn(key = 2, extended = false)]
    Misc(CauseMisc),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct CauseMisc(pub u8);
impl CauseMisc {
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
    pub const UNSPECIFIED: u8 = 5u8;
    pub const ABSTRACT_SYNTAX_ERROR_FALSELY_CONSTRUCTED_MESSAGE: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct CauseRadioNetwork(pub u8);
impl CauseRadioNetwork {
    pub const UNSPECIFIED: u8 = 0u8;
    pub const REQUESTED_ITEM_NOT_SUPPORTED: u8 = 1u8;
    pub const REQUESTED_ITEM_TEMPORARILY_NOT_AVAILABLE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255", extensible = true)]
pub struct Cell_Portion_ID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Criticality(pub u8);
impl Criticality {
    pub const REJECT: u8 = 0u8;
    pub const IGNORE: u8 = 1u8;
    pub const NOTIFY: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct CriticalityDiagnostics {
    #[asn(optional_idx = 0)]
    pub procedure_code: Option<ProcedureCode>,
    #[asn(optional_idx = 1)]
    pub triggering_message: Option<TriggeringMessage>,
    #[asn(optional_idx = 2)]
    pub procedure_criticality: Option<Criticality>,
    #[asn(optional_idx = 3)]
    pub lppatransaction_id: Option<LPPATransactionID>,
    #[asn(optional_idx = 4)]
    pub i_es_criticality_diagnostics: Option<CriticalityDiagnostics_IE_List>,
    #[asn(optional_idx = 5)]
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
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct DL_Bandwidth(pub u8);
impl DL_Bandwidth {
    pub const BW6: u8 = 0u8;
    pub const BW15: u8 = 1u8;
    pub const BW25: u8 = 2u8;
    pub const BW50: u8 = 3u8;
    pub const BW75: u8 = 4u8;
    pub const BW100: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct E_CID_MeasurementResult {
    pub serving_cell_id: ECGI,
    pub serving_cell_tac: TAC,
    #[asn(optional_idx = 0)]
    pub e_utran_access_point_position: Option<E_UTRANAccessPointPosition>,
    #[asn(optional_idx = 1)]
    pub measured_results: Option<MeasuredResults>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_CIDMeasurementFailureIndication {
    pub protocol_i_es: E_CIDMeasurementFailureIndicationProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_CIDMeasurementInitiationFailure {
    pub protocol_i_es: E_CIDMeasurementInitiationFailureProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_CIDMeasurementInitiationRequest {
    pub protocol_i_es: E_CIDMeasurementInitiationRequestProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_CIDMeasurementInitiationResponse {
    pub protocol_i_es: E_CIDMeasurementInitiationResponseProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_CIDMeasurementReport {
    pub protocol_i_es: E_CIDMeasurementReportProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_CIDMeasurementTerminationCommand {
    pub protocol_i_es: E_CIDMeasurementTerminationCommandProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct E_UTRANAccessPointPosition {
    pub latitude_sign: E_UTRANAccessPointPositionLatitudeSign,
    pub latitude: E_UTRANAccessPointPositionLatitude,
    pub longitude: E_UTRANAccessPointPositionLongitude,
    pub direction_of_altitude: E_UTRANAccessPointPositionDirectionOfAltitude,
    pub altitude: E_UTRANAccessPointPositionAltitude,
    pub uncertainty_semi_major: E_UTRANAccessPointPositionUncertaintySemi_major,
    pub uncertainty_semi_minor: E_UTRANAccessPointPositionUncertaintySemi_minor,
    pub orientation_of_major_axis: E_UTRANAccessPointPositionOrientationOfMajorAxis,
    pub uncertainty_altitude: E_UTRANAccessPointPositionUncertaintyAltitude,
    pub confidence: E_UTRANAccessPointPositionConfidence,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535", extensible = true)]
pub struct EARFCN(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ECGI {
    pub plmn_identity: PLMN_Identity,
    pub eutra_ncell_identifier: EUTRANCellIdentifier,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ECGIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct EUTRANCellIdentifier(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ErrorIndication {
    pub protocol_i_es: ErrorIndicationProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct HESSID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InitiatingMessage {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub lppatransaction_id: LPPATransactionID,
    pub value: InitiatingMessageValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum InterRATMeasuredResultsValue {
    #[asn(key = 0, extended = false)]
    ResultGERAN(ResultGERAN),
    #[asn(key = 1, extended = false)]
    ResultUTRAN(ResultUTRAN),
    #[asn(key = 0, extended = true)]
    ResultNR(ResultNR),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "63")]
pub struct InterRATMeasurementQuantities(pub Vec<InterRATMeasurementQuantities_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct InterRATMeasurementQuantities_Item {
    pub inter_rat_measurement_quantities_value: InterRATMeasurementQuantitiesValue,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<InterRATMeasurementQuantities_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct InterRATMeasurementQuantitiesValue(pub u8);
impl InterRATMeasurementQuantitiesValue {
    pub const GERAN: u8 = 0u8;
    pub const UTRAN: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "63")]
pub struct InterRATMeasurementResult(pub Vec<InterRATMeasuredResultsValue>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum LPPA_PDU {
    #[asn(key = 0, extended = false)]
    InitiatingMessage(InitiatingMessage),
    #[asn(key = 1, extended = false)]
    SuccessfulOutcome(SuccessfulOutcome),
    #[asn(key = 2, extended = false)]
    UnsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct LPPATransactionID(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct MBSFNsubframeConfiguration(pub Vec<MBSFNsubframeConfigurationValue>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MBSFNsubframeConfigurationValue {
    pub radioframe_allocation_period: MBSFNsubframeConfigurationValueRadioframeAllocationPeriod,
    pub radioframe_allocation_offset: MBSFNsubframeConfigurationValueRadioframeAllocationOffset,
    pub subframe_allocation: Subframeallocation,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "63")]
pub struct MeasuredResults(pub Vec<MeasuredResultsValue>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = true)]
pub enum MeasuredResultsValue {
    #[asn(key = 0, extended = false)]
    ValueAngleOfArrival(MeasuredResultsValue_valueAngleOfArrival),
    #[asn(key = 1, extended = false)]
    ValueTimingAdvanceType1(MeasuredResultsValue_valueTimingAdvanceType1),
    #[asn(key = 2, extended = false)]
    ValueTimingAdvanceType2(MeasuredResultsValue_valueTimingAdvanceType2),
    #[asn(key = 3, extended = false)]
    ResultRSRP(ResultRSRP),
    #[asn(key = 4, extended = false)]
    ResultRSRQ(ResultRSRQ),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "15", extensible = true)]
pub struct Measurement_ID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "12")]
pub struct MeasurementPeriodicity(pub u8);
impl MeasurementPeriodicity {
    pub const MS120: u8 = 0u8;
    pub const MS240: u8 = 1u8;
    pub const MS480: u8 = 2u8;
    pub const MS640: u8 = 3u8;
    pub const MS1024: u8 = 4u8;
    pub const MS2048: u8 = 5u8;
    pub const MS5120: u8 = 6u8;
    pub const MS10240: u8 = 7u8;
    pub const MIN1: u8 = 8u8;
    pub const MIN6: u8 = 9u8;
    pub const MIN12: u8 = 10u8;
    pub const MIN30: u8 = 11u8;
    pub const MIN60: u8 = 12u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "63")]
pub struct MeasurementQuantities(pub Vec<MeasurementQuantities_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MeasurementQuantities_Item {
    pub measurement_quantities_value: MeasurementQuantitiesValue,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<MeasurementQuantities_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct MeasurementQuantitiesValue(pub u8);
impl MeasurementQuantitiesValue {
    pub const CELL_ID: u8 = 0u8;
    pub const ANGLE_OF_ARRIVAL: u8 = 1u8;
    pub const TIMING_ADVANCE_TYPE1: u8 = 2u8;
    pub const TIMING_ADVANCE_TYPE2: u8 = 3u8;
    pub const R_SRP: u8 = 4u8;
    pub const R_SRQ: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NPRSConfiguration {
    #[asn(optional_idx = 0)]
    pub nprs_subframe_part_a: Option<NPRSSubframePartA>,
    #[asn(optional_idx = 1)]
    pub nprs_subframe_part_b: Option<NPRSSubframePartB>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum NPRSMutingConfiguration {
    #[asn(key = 0, extended = false)]
    Two(NPRSMutingConfiguration_two),
    #[asn(key = 1, extended = false)]
    Four(NPRSMutingConfiguration_four),
    #[asn(key = 2, extended = false)]
    Eight(NPRSMutingConfiguration_eight),
    #[asn(key = 3, extended = false)]
    Sixteen(NPRSMutingConfiguration_sixteen),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "174", extensible = true)]
pub struct NPRSSequenceInfo(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NPRSSubframePartA {
    pub bitmapsfor_nprs: BitmapsforNPRS,
    #[asn(optional_idx = 0)]
    pub nprs_muting_configuration: Option<NPRSMutingConfiguration>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NPRSSubframePartB {
    pub numberof_nprs_one_occasion: NPRSSubframePartBNumberofNPRSOneOccasion,
    pub periodicityof_nprs: NPRSSubframePartBPeriodicityofNPRS,
    pub startingsubframeoffset: NPRSSubframePartBStartingsubframeoffset,
    #[asn(optional_idx = 0)]
    pub nprs_muting_configuration: Option<NPRSMutingConfiguration>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_CGI {
    pub plmn_identity: PLMN_Identity,
    pub nr_cell_identity: NRCellIdentity,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<NR_CGIIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct NRARFCN(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "36", sz_ub = "36")]
pub struct NRCellIdentity(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1007")]
pub struct NRPCI(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15", extensible = true)]
pub struct NarrowBandIndex(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NumberOfAntennaPorts(pub u8);
impl NumberOfAntennaPorts {
    pub const N1_OR_N2: u8 = 0u8;
    pub const N4: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NumberOfDlFrames(pub u8);
impl NumberOfDlFrames {
    pub const SF1: u8 = 0u8;
    pub const SF2: u8 = 1u8;
    pub const SF4: u8 = 2u8;
    pub const SF6: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "160", extensible = true)]
pub struct NumberOfDlFrames_Extended(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NumberOfFrequencyHoppingBands(pub u8);
impl NumberOfFrequencyHoppingBands {
    pub const TWOBANDS: u8 = 0u8;
    pub const FOURBANDS: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "9")]
pub struct OTDOA_Information_Item(pub u8);
impl OTDOA_Information_Item {
    pub const PCI: u8 = 0u8;
    pub const CELLID: u8 = 1u8;
    pub const TAC: u8 = 2u8;
    pub const EARFCN: u8 = 3u8;
    pub const PRS_BANDWIDTH: u8 = 4u8;
    pub const PRS_CONFIG_INDEX: u8 = 5u8;
    pub const CP_LENGTH: u8 = 6u8;
    pub const NO_DL_FRAMES: u8 = 7u8;
    pub const NO_ANTENNA_PORTS: u8 = 8u8;
    pub const S_FN_INIT_TIME: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "63")]
pub struct OTDOA_Information_Type(pub Vec<OTDOA_Information_Type_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct OTDOA_Information_Type_Item {
    pub otdoa_information_type_item: OTDOA_Information_Item,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<OTDOA_Information_Type_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "63")]
pub struct OTDOACell_Information(pub Vec<OTDOACell_Information_Item>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "10", extensible = true)]
pub enum OTDOACell_Information_Item {
    #[asn(key = 0, extended = false)]
    PCI(PCI),
    #[asn(key = 1, extended = false)]
    CellId(ECGI),
    #[asn(key = 2, extended = false)]
    TAC(TAC),
    #[asn(key = 3, extended = false)]
    EARFCN(EARFCN),
    #[asn(key = 4, extended = false)]
    PRS_Bandwidth(PRS_Bandwidth),
    #[asn(key = 5, extended = false)]
    PRS_ConfigurationIndex(PRS_Configuration_Index),
    #[asn(key = 6, extended = false)]
    CPLength(CPLength),
    #[asn(key = 7, extended = false)]
    NumberOfDlFrames(NumberOfDlFrames),
    #[asn(key = 8, extended = false)]
    NumberOfAntennaPorts(NumberOfAntennaPorts),
    #[asn(key = 9, extended = false)]
    SFNInitialisationTime(SFNInitialisationTime),
    #[asn(key = 10, extended = false)]
    E_UTRANAccessPointPosition(E_UTRANAccessPointPosition),
    #[asn(key = 0, extended = true)]
    PRSMutingConfiguration(PRSMutingConfiguration),
    #[asn(key = 1, extended = true)]
    Prsid(PRS_ID),
    #[asn(key = 2, extended = true)]
    Tpid(TP_ID),
    #[asn(key = 3, extended = true)]
    TpType(TP_Type),
    #[asn(key = 4, extended = true)]
    NumberOfDlFrames_Extended(NumberOfDlFrames_Extended),
    #[asn(key = 5, extended = true)]
    CrsCPlength(CPLength),
    #[asn(key = 6, extended = true)]
    MBSFNsubframeConfiguration(MBSFNsubframeConfiguration),
    #[asn(key = 7, extended = true)]
    NPRSConfiguration(NPRSConfiguration),
    #[asn(key = 8, extended = true)]
    OffsetNBChanneltoEARFCN(OffsetNBChanneltoEARFCN),
    #[asn(key = 9, extended = true)]
    OperationModeInfo(OperationModeInfo),
    #[asn(key = 10, extended = true)]
    NPRS_ID(OTDOACell_Information_Item_nPRS_ID),
    #[asn(key = 11, extended = true)]
    DL_Bandwidth(DL_Bandwidth),
    #[asn(key = 12, extended = true)]
    PRSOccasionGroup(PRSOccasionGroup),
    #[asn(key = 13, extended = true)]
    PRSFreqHoppingConfig(PRSFrequencyHoppingConfiguration),
    #[asn(key = 14, extended = true)]
    RepetitionNumberofSIB1_NB(RepetitionNumberofSIB1_NB),
    #[asn(key = 15, extended = true)]
    NPRSSequenceInfo(NPRSSequenceInfo),
    #[asn(key = 16, extended = true)]
    NPRSType2(NPRSConfiguration),
    #[asn(key = 17, extended = true)]
    TddConfiguration(TDDConfiguration),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct OTDOACells(pub Vec<OTDOACells_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OTDOAInformationFailure {
    pub protocol_i_es: OTDOAInformationFailureProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OTDOAInformationRequest {
    pub protocol_i_es: OTDOAInformationRequestProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OTDOAInformationResponse {
    pub protocol_i_es: OTDOAInformationResponseProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "20")]
pub struct OffsetNBChanneltoEARFCN(pub u8);
impl OffsetNBChanneltoEARFCN {
    pub const MINUS_TEN: u8 = 0u8;
    pub const MINUS_NINE: u8 = 1u8;
    pub const MINUS_EIGHT: u8 = 2u8;
    pub const MINUS_SEVEN: u8 = 3u8;
    pub const MINUS_SIX: u8 = 4u8;
    pub const MINUS_FIVE: u8 = 5u8;
    pub const MINUS_FOUR: u8 = 6u8;
    pub const MINUS_THREE: u8 = 7u8;
    pub const MINUS_TWO: u8 = 8u8;
    pub const MINUS_ONE: u8 = 9u8;
    pub const MINUS_ZERO_DOT_FIVE: u8 = 10u8;
    pub const ZERO: u8 = 11u8;
    pub const ONE: u8 = 12u8;
    pub const TWO: u8 = 13u8;
    pub const THREE: u8 = 14u8;
    pub const FOUR: u8 = 15u8;
    pub const FIVE: u8 = 16u8;
    pub const SIX: u8 = 17u8;
    pub const SEVEN: u8 = 18u8;
    pub const EIGHT: u8 = 19u8;
    pub const NINE: u8 = 20u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct OperationModeInfo(pub u8);
impl OperationModeInfo {
    pub const INBAND: u8 = 0u8;
    pub const GUARDBAND: u8 = 1u8;
    pub const STANDALONE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Outcome(pub u8);
impl Outcome {
    pub const FAILED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503", extensible = true)]
pub struct PCI(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct PLMN_Identity(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct PRS_Bandwidth(pub u8);
impl PRS_Bandwidth {
    pub const BW6: u8 = 0u8;
    pub const BW15: u8 = 1u8;
    pub const BW25: u8 = 2u8;
    pub const BW50: u8 = 3u8;
    pub const BW75: u8 = 4u8;
    pub const BW100: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct PRS_Configuration_Index(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct PRS_ID(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PRSFrequencyHoppingConfiguration {
    pub no_of_freq_hopping_bands: NumberOfFrequencyHoppingBands,
    pub band_positions: PRSFrequencyHoppingConfigurationBandPositions,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PRSFrequencyHoppingConfigurationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum PRSMutingConfiguration {
    #[asn(key = 0, extended = false)]
    Two(PRSMutingConfiguration_two),
    #[asn(key = 1, extended = false)]
    Four(PRSMutingConfiguration_four),
    #[asn(key = 2, extended = false)]
    Eight(PRSMutingConfiguration_eight),
    #[asn(key = 3, extended = false)]
    Sixteen(PRSMutingConfiguration_sixteen),
    #[asn(key = 0, extended = true)]
    Thirty_two(PRSMutingConfiguration_thirty_two),
    #[asn(key = 1, extended = true)]
    Sixty_four(PRSMutingConfiguration_sixty_four),
    #[asn(key = 2, extended = true)]
    One_hundred_and_twenty_eight(PRSMutingConfiguration_one_hundred_and_twenty_eight),
    #[asn(key = 3, extended = true)]
    Two_hundred_and_fifty_six(PRSMutingConfiguration_two_hundred_and_fifty_six),
    #[asn(key = 4, extended = true)]
    Five_hundred_and_twelve(PRSMutingConfiguration_five_hundred_and_twelve),
    #[asn(key = 5, extended = true)]
    One_thousand_and_twenty_four(PRSMutingConfiguration_one_thousand_and_twenty_four),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct PRSOccasionGroup(pub u8);
impl PRSOccasionGroup {
    pub const OG2: u8 = 0u8;
    pub const OG4: u8 = 1u8;
    pub const OG8: u8 = 2u8;
    pub const OG16: u8 = 3u8;
    pub const OG32: u8 = 4u8;
    pub const OG64: u8 = 5u8;
    pub const OG128: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63", extensible = true)]
pub struct PhysCellIDGERAN(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511", extensible = true)]
pub struct PhysCellIDUTRA_FDD(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127", extensible = true)]
pub struct PhysCellIDUTRA_TDD(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct PosSIB_Segments(pub Vec<PosSIB_Segments_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "26")]
pub struct PosSIB_Type(pub u8);
impl PosSIB_Type {
    pub const POS_SIB_TYPE1_1: u8 = 0u8;
    pub const POS_SIB_TYPE1_2: u8 = 1u8;
    pub const POS_SIB_TYPE1_3: u8 = 2u8;
    pub const POS_SIB_TYPE1_4: u8 = 3u8;
    pub const POS_SIB_TYPE1_5: u8 = 4u8;
    pub const POS_SIB_TYPE1_6: u8 = 5u8;
    pub const POS_SIB_TYPE1_7: u8 = 6u8;
    pub const POS_SIB_TYPE2_1: u8 = 7u8;
    pub const POS_SIB_TYPE2_2: u8 = 8u8;
    pub const POS_SIB_TYPE2_3: u8 = 9u8;
    pub const POS_SIB_TYPE2_4: u8 = 10u8;
    pub const POS_SIB_TYPE2_5: u8 = 11u8;
    pub const POS_SIB_TYPE2_6: u8 = 12u8;
    pub const POS_SIB_TYPE2_7: u8 = 13u8;
    pub const POS_SIB_TYPE2_8: u8 = 14u8;
    pub const POS_SIB_TYPE2_9: u8 = 15u8;
    pub const POS_SIB_TYPE2_10: u8 = 16u8;
    pub const POS_SIB_TYPE2_11: u8 = 17u8;
    pub const POS_SIB_TYPE2_12: u8 = 18u8;
    pub const POS_SIB_TYPE2_13: u8 = 19u8;
    pub const POS_SIB_TYPE2_14: u8 = 20u8;
    pub const POS_SIB_TYPE2_15: u8 = 21u8;
    pub const POS_SIB_TYPE2_16: u8 = 22u8;
    pub const POS_SIB_TYPE2_17: u8 = 23u8;
    pub const POS_SIB_TYPE2_18: u8 = 24u8;
    pub const POS_SIB_TYPE2_19: u8 = 25u8;
    pub const POS_SIB_TYPE3_1: u8 = 26u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct PosSIBs(pub Vec<PosSIBs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Presence(pub u8);
impl Presence {
    pub const OPTIONAL: u8 = 0u8;
    pub const CONDITIONAL: u8 = 1u8;
    pub const MANDATORY: u8 = 2u8;
}

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
#[asn(type = "INTEGER", lb = "0", ub = "63", extensible = true)]
pub struct RSSI(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct RepetitionNumberofSIB1_NB(pub u8);
impl RepetitionNumberofSIB1_NB {
    pub const R4: u8 = 0u8;
    pub const R8: u8 = 1u8;
    pub const R16: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct ReportCharacteristics(pub u8);
impl ReportCharacteristics {
    pub const ON_DEMAND: u8 = 0u8;
    pub const PERIODIC: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RequestedSRSTransmissionCharacteristics {
    pub number_of_transmissions: RequestedSRSTransmissionCharacteristicsNumberOfTransmissions,
    pub bandwidth: RequestedSRSTransmissionCharacteristicsBandwidth,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct ResultGERAN(pub Vec<ResultGERAN_Item>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResultGERAN_Item {
    pub bcch: BCCH,
    pub phys_cell_idgeran: PhysCellIDGERAN,
    pub rssi: RSSI,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<ResultGERAN_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct ResultNR(pub Vec<ResultNR_Item>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ResultNR_Item {
    pub nrarfcn: NRARFCN,
    pub nrpci: NRPCI,
    #[asn(optional_idx = 0)]
    pub ss_nrrsrp: Option<SS_NRRSRP>,
    #[asn(optional_idx = 1)]
    pub ss_nrrsrq: Option<SS_NRRSRQ>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ResultNR_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "9")]
pub struct ResultRSRP(pub Vec<ResultRSRP_Item>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ResultRSRP_Item {
    pub pci: PCI,
    pub earfcn: EARFCN,
    #[asn(optional_idx = 0)]
    pub ecgi: Option<ECGI>,
    pub value_rsrp: ValueRSRP,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ResultRSRP_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "9")]
pub struct ResultRSRQ(pub Vec<ResultRSRQ_Item>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ResultRSRQ_Item {
    pub pci: PCI,
    pub earfcn: EARFCN,
    #[asn(optional_idx = 0)]
    pub ecgi: Option<ECGI>,
    pub value_rsrq: ValueRSRQ,
    #[asn(optional_idx = 1)]
    pub ie_extensions: Option<ResultRSRQ_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct ResultUTRAN(pub Vec<ResultUTRAN_Item>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ResultUTRAN_Item {
    pub uarfcn: UARFCN,
    pub phys_cell_idutran: ResultUTRAN_ItemPhysCellIDUTRAN,
    #[asn(optional_idx = 0)]
    pub utra_rscp: Option<UTRA_RSCP>,
    #[asn(optional_idx = 1)]
    pub utra_ec_n0: Option<UTRA_EcN0>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ResultUTRAN_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ResultsPerSSB_Index_Item {
    pub ssb_index: SSB_Index,
    #[asn(optional_idx = 0)]
    pub ss_nrrsrp_beam_value: Option<SS_NRRSRP>,
    #[asn(optional_idx = 1)]
    pub ss_nrrsrq_beam_value: Option<SS_NRRSRQ>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<ResultsPerSSB_Index_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct ResultsPerSSB_Index_List(pub Vec<ResultsPerSSB_Index_Item>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct SFNInitialisationTime(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "5")]
pub struct SRSConfigurationForAllCells(pub Vec<SRSConfigurationForOneCell>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SRSConfigurationForOneCell {
    pub pci: PCI,
    pub ul_earfcn: EARFCN,
    pub ul_bandwidth: SRSConfigurationForOneCellUl_bandwidth,
    pub ul_cyclic_prefix_length: CPLength,
    pub srs_bandwidth_config: SRSConfigurationForOneCellSrs_BandwidthConfig,
    pub srs_bandwidth: SRSConfigurationForOneCellSrs_Bandwidth,
    pub srs_antenna_port: SRSConfigurationForOneCellSrs_AntennaPort,
    pub srs_hopping_bandwidth: SRSConfigurationForOneCellSrs_HoppingBandwidth,
    pub srs_cyclic_shift: SRSConfigurationForOneCellSrs_cyclicShift,
    pub srs_config_index: SRSConfigurationForOneCellSrs_ConfigIndex,
    #[asn(optional_idx = 0)]
    pub max_up_pts: Option<SRSConfigurationForOneCellMaxUpPts>,
    pub transmission_comb: SRSConfigurationForOneCellTransmissionComb,
    pub freq_domain_position: SRSConfigurationForOneCellFreqDomainPosition,
    pub group_hopping_enabled: SRSConfigurationForOneCellGroupHoppingEnabled,
    #[asn(optional_idx = 1)]
    pub delta_ss: Option<SRSConfigurationForOneCellDeltaSS>,
    pub sfn_initialisation_time: SFNInitialisationTime,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct SS_NRRSRP(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct SS_NRRSRQ(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct SSB_Index(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "32"
)]
pub struct SSID(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum Subframeallocation {
    #[asn(key = 0, extended = false)]
    OneFrame(Subframeallocation_oneFrame),
    #[asn(key = 1, extended = false)]
    FourFrames(Subframeallocation_fourFrames),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub lppatransaction_id: LPPATransactionID,
    pub value: SuccessfulOutcomeValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct SystemInformation(pub Vec<SystemInformation_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct TAC(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TDDConfiguration {
    pub subframe_assignment: TDDConfigurationSubframeAssignment,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<TDDConfigurationIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct TP_ID(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct TP_Type(pub u8);
impl TP_Type {
    pub const PRS_ONLY_TP: u8 = 0u8;
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
#[asn(type = "INTEGER", lb = "0", ub = "16383", extensible = true)]
pub struct UARFCN(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ULConfiguration {
    pub pci: PCI,
    pub ul_earfcn: EARFCN,
    #[asn(optional_idx = 0)]
    pub timing_advance_type1: Option<ULConfigurationTimingAdvanceType1>,
    #[asn(optional_idx = 1)]
    pub timing_advance_type2: Option<ULConfigurationTimingAdvanceType2>,
    pub number_of_transmissions: ULConfigurationNumberOfTransmissions,
    pub srs_configuration: SRSConfigurationForAllCells,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UTDOAInformationFailure {
    pub protocol_i_es: UTDOAInformationFailureProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UTDOAInformationRequest {
    pub protocol_i_es: UTDOAInformationRequestProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UTDOAInformationResponse {
    pub protocol_i_es: UTDOAInformationResponseProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UTDOAInformationUpdate {
    pub protocol_i_es: UTDOAInformationUpdateProtocolIEs,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "49", extensible = true)]
pub struct UTRA_EcN0(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "-5", ub = "91", extensible = true)]
pub struct UTRA_RSCP(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UnsuccessfulOutcome {
    #[asn(key_field = true)]
    pub procedure_code: ProcedureCode,
    pub criticality: Criticality,
    pub lppatransaction_id: LPPATransactionID,
    pub value: UnsuccessfulOutcomeValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "97", extensible = true)]
pub struct ValueRSRP(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "34", extensible = true)]
pub struct ValueRSRQ(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "141", extensible = true)]
pub struct WLAN_RSSI(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct WLANBand(pub u8);
impl WLANBand {
    pub const BAND2DOT4: u8 = 0u8;
    pub const BAND5: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct WLANChannel(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct WLANChannelList(pub Vec<WLANChannel>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct WLANCountryCode(pub u8);
impl WLANCountryCode {
    pub const UNITED_STATES: u8 = 0u8;
    pub const EUROPE: u8 = 1u8;
    pub const JAPAN: u8 = 2u8;
    pub const GLOBAL: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "63")]
pub struct WLANMeasurementQuantities(pub Vec<WLANMeasurementQuantities_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WLANMeasurementQuantities_Item {
    pub wlan_measurement_quantities_value: WLANMeasurementQuantitiesValue,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<WLANMeasurementQuantities_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct WLANMeasurementQuantitiesValue(pub u8);
impl WLANMeasurementQuantitiesValue {
    pub const WLAN: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "63")]
pub struct WLANMeasurementResult(pub Vec<WLANMeasurementResult_Item>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 8)]
pub struct WLANMeasurementResult_Item {
    pub wlan_rssi: WLAN_RSSI,
    #[asn(optional_idx = 0)]
    pub ssid: Option<SSID>,
    #[asn(optional_idx = 1)]
    pub bssid: Option<BSSID>,
    #[asn(optional_idx = 2)]
    pub hessid: Option<HESSID>,
    #[asn(optional_idx = 3)]
    pub operating_class: Option<WLANOperatingClass>,
    #[asn(optional_idx = 4)]
    pub country_code: Option<WLANCountryCode>,
    #[asn(optional_idx = 5)]
    pub wlan_channel_list: Option<WLANChannelList>,
    #[asn(optional_idx = 6)]
    pub wlan_band: Option<WLANBand>,
    #[asn(optional_idx = 7)]
    pub ie_extensions: Option<WLANMeasurementResult_ItemIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct WLANOperatingClass(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Add_OTDOACells_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Add_OTDOACells_EntryIE_Extensions(pub Vec<Add_OTDOACells_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Add_OTDOACells_Entry {
    pub add_otdoa_cell_info: Add_OTDOACell_Information,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<Add_OTDOACells_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Assistance_InformationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct Assistance_InformationIE_Extensions(pub Vec<Assistance_InformationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum AssistanceInformationControlProtocolIEs_EntryValue {
    #[asn(key = 22)]
    Id_Assistance_Information(Assistance_Information),
    #[asn(key = 23)]
    Id_Broadcast(Broadcast),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceInformationControlProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AssistanceInformationControlProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AssistanceInformationControlProtocolIEs(
    pub Vec<AssistanceInformationControlProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceInformationFailureList_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssistanceInformationFailureList_EntryIE_Extensions(
    pub Vec<AssistanceInformationFailureList_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AssistanceInformationFailureList_Entry {
    pub pos_sib_type: PosSIB_Type,
    pub outcome: Outcome,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<AssistanceInformationFailureList_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum AssistanceInformationFeedbackProtocolIEs_EntryValue {
    #[asn(key = 24)]
    Id_AssistanceInformationFailureList(AssistanceInformationFailureList),
    #[asn(key = 1)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceInformationFeedbackProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: AssistanceInformationFeedbackProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct AssistanceInformationFeedbackProtocolIEs(
    pub Vec<AssistanceInformationFeedbackProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct AssistanceInformationMetaDataEncrypted(pub u8);
impl AssistanceInformationMetaDataEncrypted {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct AssistanceInformationMetaDataGNSSID(pub u8);
impl AssistanceInformationMetaDataGNSSID {
    pub const GPS: u8 = 0u8;
    pub const SBAS: u8 = 1u8;
    pub const GZSS: u8 = 2u8;
    pub const GALILEO: u8 = 3u8;
    pub const GLONASS: u8 = 4u8;
    pub const BDS: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct AssistanceInformationMetaDataSBASID(pub u8);
impl AssistanceInformationMetaDataSBASID {
    pub const WAAS: u8 = 0u8;
    pub const EGNOS: u8 = 1u8;
    pub const MSAS: u8 = 2u8;
    pub const GAGAN: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AssistanceInformationMetaDataIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct AssistanceInformationMetaDataIE_Extensions(
    pub Vec<AssistanceInformationMetaDataIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct BitmapsforNPRS_ten(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "40", sz_ub = "40")]
pub struct BitmapsforNPRS_forty(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct BitmapsforNPRS_ten_tdd(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct BitmapsforNPRS_forty_tdd(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

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
#[asn(type = "OPEN")]
pub enum E_CIDMeasurementFailureIndicationProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_Cause(Cause),
    #[asn(key = 2)]
    Id_E_SMLC_UE_Measurement_ID(Measurement_ID),
    #[asn(key = 6)]
    Id_eNB_UE_Measurement_ID(Measurement_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_CIDMeasurementFailureIndicationProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_CIDMeasurementFailureIndicationProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_CIDMeasurementFailureIndicationProtocolIEs(
    pub Vec<E_CIDMeasurementFailureIndicationProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum E_CIDMeasurementInitiationFailureProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_Cause(Cause),
    #[asn(key = 1)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 2)]
    Id_E_SMLC_UE_Measurement_ID(Measurement_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_CIDMeasurementInitiationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_CIDMeasurementInitiationFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_CIDMeasurementInitiationFailureProtocolIEs(
    pub Vec<E_CIDMeasurementInitiationFailureProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum E_CIDMeasurementInitiationRequestProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_E_SMLC_UE_Measurement_ID(Measurement_ID),
    #[asn(key = 15)]
    Id_InterRATMeasurementQuantities(InterRATMeasurementQuantities),
    #[asn(key = 4)]
    Id_MeasurementPeriodicity(MeasurementPeriodicity),
    #[asn(key = 5)]
    Id_MeasurementQuantities(MeasurementQuantities),
    #[asn(key = 3)]
    Id_ReportCharacteristics(ReportCharacteristics),
    #[asn(key = 19)]
    Id_WLANMeasurementQuantities(WLANMeasurementQuantities),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_CIDMeasurementInitiationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_CIDMeasurementInitiationRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_CIDMeasurementInitiationRequestProtocolIEs(
    pub Vec<E_CIDMeasurementInitiationRequestProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum E_CIDMeasurementInitiationResponseProtocolIEs_EntryValue {
    #[asn(key = 14)]
    Id_Cell_Portion_ID(Cell_Portion_ID),
    #[asn(key = 1)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 7)]
    Id_E_CID_MeasurementResult(E_CID_MeasurementResult),
    #[asn(key = 2)]
    Id_E_SMLC_UE_Measurement_ID(Measurement_ID),
    #[asn(key = 17)]
    Id_InterRATMeasurementResult(InterRATMeasurementResult),
    #[asn(key = 21)]
    Id_WLANMeasurementResult(WLANMeasurementResult),
    #[asn(key = 6)]
    Id_eNB_UE_Measurement_ID(Measurement_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_CIDMeasurementInitiationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_CIDMeasurementInitiationResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_CIDMeasurementInitiationResponseProtocolIEs(
    pub Vec<E_CIDMeasurementInitiationResponseProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum E_CIDMeasurementReportProtocolIEs_EntryValue {
    #[asn(key = 14)]
    Id_Cell_Portion_ID(Cell_Portion_ID),
    #[asn(key = 7)]
    Id_E_CID_MeasurementResult(E_CID_MeasurementResult),
    #[asn(key = 2)]
    Id_E_SMLC_UE_Measurement_ID(Measurement_ID),
    #[asn(key = 6)]
    Id_eNB_UE_Measurement_ID(Measurement_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_CIDMeasurementReportProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_CIDMeasurementReportProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_CIDMeasurementReportProtocolIEs(pub Vec<E_CIDMeasurementReportProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum E_CIDMeasurementTerminationCommandProtocolIEs_EntryValue {
    #[asn(key = 2)]
    Id_E_SMLC_UE_Measurement_ID(Measurement_ID),
    #[asn(key = 6)]
    Id_eNB_UE_Measurement_ID(Measurement_ID),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct E_CIDMeasurementTerminationCommandProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: E_CIDMeasurementTerminationCommandProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct E_CIDMeasurementTerminationCommandProtocolIEs(
    pub Vec<E_CIDMeasurementTerminationCommandProtocolIEs_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct E_UTRANAccessPointPositionLatitudeSign(pub u8);
impl E_UTRANAccessPointPositionLatitudeSign {
    pub const NORTH: u8 = 0u8;
    pub const SOUTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct E_UTRANAccessPointPositionLatitude(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct E_UTRANAccessPointPositionLongitude(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct E_UTRANAccessPointPositionDirectionOfAltitude(pub u8);
impl E_UTRANAccessPointPositionDirectionOfAltitude {
    pub const HEIGHT: u8 = 0u8;
    pub const DEPTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct E_UTRANAccessPointPositionAltitude(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct E_UTRANAccessPointPositionUncertaintySemi_major(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct E_UTRANAccessPointPositionUncertaintySemi_minor(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct E_UTRANAccessPointPositionOrientationOfMajorAxis(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct E_UTRANAccessPointPositionUncertaintyAltitude(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct E_UTRANAccessPointPositionConfidence(pub u8);

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
#[asn(type = "OPEN")]
pub enum ErrorIndicationProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_Cause(Cause),
    #[asn(key = 1)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
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
#[asn(type = "OPEN")]
pub enum InitiatingMessageValue {
    #[asn(key = 9)]
    Id_assistanceInformationControl(AssistanceInformationControl),
    #[asn(key = 10)]
    Id_assistanceInformationFeedback(AssistanceInformationFeedback),
    #[asn(key = 3)]
    Id_e_CIDMeasurementFailureIndication(E_CIDMeasurementFailureIndication),
    #[asn(key = 2)]
    Id_e_CIDMeasurementInitiation(E_CIDMeasurementInitiationRequest),
    #[asn(key = 4)]
    Id_e_CIDMeasurementReport(E_CIDMeasurementReport),
    #[asn(key = 5)]
    Id_e_CIDMeasurementTermination(E_CIDMeasurementTerminationCommand),
    #[asn(key = 0)]
    Id_errorIndication(ErrorIndication),
    #[asn(key = 6)]
    Id_oTDOAInformationExchange(OTDOAInformationRequest),
    #[asn(key = 1)]
    Id_privateMessage(PrivateMessage),
    #[asn(key = 7)]
    Id_uTDOAInformationExchange(UTDOAInformationRequest),
    #[asn(key = 8)]
    Id_uTDOAInformationUpdate(UTDOAInformationUpdate),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum InterRATMeasurementQuantities_EntryValue {
    #[asn(key = 16)]
    Id_InterRATMeasurementQuantities_Item(InterRATMeasurementQuantities_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterRATMeasurementQuantities_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: InterRATMeasurementQuantities_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct InterRATMeasurementQuantities_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct InterRATMeasurementQuantities_ItemIE_Extensions(
    pub Vec<InterRATMeasurementQuantities_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct MBSFNsubframeConfigurationValueRadioframeAllocationPeriod(pub u8);
impl MBSFNsubframeConfigurationValueRadioframeAllocationPeriod {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct MBSFNsubframeConfigurationValueRadioframeAllocationOffset(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "719")]
pub struct MeasuredResultsValue_valueAngleOfArrival(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7690")]
pub struct MeasuredResultsValue_valueTimingAdvanceType1(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7690")]
pub struct MeasuredResultsValue_valueTimingAdvanceType2(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum MeasurementQuantities_EntryValue {
    #[asn(key = 11)]
    Id_MeasurementQuantities_Item(MeasurementQuantities_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MeasurementQuantities_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: MeasurementQuantities_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MeasurementQuantities_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct MeasurementQuantities_ItemIE_Extensions(
    pub Vec<MeasurementQuantities_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct NPRSMutingConfiguration_two(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct NPRSMutingConfiguration_four(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct NPRSMutingConfiguration_eight(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct NPRSMutingConfiguration_sixteen(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct NPRSSubframePartBNumberofNPRSOneOccasion(pub u8);
impl NPRSSubframePartBNumberofNPRSOneOccasion {
    pub const SF10: u8 = 0u8;
    pub const SF20: u8 = 1u8;
    pub const SF40: u8 = 2u8;
    pub const SF80: u8 = 3u8;
    pub const SF160: u8 = 4u8;
    pub const SF320: u8 = 5u8;
    pub const SF640: u8 = 6u8;
    pub const SF1280: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NPRSSubframePartBPeriodicityofNPRS(pub u8);
impl NPRSSubframePartBPeriodicityofNPRS {
    pub const SF160: u8 = 0u8;
    pub const SF320: u8 = 1u8;
    pub const SF640: u8 = 2u8;
    pub const SF1280: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct NPRSSubframePartBStartingsubframeoffset(pub u8);
impl NPRSSubframePartBStartingsubframeoffset {
    pub const ZERO: u8 = 0u8;
    pub const ONE_EIGHTH: u8 = 1u8;
    pub const TWO_EIGHTHS: u8 = 2u8;
    pub const THREE_EIGHTHS: u8 = 3u8;
    pub const FOUR_EIGHTHS: u8 = 4u8;
    pub const FIVE_EIGHTHS: u8 = 5u8;
    pub const SIX_EIGHTHS: u8 = 6u8;
    pub const SEVEN_EIGHTHS: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_CGIIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct NR_CGIIE_Extensions(pub Vec<NR_CGIIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum OTDOA_Information_Type_EntryValue {
    #[asn(key = 10)]
    Id_OTDOA_Information_Type_Item(OTDOA_Information_Type_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OTDOA_Information_Type_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OTDOA_Information_Type_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OTDOA_Information_Type_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct OTDOA_Information_Type_ItemIE_Extensions(
    pub Vec<OTDOA_Information_Type_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095", extensible = true)]
pub struct OTDOACell_Information_Item_nPRS_ID(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OTDOACells_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct OTDOACells_EntryIE_Extensions(pub Vec<OTDOACells_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct OTDOACells_Entry {
    pub otdoa_cell_info: OTDOACell_Information,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<OTDOACells_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum OTDOAInformationFailureProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_Cause(Cause),
    #[asn(key = 1)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OTDOAInformationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OTDOAInformationFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OTDOAInformationFailureProtocolIEs(pub Vec<OTDOAInformationFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum OTDOAInformationRequestProtocolIEs_EntryValue {
    #[asn(key = 9)]
    Id_OTDOA_Information_Type_Group(OTDOA_Information_Type),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OTDOAInformationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OTDOAInformationRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OTDOAInformationRequestProtocolIEs(pub Vec<OTDOAInformationRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum OTDOAInformationResponseProtocolIEs_EntryValue {
    #[asn(key = 18)]
    Id_AddOTDOACells(Add_OTDOACells),
    #[asn(key = 1)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 8)]
    Id_OTDOACells(OTDOACells),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct OTDOAInformationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: OTDOAInformationResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct OTDOAInformationResponseProtocolIEs(pub Vec<OTDOAInformationResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "7")]
pub struct PRSFrequencyHoppingConfigurationBandPositions(pub Vec<NarrowBandIndex>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PRSFrequencyHoppingConfigurationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PRSFrequencyHoppingConfigurationIE_Extensions(
    pub Vec<PRSFrequencyHoppingConfigurationIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct PRSMutingConfiguration_two(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct PRSMutingConfiguration_four(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct PRSMutingConfiguration_eight(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct PRSMutingConfiguration_sixteen(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct PRSMutingConfiguration_thirty_two(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct PRSMutingConfiguration_sixty_four(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "128",
    sz_ub = "128"
)]
pub struct PRSMutingConfiguration_one_hundred_and_twenty_eight(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "256",
    sz_ub = "256"
)]
pub struct PRSMutingConfiguration_two_hundred_and_fifty_six(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "512",
    sz_ub = "512"
)]
pub struct PRSMutingConfiguration_five_hundred_and_twelve(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "1024",
    sz_ub = "1024"
)]
pub struct PRSMutingConfiguration_one_thousand_and_twenty_four(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct PosSIB_Segments_EntryAssistanceDataSIBelement(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosSIB_Segments_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosSIB_Segments_EntryIE_Extensions(pub Vec<PosSIB_Segments_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PosSIB_Segments_Entry {
    pub assistance_data_si_belement: PosSIB_Segments_EntryAssistanceDataSIBelement,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<PosSIB_Segments_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "16", extensible = true)]
pub struct PosSIBs_EntryBroadcastPriority(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PosSIBs_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct PosSIBs_EntryIE_Extensions(pub Vec<PosSIBs_EntryIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PosSIBs_Entry {
    pub pos_sib_type: PosSIB_Type,
    pub pos_sib_segments: PosSIB_Segments,
    #[asn(optional_idx = 0)]
    pub assistance_information_meta_data: Option<AssistanceInformationMetaData>,
    #[asn(optional_idx = 1)]
    pub broadcast_priority: Option<PosSIBs_EntryBroadcastPriority>,
    #[asn(optional_idx = 2)]
    pub ie_extensions: Option<PosSIBs_EntryIE_Extensions>,
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
#[asn(type = "INTEGER", lb = "0", ub = "500", extensible = true)]
pub struct RequestedSRSTransmissionCharacteristicsNumberOfTransmissions(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "100", extensible = true)]
pub struct RequestedSRSTransmissionCharacteristicsBandwidth(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResultGERAN_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResultGERAN_ItemIE_Extensions(pub Vec<ResultGERAN_ItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum ResultNR_ItemIE_Extensions_EntryExtensionValue {
    #[asn(key = 27)]
    Id_NR_CGI(NR_CGI),
    #[asn(key = 25)]
    Id_ResultsPerSSB_Index_List(ResultsPerSSB_Index_List),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResultNR_ItemIE_Extensions_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub extension_value: ResultNR_ItemIE_Extensions_EntryExtensionValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResultNR_ItemIE_Extensions(pub Vec<ResultNR_ItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResultRSRP_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResultRSRP_ItemIE_Extensions(pub Vec<ResultRSRP_ItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResultRSRQ_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResultRSRQ_ItemIE_Extensions(pub Vec<ResultRSRQ_ItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum ResultUTRAN_ItemPhysCellIDUTRAN {
    #[asn(key = 0, extended = false)]
    PhysCellIDUTRA_FDD(PhysCellIDUTRA_FDD),
    #[asn(key = 1, extended = false)]
    PhysCellIDUTRA_TDD(PhysCellIDUTRA_TDD),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResultUTRAN_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResultUTRAN_ItemIE_Extensions(pub Vec<ResultUTRAN_ItemIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResultsPerSSB_Index_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct ResultsPerSSB_Index_ItemIE_Extensions(
    pub Vec<ResultsPerSSB_Index_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct SRSConfigurationForOneCellUl_bandwidth(pub u8);
impl SRSConfigurationForOneCellUl_bandwidth {
    pub const N6: u8 = 0u8;
    pub const N15: u8 = 1u8;
    pub const N25: u8 = 2u8;
    pub const N50: u8 = 3u8;
    pub const N75: u8 = 4u8;
    pub const N100: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct SRSConfigurationForOneCellSrs_BandwidthConfig(pub u8);
impl SRSConfigurationForOneCellSrs_BandwidthConfig {
    pub const BW0: u8 = 0u8;
    pub const BW1: u8 = 1u8;
    pub const BW2: u8 = 2u8;
    pub const BW3: u8 = 3u8;
    pub const BW4: u8 = 4u8;
    pub const BW5: u8 = 5u8;
    pub const BW6: u8 = 6u8;
    pub const BW7: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct SRSConfigurationForOneCellSrs_Bandwidth(pub u8);
impl SRSConfigurationForOneCellSrs_Bandwidth {
    pub const BW0: u8 = 0u8;
    pub const BW1: u8 = 1u8;
    pub const BW2: u8 = 2u8;
    pub const BW3: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct SRSConfigurationForOneCellSrs_AntennaPort(pub u8);
impl SRSConfigurationForOneCellSrs_AntennaPort {
    pub const AN1: u8 = 0u8;
    pub const AN2: u8 = 1u8;
    pub const AN4: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct SRSConfigurationForOneCellSrs_HoppingBandwidth(pub u8);
impl SRSConfigurationForOneCellSrs_HoppingBandwidth {
    pub const HBW0: u8 = 0u8;
    pub const HBW1: u8 = 1u8;
    pub const HBW2: u8 = 2u8;
    pub const HBW3: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct SRSConfigurationForOneCellSrs_cyclicShift(pub u8);
impl SRSConfigurationForOneCellSrs_cyclicShift {
    pub const CS0: u8 = 0u8;
    pub const CS1: u8 = 1u8;
    pub const CS2: u8 = 2u8;
    pub const CS3: u8 = 3u8;
    pub const CS4: u8 = 4u8;
    pub const CS5: u8 = 5u8;
    pub const CS6: u8 = 6u8;
    pub const CS7: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct SRSConfigurationForOneCellSrs_ConfigIndex(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct SRSConfigurationForOneCellMaxUpPts(pub u8);
impl SRSConfigurationForOneCellMaxUpPts {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct SRSConfigurationForOneCellTransmissionComb(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "23")]
pub struct SRSConfigurationForOneCellFreqDomainPosition(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SRSConfigurationForOneCellGroupHoppingEnabled(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "29")]
pub struct SRSConfigurationForOneCellDeltaSS(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct Subframeallocation_oneFrame(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "24", sz_ub = "24")]
pub struct Subframeallocation_fourFrames(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum SuccessfulOutcomeValue {
    #[asn(key = 2)]
    Id_e_CIDMeasurementInitiation(E_CIDMeasurementInitiationResponse),
    #[asn(key = 6)]
    Id_oTDOAInformationExchange(OTDOAInformationResponse),
    #[asn(key = 7)]
    Id_uTDOAInformationExchange(UTDOAInformationResponse),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SystemInformation_EntryIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct SystemInformation_EntryIE_Extensions(
    pub Vec<SystemInformation_EntryIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SystemInformation_Entry {
    pub broadcast_periodicity: BroadcastPeriodicity,
    pub pos_si_bs: PosSIBs,
    #[asn(optional_idx = 0)]
    pub ie_extensions: Option<SystemInformation_EntryIE_Extensions>,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct TDDConfigurationSubframeAssignment(pub u8);
impl TDDConfigurationSubframeAssignment {
    pub const SA0: u8 = 0u8;
    pub const SA1: u8 = 1u8;
    pub const SA2: u8 = 2u8;
    pub const SA3: u8 = 3u8;
    pub const SA4: u8 = 4u8;
    pub const SA5: u8 = 5u8;
    pub const SA6: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct TDDConfigurationIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct TDDConfigurationIE_Extensions(pub Vec<TDDConfigurationIE_Extensions_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7690")]
pub struct ULConfigurationTimingAdvanceType1(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7690")]
pub struct ULConfigurationTimingAdvanceType2(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "500", extensible = true)]
pub struct ULConfigurationNumberOfTransmissions(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UTDOAInformationFailureProtocolIEs_EntryValue {
    #[asn(key = 0)]
    Id_Cause(Cause),
    #[asn(key = 1)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UTDOAInformationFailureProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UTDOAInformationFailureProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UTDOAInformationFailureProtocolIEs(pub Vec<UTDOAInformationFailureProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UTDOAInformationRequestProtocolIEs_EntryValue {
    #[asn(key = 12)]
    Id_RequestedSRSTransmissionCharacteristics(RequestedSRSTransmissionCharacteristics),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UTDOAInformationRequestProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UTDOAInformationRequestProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UTDOAInformationRequestProtocolIEs(pub Vec<UTDOAInformationRequestProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UTDOAInformationResponseProtocolIEs_EntryValue {
    #[asn(key = 1)]
    Id_CriticalityDiagnostics(CriticalityDiagnostics),
    #[asn(key = 13)]
    Id_ULConfiguration(ULConfiguration),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UTDOAInformationResponseProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UTDOAInformationResponseProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UTDOAInformationResponseProtocolIEs(pub Vec<UTDOAInformationResponseProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UTDOAInformationUpdateProtocolIEs_EntryValue {
    #[asn(key = 13)]
    Id_ULConfiguration(ULConfiguration),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct UTDOAInformationUpdateProtocolIEs_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: UTDOAInformationUpdateProtocolIEs_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "0",
    sz_ub = "65535"
)]
pub struct UTDOAInformationUpdateProtocolIEs(pub Vec<UTDOAInformationUpdateProtocolIEs_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum UnsuccessfulOutcomeValue {
    #[asn(key = 2)]
    Id_e_CIDMeasurementInitiation(E_CIDMeasurementInitiationFailure),
    #[asn(key = 6)]
    Id_oTDOAInformationExchange(OTDOAInformationFailure),
    #[asn(key = 7)]
    Id_uTDOAInformationExchange(UTDOAInformationFailure),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "OPEN")]
pub enum WLANMeasurementQuantities_EntryValue {
    #[asn(key = 20)]
    Id_WLANMeasurementQuantities_Item(WLANMeasurementQuantities_Item),
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WLANMeasurementQuantities_Entry {
    #[asn(key_field = true)]
    pub id: ProtocolIE_ID,
    pub criticality: Criticality,
    pub value: WLANMeasurementQuantities_EntryValue,
}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WLANMeasurementQuantities_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct WLANMeasurementQuantities_ItemIE_Extensions(
    pub Vec<WLANMeasurementQuantities_ItemIE_Extensions_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct WLANMeasurementResult_ItemIE_Extensions_Entry {}

#[derive(asn1_codecs_derive :: UperCodec, serde :: Serialize, serde :: Deserialize, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "65535"
)]
pub struct WLANMeasurementResult_ItemIE_Extensions(
    pub Vec<WLANMeasurementResult_ItemIE_Extensions_Entry>,
);
