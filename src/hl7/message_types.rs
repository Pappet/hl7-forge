/// HL7 v2.x message type registry.
///
/// Maps "TYPE^EVENT" strings to human-readable descriptions and the segments
/// that are typically present in that message type.  The data is a curated
/// subset of the HL7 v2.5.1 specification and covers the most common message
/// families encountered in hospital integration engines.
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct MessageTypeInfo {
    pub description: &'static str,
    pub typical_segments: &'static [&'static str],
}

static REGISTRY: OnceLock<HashMap<&'static str, MessageTypeInfo>> = OnceLock::new();

pub fn get_message_type_info(message_type: &str) -> Option<&'static MessageTypeInfo> {
    REGISTRY.get_or_init(build_registry).get(message_type)
}

fn build_registry() -> HashMap<&'static str, MessageTypeInfo> {
    let mut m = HashMap::new();

    // ── ADT — Admit / Discharge / Transfer ───────────────────────────────────
    m.insert(
        "ADT^A01",
        MessageTypeInfo {
            description: "Admit / Visit Notification",
            typical_segments: &[
                "MSH", "EVN", "PID", "PD1", "NK1", "PV1", "PV2", "OBX", "AL1", "DG1",
            ],
        },
    );
    m.insert(
        "ADT^A02",
        MessageTypeInfo {
            description: "Transfer a Patient",
            typical_segments: &["MSH", "EVN", "PID", "PD1", "PV1", "PV2"],
        },
    );
    m.insert(
        "ADT^A03",
        MessageTypeInfo {
            description: "Discharge / End Visit",
            typical_segments: &["MSH", "EVN", "PID", "PD1", "PV1", "PV2", "DG1", "DRG"],
        },
    );
    m.insert(
        "ADT^A04",
        MessageTypeInfo {
            description: "Register a Patient",
            typical_segments: &[
                "MSH", "EVN", "PID", "PD1", "NK1", "PV1", "PV2", "AL1", "DG1", "GT1", "IN1",
            ],
        },
    );
    m.insert(
        "ADT^A05",
        MessageTypeInfo {
            description: "Pre-Admit a Patient",
            typical_segments: &[
                "MSH", "EVN", "PID", "PD1", "NK1", "PV1", "PV2", "DG1", "GT1", "IN1",
            ],
        },
    );
    m.insert(
        "ADT^A06",
        MessageTypeInfo {
            description: "Change Outpatient to Inpatient",
            typical_segments: &["MSH", "EVN", "PID", "PD1", "PV1", "PV2"],
        },
    );
    m.insert(
        "ADT^A07",
        MessageTypeInfo {
            description: "Change Inpatient to Outpatient",
            typical_segments: &["MSH", "EVN", "PID", "PD1", "PV1", "PV2"],
        },
    );
    m.insert(
        "ADT^A08",
        MessageTypeInfo {
            description: "Update Patient Information",
            typical_segments: &[
                "MSH", "EVN", "PID", "PD1", "NK1", "PV1", "PV2", "OBX", "AL1", "DG1", "GT1", "IN1",
            ],
        },
    );
    m.insert(
        "ADT^A09",
        MessageTypeInfo {
            description: "Patient Departing — Tracking",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "PV2"],
        },
    );
    m.insert(
        "ADT^A10",
        MessageTypeInfo {
            description: "Patient Arriving — Tracking",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "PV2"],
        },
    );
    m.insert(
        "ADT^A11",
        MessageTypeInfo {
            description: "Cancel Admit / Visit Notification",
            typical_segments: &["MSH", "EVN", "PID", "PD1", "PV1"],
        },
    );
    m.insert(
        "ADT^A12",
        MessageTypeInfo {
            description: "Cancel Transfer",
            typical_segments: &["MSH", "EVN", "PID", "PD1", "PV1"],
        },
    );
    m.insert(
        "ADT^A13",
        MessageTypeInfo {
            description: "Cancel Discharge / End Visit",
            typical_segments: &["MSH", "EVN", "PID", "PD1", "PV1"],
        },
    );
    m.insert(
        "ADT^A17",
        MessageTypeInfo {
            description: "Swap Patients",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "PID", "PV1"],
        },
    );
    m.insert(
        "ADT^A18",
        MessageTypeInfo {
            description: "Merge Patient Information",
            typical_segments: &["MSH", "EVN", "PID", "MRG", "PV1"],
        },
    );
    m.insert(
        "ADT^A19",
        MessageTypeInfo {
            description: "Patient Query",
            typical_segments: &["MSH", "QRD", "QRF"],
        },
    );
    m.insert(
        "ADT^A21",
        MessageTypeInfo {
            description: "Patient Goes on Leave of Absence",
            typical_segments: &["MSH", "EVN", "PID", "PV1"],
        },
    );
    m.insert(
        "ADT^A22",
        MessageTypeInfo {
            description: "Patient Returns from Leave of Absence",
            typical_segments: &["MSH", "EVN", "PID", "PV1"],
        },
    );
    m.insert(
        "ADT^A23",
        MessageTypeInfo {
            description: "Delete a Patient Record",
            typical_segments: &["MSH", "EVN", "PID", "PV1"],
        },
    );
    m.insert(
        "ADT^A24",
        MessageTypeInfo {
            description: "Link Patient Information",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "PID"],
        },
    );
    m.insert(
        "ADT^A25",
        MessageTypeInfo {
            description: "Cancel Pending Discharge",
            typical_segments: &["MSH", "EVN", "PID", "PV1"],
        },
    );
    m.insert(
        "ADT^A26",
        MessageTypeInfo {
            description: "Cancel Pending Transfer",
            typical_segments: &["MSH", "EVN", "PID", "PV1"],
        },
    );
    m.insert(
        "ADT^A27",
        MessageTypeInfo {
            description: "Cancel Pending Admit",
            typical_segments: &["MSH", "EVN", "PID", "PV1"],
        },
    );
    m.insert(
        "ADT^A28",
        MessageTypeInfo {
            description: "Add Person Information",
            typical_segments: &["MSH", "EVN", "PID", "PD1", "NK1", "GT1", "IN1"],
        },
    );
    m.insert(
        "ADT^A29",
        MessageTypeInfo {
            description: "Delete Person Information",
            typical_segments: &["MSH", "EVN", "PID"],
        },
    );
    m.insert(
        "ADT^A30",
        MessageTypeInfo {
            description: "Merge Person Information",
            typical_segments: &["MSH", "EVN", "PID", "MRG"],
        },
    );
    m.insert(
        "ADT^A31",
        MessageTypeInfo {
            description: "Update Person Information",
            typical_segments: &["MSH", "EVN", "PID", "PD1", "NK1", "GT1", "IN1"],
        },
    );
    m.insert(
        "ADT^A34",
        MessageTypeInfo {
            description: "Merge Patient — Patient ID Only",
            typical_segments: &["MSH", "EVN", "PID", "MRG"],
        },
    );
    m.insert(
        "ADT^A35",
        MessageTypeInfo {
            description: "Merge Patient — Account Number Only",
            typical_segments: &["MSH", "EVN", "PID", "MRG"],
        },
    );
    m.insert(
        "ADT^A36",
        MessageTypeInfo {
            description: "Merge Patient — Patient ID & Account Number",
            typical_segments: &["MSH", "EVN", "PID", "MRG"],
        },
    );
    m.insert(
        "ADT^A37",
        MessageTypeInfo {
            description: "Unlink Patient Information",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "PID"],
        },
    );
    m.insert(
        "ADT^A38",
        MessageTypeInfo {
            description: "Cancel Pre-Admit",
            typical_segments: &["MSH", "EVN", "PID", "PV1"],
        },
    );
    m.insert(
        "ADT^A39",
        MessageTypeInfo {
            description: "Merge Patient — Patient Identifier List",
            typical_segments: &["MSH", "EVN", "PID", "MRG", "PV1"],
        },
    );
    m.insert(
        "ADT^A40",
        MessageTypeInfo {
            description: "Merge Patient — Patient Identifier List",
            typical_segments: &["MSH", "EVN", "PID", "MRG"],
        },
    );
    m.insert(
        "ADT^A41",
        MessageTypeInfo {
            description: "Merge Patient — Patient Account Number",
            typical_segments: &["MSH", "EVN", "PID", "MRG"],
        },
    );
    m.insert(
        "ADT^A45",
        MessageTypeInfo {
            description: "Move Visit Information — Visit Number",
            typical_segments: &["MSH", "EVN", "PID", "MRG", "PV1"],
        },
    );
    m.insert(
        "ADT^A47",
        MessageTypeInfo {
            description: "Change Patient Identifier List",
            typical_segments: &["MSH", "EVN", "PID", "MRG"],
        },
    );
    m.insert(
        "ADT^A50",
        MessageTypeInfo {
            description: "Change Alternate Patient ID",
            typical_segments: &["MSH", "EVN", "PID", "MRG", "PV1"],
        },
    );
    m.insert(
        "ADT^A51",
        MessageTypeInfo {
            description: "Change Visit Number",
            typical_segments: &["MSH", "EVN", "PID", "MRG", "PV1"],
        },
    );
    m.insert(
        "ADT^A60",
        MessageTypeInfo {
            description: "Update Allergy Information",
            typical_segments: &["MSH", "EVN", "PID", "IAM"],
        },
    );

    // ── ORU — Observation Results (Unsolicited) ───────────────────────────────
    m.insert(
        "ORU^R01",
        MessageTypeInfo {
            description: "Unsolicited Observation Result",
            typical_segments: &["MSH", "PID", "PV1", "ORC", "OBR", "OBX", "NTE"],
        },
    );
    m.insert(
        "ORU^R30",
        MessageTypeInfo {
            description: "Unsolicited Point-of-Care Observation (no existing order)",
            typical_segments: &["MSH", "PID", "PV1", "OBR", "OBX"],
        },
    );
    m.insert(
        "ORU^W01",
        MessageTypeInfo {
            description: "Waveform Result (Unsolicited)",
            typical_segments: &["MSH", "PID", "PV1", "OBR", "OBX"],
        },
    );

    // ── ORM / OML — Orders ───────────────────────────────────────────────────
    m.insert(
        "ORM^O01",
        MessageTypeInfo {
            description: "General Order Message",
            typical_segments: &[
                "MSH", "PID", "PV1", "IN1", "GT1", "ORC", "OBR", "RQD", "RX1", "OBX", "BLG",
            ],
        },
    );
    m.insert(
        "OML^O21",
        MessageTypeInfo {
            description: "Laboratory Order",
            typical_segments: &["MSH", "PID", "PV1", "ORC", "OBR", "OBX", "SPM"],
        },
    );
    m.insert(
        "OML^O33",
        MessageTypeInfo {
            description: "Laboratory Order for Multiple Orders per Specimen",
            typical_segments: &["MSH", "PID", "PV1", "SPM", "ORC", "OBR", "OBX"],
        },
    );
    m.insert(
        "OML^O35",
        MessageTypeInfo {
            description: "Laboratory Order for Multiple Orders per Container",
            typical_segments: &["MSH", "PID", "PV1", "SAC", "ORC", "OBR", "OBX"],
        },
    );

    // ── ORR / ORL — Order Responses ──────────────────────────────────────────
    m.insert(
        "ORR^O02",
        MessageTypeInfo {
            description: "Order Acknowledgement",
            typical_segments: &["MSH", "MSA", "ERR", "PID", "ORC", "OBR"],
        },
    );
    m.insert(
        "ORL^O22",
        MessageTypeInfo {
            description: "General Laboratory Order Response",
            typical_segments: &["MSH", "MSA", "ERR", "PID", "ORC", "OBR", "SPM"],
        },
    );

    // ── SIU — Scheduling Information (Unsolicited) ────────────────────────────
    m.insert(
        "SIU^S12",
        MessageTypeInfo {
            description: "New Appointment Booking",
            typical_segments: &[
                "MSH", "SCH", "TQ1", "NTE", "PID", "PV1", "RGS", "AIS", "AIG", "AIL", "AIP",
            ],
        },
    );
    m.insert(
        "SIU^S13",
        MessageTypeInfo {
            description: "Appointment Rescheduling",
            typical_segments: &[
                "MSH", "SCH", "TQ1", "NTE", "PID", "PV1", "RGS", "AIS", "AIG", "AIL", "AIP",
            ],
        },
    );
    m.insert(
        "SIU^S14",
        MessageTypeInfo {
            description: "Appointment Modification",
            typical_segments: &[
                "MSH", "SCH", "TQ1", "NTE", "PID", "PV1", "RGS", "AIS", "AIG", "AIL", "AIP",
            ],
        },
    );
    m.insert(
        "SIU^S15",
        MessageTypeInfo {
            description: "Appointment Cancellation",
            typical_segments: &["MSH", "SCH", "TQ1", "NTE", "PID", "PV1", "RGS", "AIS"],
        },
    );
    m.insert(
        "SIU^S17",
        MessageTypeInfo {
            description: "Appointment Deletion",
            typical_segments: &["MSH", "SCH", "TQ1", "PID", "PV1", "RGS", "AIS"],
        },
    );
    m.insert(
        "SIU^S24",
        MessageTypeInfo {
            description: "Patient Walked Out",
            typical_segments: &["MSH", "SCH", "PID", "PV1"],
        },
    );
    m.insert(
        "SIU^S26",
        MessageTypeInfo {
            description: "Patient Did Not Show (No-Show)",
            typical_segments: &["MSH", "SCH", "PID", "PV1"],
        },
    );

    // ── MDM — Medical Document Management ────────────────────────────────────
    m.insert(
        "MDM^T01",
        MessageTypeInfo {
            description: "Original Document Notification",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA"],
        },
    );
    m.insert(
        "MDM^T02",
        MessageTypeInfo {
            description: "Original Document Notification with Content",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA", "OBX"],
        },
    );
    m.insert(
        "MDM^T03",
        MessageTypeInfo {
            description: "Document Status Change Notification",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA"],
        },
    );
    m.insert(
        "MDM^T04",
        MessageTypeInfo {
            description: "Document Status Change Notification with Content",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA", "OBX"],
        },
    );
    m.insert(
        "MDM^T05",
        MessageTypeInfo {
            description: "Document Addendum Notification",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA"],
        },
    );
    m.insert(
        "MDM^T06",
        MessageTypeInfo {
            description: "Document Addendum Notification with Content",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA", "OBX"],
        },
    );
    m.insert(
        "MDM^T07",
        MessageTypeInfo {
            description: "Document Edit Notification",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA"],
        },
    );
    m.insert(
        "MDM^T08",
        MessageTypeInfo {
            description: "Document Edit Notification with Content",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA", "OBX"],
        },
    );
    m.insert(
        "MDM^T09",
        MessageTypeInfo {
            description: "Document Replacement Notification",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA"],
        },
    );
    m.insert(
        "MDM^T10",
        MessageTypeInfo {
            description: "Document Replacement Notification with Content",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA", "OBX"],
        },
    );
    m.insert(
        "MDM^T11",
        MessageTypeInfo {
            description: "Document Cancel Notification",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "TXA"],
        },
    );

    // ── BAR — Add/Change Billing Account ─────────────────────────────────────
    m.insert(
        "BAR^P01",
        MessageTypeInfo {
            description: "Add Patient Account",
            typical_segments: &[
                "MSH", "EVN", "PID", "PD1", "PV1", "PV2", "DG1", "DRG", "GT1", "NK1", "IN1", "IN2",
                "ACC",
            ],
        },
    );
    m.insert(
        "BAR^P02",
        MessageTypeInfo {
            description: "Purge Patient Billing Account",
            typical_segments: &["MSH", "EVN", "PID", "PV1"],
        },
    );
    m.insert(
        "BAR^P05",
        MessageTypeInfo {
            description: "Update Account",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "DG1", "GT1", "IN1"],
        },
    );
    m.insert(
        "BAR^P06",
        MessageTypeInfo {
            description: "End Account",
            typical_segments: &["MSH", "EVN", "PID", "PV1"],
        },
    );
    m.insert(
        "BAR^P10",
        MessageTypeInfo {
            description: "Transmit Ambulatory Payment Classification",
            typical_segments: &["MSH", "PID", "PV1", "DG1", "GP1", "GP2", "PR1"],
        },
    );
    m.insert(
        "BAR^P12",
        MessageTypeInfo {
            description: "Update Diagnosis / Procedure",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "DG1", "DRG", "PR1"],
        },
    );

    // ── DFT — Detail Financial Transactions ──────────────────────────────────
    m.insert(
        "DFT^P03",
        MessageTypeInfo {
            description: "Post Detail Financial Transaction",
            typical_segments: &[
                "MSH", "EVN", "PID", "PV1", "PV2", "FT1", "DG1", "GT1", "IN1",
            ],
        },
    );
    m.insert(
        "DFT^P11",
        MessageTypeInfo {
            description: "Post Detail Financial Transactions",
            typical_segments: &["MSH", "EVN", "PID", "PV1", "PV2", "FT1", "DG1"],
        },
    );

    // ── MFN — Master File Notifications ──────────────────────────────────────
    m.insert(
        "MFN^M01",
        MessageTypeInfo {
            description: "Master File — Not Otherwise Specified",
            typical_segments: &["MSH", "MFI", "MFE", "ZBX"],
        },
    );
    m.insert(
        "MFN^M02",
        MessageTypeInfo {
            description: "Master File — Staff Practitioner",
            typical_segments: &[
                "MSH", "MFI", "MFE", "STF", "PRA", "ORG", "AFF", "LAN", "EDU",
            ],
        },
    );
    m.insert(
        "MFN^M04",
        MessageTypeInfo {
            description: "Master File — Charge Description",
            typical_segments: &["MSH", "MFI", "MFE", "CDM", "PRC"],
        },
    );
    m.insert(
        "MFN^M05",
        MessageTypeInfo {
            description: "Patient Location Master File",
            typical_segments: &["MSH", "MFI", "MFE", "LOC", "LCH", "LRL", "LDP", "LCC"],
        },
    );
    m.insert(
        "MFN^M06",
        MessageTypeInfo {
            description: "Clinical Study Master File",
            typical_segments: &["MSH", "MFI", "MFE", "CM0", "CM1", "CM2"],
        },
    );
    m.insert(
        "MFN^M08",
        MessageTypeInfo {
            description: "Observation (Numeric) Master File",
            typical_segments: &["MSH", "MFI", "MFE", "OM1", "OM2"],
        },
    );
    m.insert(
        "MFN^M09",
        MessageTypeInfo {
            description: "Observation (Categorical) Master File",
            typical_segments: &["MSH", "MFI", "MFE", "OM1", "OM3"],
        },
    );
    m.insert(
        "MFN^M10",
        MessageTypeInfo {
            description: "Observation Batteries Master File",
            typical_segments: &["MSH", "MFI", "MFE", "OM1", "OM5"],
        },
    );
    m.insert(
        "MFN^M11",
        MessageTypeInfo {
            description: "Calculated Observations Master File",
            typical_segments: &["MSH", "MFI", "MFE", "OM1", "OM6"],
        },
    );
    m.insert(
        "MFN^M12",
        MessageTypeInfo {
            description: "Master File — Observation / Service Attributes",
            typical_segments: &["MSH", "MFI", "MFE", "OM1", "OM7"],
        },
    );
    m.insert(
        "MFN^M15",
        MessageTypeInfo {
            description: "Inventory Item Master File",
            typical_segments: &["MSH", "MFI", "MFE", "IIM"],
        },
    );

    // ── QBP / QRY — Queries ──────────────────────────────────────────────────
    m.insert(
        "QBP^Q11",
        MessageTypeInfo {
            description: "Get Identifiers Request",
            typical_segments: &["MSH", "QPD", "RCP"],
        },
    );
    m.insert(
        "QBP^Q22",
        MessageTypeInfo {
            description: "Find Candidates",
            typical_segments: &["MSH", "QPD", "RCP"],
        },
    );
    m.insert(
        "QBP^Q23",
        MessageTypeInfo {
            description: "Get Corresponding Identifiers",
            typical_segments: &["MSH", "QPD", "RCP"],
        },
    );
    m.insert(
        "QBP^Q25",
        MessageTypeInfo {
            description: "Allocate Identifiers",
            typical_segments: &["MSH", "QPD", "RCP"],
        },
    );
    m.insert(
        "QRY^A19",
        MessageTypeInfo {
            description: "Patient Query",
            typical_segments: &["MSH", "QRD", "QRF"],
        },
    );
    m.insert(
        "QRY^R02",
        MessageTypeInfo {
            description: "Query for Results of Observation",
            typical_segments: &["MSH", "QRD", "QRF"],
        },
    );

    // ── RSP — Response to Query ───────────────────────────────────────────────
    m.insert(
        "RSP^K11",
        MessageTypeInfo {
            description: "Get Identifiers Response",
            typical_segments: &["MSH", "MSA", "ERR", "QAK", "QPD", "PID"],
        },
    );
    m.insert(
        "RSP^K22",
        MessageTypeInfo {
            description: "Find Candidates Response",
            typical_segments: &["MSH", "MSA", "ERR", "QAK", "QPD", "PID", "PD1", "NK1"],
        },
    );
    m.insert(
        "RSP^K23",
        MessageTypeInfo {
            description: "Get Corresponding Identifiers Response",
            typical_segments: &["MSH", "MSA", "ERR", "QAK", "QPD", "PID"],
        },
    );

    // ── VXU / VXQ — Vaccination ───────────────────────────────────────────────
    m.insert(
        "VXU^V04",
        MessageTypeInfo {
            description: "Unsolicited Vaccination Record Update",
            typical_segments: &[
                "MSH", "PID", "PD1", "NK1", "PV1", "IN1", "ORC", "RXA", "RXR", "OBX", "NTE",
            ],
        },
    );
    m.insert(
        "VXQ^V01",
        MessageTypeInfo {
            description: "Query for Vaccination Record",
            typical_segments: &["MSH", "QRD", "QRF"],
        },
    );
    m.insert(
        "VXR^V03",
        MessageTypeInfo {
            description: "Vaccination Record Response",
            typical_segments: &[
                "MSH", "MSA", "QRD", "PID", "NK1", "ORC", "RXA", "RXR", "OBX",
            ],
        },
    );
    m.insert(
        "VXX^V02",
        MessageTypeInfo {
            description: "Response for Vaccination Query — Multiple PID Match",
            typical_segments: &["MSH", "MSA", "QRD", "PID", "NK1"],
        },
    );

    // ── PPR — Patient Problems ────────────────────────────────────────────────
    m.insert(
        "PPR^PC1",
        MessageTypeInfo {
            description: "Problem Add",
            typical_segments: &["MSH", "PID", "PV1", "PRB", "NTE", "VAR", "OBX"],
        },
    );
    m.insert(
        "PPR^PC2",
        MessageTypeInfo {
            description: "Problem Update",
            typical_segments: &["MSH", "PID", "PV1", "PRB", "NTE", "VAR", "OBX"],
        },
    );
    m.insert(
        "PPR^PC3",
        MessageTypeInfo {
            description: "Problem Delete",
            typical_segments: &["MSH", "PID", "PV1", "PRB"],
        },
    );

    // ── RAS / RDE — Pharmacy / Treatment ─────────────────────────────────────
    m.insert(
        "RAS^O17",
        MessageTypeInfo {
            description: "Pharmacy / Treatment Administration",
            typical_segments: &["MSH", "PID", "PV1", "ORC", "RXA", "RXR", "OBX"],
        },
    );
    m.insert(
        "RDE^O11",
        MessageTypeInfo {
            description: "Pharmacy / Treatment Encoded Order",
            typical_segments: &[
                "MSH", "PID", "PV1", "IN1", "ORC", "RXE", "TQ1", "RXR", "RXC", "OBX",
            ],
        },
    );
    m.insert(
        "RDS^O13",
        MessageTypeInfo {
            description: "Pharmacy / Treatment Dispense",
            typical_segments: &["MSH", "PID", "PV1", "ORC", "RXD", "RXR", "RXC", "OBX"],
        },
    );

    // ── ACK — General Acknowledgement ────────────────────────────────────────
    m.insert(
        "ACK",
        MessageTypeInfo {
            description: "General Acknowledgement",
            typical_segments: &["MSH", "MSA", "ERR"],
        },
    );

    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_type() {
        let info = get_message_type_info("ADT^A01").unwrap();
        assert_eq!(info.description, "Admit / Visit Notification");
        assert!(info.typical_segments.contains(&"PID"));
    }

    #[test]
    fn test_unknown_type() {
        assert!(get_message_type_info("ZZZ^Z99").is_none());
    }

    #[test]
    fn test_oru_r01() {
        let info = get_message_type_info("ORU^R01").unwrap();
        assert_eq!(info.description, "Unsolicited Observation Result");
        assert!(info.typical_segments.contains(&"OBX"));
    }
}
