/// HL7 v2.x message validation.
///
/// Checks required segments and fields per message type and returns a list
/// of human-readable warnings.  Rules are intentionally pragmatic: they cover
/// the most critical constraints from the HL7 v2.5.1 specification for the
/// message types most commonly seen in hospital integration engines.
///
/// The validator is non-blocking — every message is stored regardless of
/// warnings.  Warnings are surfaced in the UI so developers can spot missing
/// fields at a glance without consulting the spec.
use crate::hl7::types::{Hl7Message, Hl7Segment};
use serde::{Deserialize, Serialize};

// ─── Public types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Short machine-readable code (e.g. `"MISSING_SEGMENT"`)
    pub code: String,
    /// Human-readable message shown in the UI
    pub message: String,
    /// Segment the warning relates to (e.g. `"PID"`)
    pub segment: String,
    /// Field index within the segment, if applicable (1-based, HL7 standard)
    pub field: Option<usize>,
}

// ─── Entry point ──────────────────────────────────────────────────────────────

/// Validate a parsed HL7 message and return all warnings found.
/// Returns an empty `Vec` if the message is fully valid (or unknown type).
pub fn validate_message(msg: &Hl7Message) -> Vec<ValidationWarning> {
    let mut warnings = Vec::new();

    // Always validate MSH required fields first
    validate_msh(msg, &mut warnings);

    // Message-type–specific rules
    match msg.message_type.as_str() {
        t if t.starts_with("ADT^") => validate_adt(msg, &mut warnings),
        "ORU^R01" => validate_oru_r01(msg, &mut warnings),
        "ORM^O01" => validate_orm_o01(msg, &mut warnings),
        "OML^O21" => validate_oml_o21(msg, &mut warnings),
        t if t.starts_with("SIU^") => validate_siu(msg, &mut warnings),
        t if t.starts_with("MDM^") => validate_mdm(msg, &mut warnings),
        _ => {} // No rules for unknown types — do not emit spurious warnings
    }

    warnings
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn find_segment<'a>(msg: &'a Hl7Message, name: &str) -> Option<&'a Hl7Segment> {
    msg.segments.iter().find(|s| s.name == name)
}

fn has_segment(msg: &Hl7Message, name: &str) -> bool {
    msg.segments.iter().any(|s| s.name == name)
}

/// Return the field value for a segment (1-based HL7 field index).
fn field_value(seg: &Hl7Segment, index: usize) -> Option<&str> {
    seg.fields
        .iter()
        .find(|f| f.index == index)
        .map(|f| f.value.as_str())
        .filter(|v| !v.is_empty())
}

fn warn_missing_segment(warnings: &mut Vec<ValidationWarning>, seg: &str, context: &str) {
    warnings.push(ValidationWarning {
        code: "MISSING_SEGMENT".into(),
        message: format!("{seg} segment is required for {context}"),
        segment: seg.into(),
        field: None,
    });
}

fn warn_missing_field(
    warnings: &mut Vec<ValidationWarning>,
    seg: &str,
    field: usize,
    field_name: &str,
    context: &str,
) {
    warnings.push(ValidationWarning {
        code: "MISSING_FIELD".into(),
        message: format!("{seg}-{field} ({field_name}) is required for {context}"),
        segment: seg.into(),
        field: Some(field),
    });
}

// ─── MSH — universal required fields ─────────────────────────────────────────

fn validate_msh(msg: &Hl7Message, warnings: &mut Vec<ValidationWarning>) {
    let Some(msh) = find_segment(msg, "MSH") else {
        // Without MSH the parser would have failed; guard anyway
        warn_missing_segment(warnings, "MSH", "all HL7 messages");
        return;
    };

    // MSH-3: Sending Application
    if field_value(msh, 3).is_none() {
        warn_missing_field(
            warnings,
            "MSH",
            3,
            "Sending Application",
            "all HL7 messages",
        );
    }
    // MSH-4: Sending Facility
    if field_value(msh, 4).is_none() {
        warn_missing_field(warnings, "MSH", 4, "Sending Facility", "all HL7 messages");
    }
    // MSH-9: Message Type
    if field_value(msh, 9).is_none() {
        warn_missing_field(warnings, "MSH", 9, "Message Type", "all HL7 messages");
    }
    // MSH-10: Message Control ID
    if field_value(msh, 10).is_none() {
        warn_missing_field(
            warnings,
            "MSH",
            10,
            "Message Control ID",
            "all HL7 messages",
        );
    }
    // MSH-11: Processing ID (P/T/D)
    if field_value(msh, 11).is_none() {
        warn_missing_field(warnings, "MSH", 11, "Processing ID", "all HL7 messages");
    }
    // MSH-12: Version ID
    if field_value(msh, 12).is_none() {
        warn_missing_field(warnings, "MSH", 12, "Version ID", "all HL7 messages");
    }
}

// ─── ADT ──────────────────────────────────────────────────────────────────────

fn validate_adt(msg: &Hl7Message, warnings: &mut Vec<ValidationWarning>) {
    let ctx = &msg.message_type;

    // EVN is required for all ADT messages except A19 (query)
    if msg.message_type != "ADT^A19" && !has_segment(msg, "EVN") {
        warn_missing_segment(warnings, "EVN", ctx);
    }

    // PID is required for all ADT messages
    let Some(pid) = find_segment(msg, "PID") else {
        warn_missing_segment(warnings, "PID", ctx);
        return; // Can't validate PID fields without the segment
    };

    // PID-3: Patient Identifier List
    if field_value(pid, 3).is_none() {
        warn_missing_field(warnings, "PID", 3, "Patient Identifier List", ctx);
    }
    // PID-5: Patient Name
    if field_value(pid, 5).is_none() {
        warn_missing_field(warnings, "PID", 5, "Patient Name", ctx);
    }
    // PID-7: Date/Time of Birth (recommended)
    // PID-8: Administrative Sex
    if field_value(pid, 8).is_none() {
        warn_missing_field(warnings, "PID", 8, "Administrative Sex", ctx);
    }

    // PV1 is required for most ADT events (transfer/admit/discharge)
    let pv1_required = matches!(
        msg.message_type.as_str(),
        "ADT^A01"
            | "ADT^A02"
            | "ADT^A03"
            | "ADT^A04"
            | "ADT^A05"
            | "ADT^A06"
            | "ADT^A07"
            | "ADT^A08"
            | "ADT^A09"
            | "ADT^A10"
            | "ADT^A11"
            | "ADT^A12"
            | "ADT^A13"
    );
    if pv1_required {
        if let Some(pv1) = find_segment(msg, "PV1") {
            // PV1-2: Patient Class (I/O/E/P/R/B/C/N/U)
            if field_value(pv1, 2).is_none() {
                warn_missing_field(warnings, "PV1", 2, "Patient Class", ctx);
            }
        } else {
            warn_missing_segment(warnings, "PV1", ctx);
        }
    }
}

// ─── ORU^R01 ─────────────────────────────────────────────────────────────────

fn validate_oru_r01(msg: &Hl7Message, warnings: &mut Vec<ValidationWarning>) {
    let ctx = "ORU^R01";

    // OBR is required
    let Some(obr) = find_segment(msg, "OBR") else {
        warn_missing_segment(warnings, "OBR", ctx);
        return;
    };
    // OBR-1: Set ID
    // OBR-3: Filler Order Number (required in R01)
    if field_value(obr, 3).is_none() {
        warn_missing_field(warnings, "OBR", 3, "Filler Order Number", ctx);
    }
    // OBR-4: Universal Service Identifier
    if field_value(obr, 4).is_none() {
        warn_missing_field(warnings, "OBR", 4, "Universal Service Identifier", ctx);
    }

    // OBX is required (at least one observation)
    if !has_segment(msg, "OBX") {
        warn_missing_segment(warnings, "OBX", ctx);
    }
    // Validate each OBX
    for seg in msg.segments.iter().filter(|s| s.name == "OBX") {
        // OBX-2: Value Type (NM, ST, CWE, etc.)
        if field_value(seg, 2).is_none() {
            warnings.push(ValidationWarning {
                code: "MISSING_FIELD".into(),
                message: "OBX-2 (Value Type) is required for ORU^R01".into(),
                segment: "OBX".into(),
                field: Some(2),
            });
        }
        // OBX-3: Observation Identifier
        if field_value(seg, 3).is_none() {
            warnings.push(ValidationWarning {
                code: "MISSING_FIELD".into(),
                message: "OBX-3 (Observation Identifier) is required for ORU^R01".into(),
                segment: "OBX".into(),
                field: Some(3),
            });
        }
        // OBX-11: Observation Result Status
        if field_value(seg, 11).is_none() {
            warnings.push(ValidationWarning {
                code: "MISSING_FIELD".into(),
                message: "OBX-11 (Observation Result Status) is required for ORU^R01".into(),
                segment: "OBX".into(),
                field: Some(11),
            });
        }
    }
}

// ─── ORM^O01 ─────────────────────────────────────────────────────────────────

fn validate_orm_o01(msg: &Hl7Message, warnings: &mut Vec<ValidationWarning>) {
    let ctx = "ORM^O01";

    if !has_segment(msg, "PID") {
        warn_missing_segment(warnings, "PID", ctx);
    }

    let Some(orc) = find_segment(msg, "ORC") else {
        warn_missing_segment(warnings, "ORC", ctx);
        return;
    };
    // ORC-1: Order Control
    if field_value(orc, 1).is_none() {
        warn_missing_field(warnings, "ORC", 1, "Order Control", ctx);
    }

    if !has_segment(msg, "OBR") {
        warn_missing_segment(warnings, "OBR", ctx);
    }
}

// ─── OML^O21 ─────────────────────────────────────────────────────────────────

fn validate_oml_o21(msg: &Hl7Message, warnings: &mut Vec<ValidationWarning>) {
    let ctx = "OML^O21";

    if !has_segment(msg, "PID") {
        warn_missing_segment(warnings, "PID", ctx);
    }
    if !has_segment(msg, "ORC") {
        warn_missing_segment(warnings, "ORC", ctx);
    }
    if !has_segment(msg, "OBR") {
        warn_missing_segment(warnings, "OBR", ctx);
    }
}

// ─── SIU ─────────────────────────────────────────────────────────────────────

fn validate_siu(msg: &Hl7Message, warnings: &mut Vec<ValidationWarning>) {
    let ctx = &msg.message_type;

    let Some(sch) = find_segment(msg, "SCH") else {
        warn_missing_segment(warnings, "SCH", ctx);
        return;
    };
    // SCH-1 or SCH-2 must identify the appointment
    if field_value(sch, 1).is_none() && field_value(sch, 2).is_none() {
        warnings.push(ValidationWarning {
            code: "MISSING_FIELD".into(),
            message: format!(
                "SCH-1 (Placer Appointment ID) or SCH-2 (Filler Appointment ID) is required for {ctx}"
            ),
            segment: "SCH".into(),
            field: Some(1),
        });
    }
    // SCH-7: Appointment Reason
    // SCH-25: Filler Status Code
}

// ─── MDM ─────────────────────────────────────────────────────────────────────

fn validate_mdm(msg: &Hl7Message, warnings: &mut Vec<ValidationWarning>) {
    let ctx = &msg.message_type;

    if !has_segment(msg, "EVN") {
        warn_missing_segment(warnings, "EVN", ctx);
    }
    if !has_segment(msg, "PID") {
        warn_missing_segment(warnings, "PID", ctx);
    }
    if !has_segment(msg, "PV1") {
        warn_missing_segment(warnings, "PV1", ctx);
    }

    let Some(txa) = find_segment(msg, "TXA") else {
        warn_missing_segment(warnings, "TXA", ctx);
        return;
    };
    // TXA-2: Document Type
    if field_value(txa, 2).is_none() {
        warn_missing_field(warnings, "TXA", 2, "Document Type", ctx);
    }
    // TXA-9: Originator Code/Name
    // TXA-17: Document Completion Status
    if field_value(txa, 17).is_none() {
        warn_missing_field(warnings, "TXA", 17, "Document Completion Status", ctx);
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hl7::parser::parse_message;

    const VALID_ADT_A01: &str =
        "MSH|^~\\&|SEND_APP|SEND_FAC|REC_APP|REC_FAC|20240101120000||ADT^A01|MSG001|P|2.5\r\
         EVN||20240101120000\r\
         PID|||12345^^^HOSP||Smith^John^^||19800515|M\r\
         PV1||I|WARD1^ROOM1^BED1";

    const MISSING_PID3: &str =
        "MSH|^~\\&|SEND_APP|SEND_FAC|REC_APP|REC_FAC|20240101120000||ADT^A01|MSG001|P|2.5\r\
         EVN||20240101120000\r\
         PID|||||Smith^John^^||19800515|M\r\
         PV1||I|WARD^ROOM^BED";

    const NO_PV1: &str =
        "MSH|^~\\&|SEND_APP|SEND_FAC|REC_APP|REC_FAC|20240101120000||ADT^A01|MSG001|P|2.5\r\
         EVN||20240101120000\r\
         PID|||12345^^^HOSP||Smith^John^^||19800515|M";

    #[test]
    fn test_valid_adt_has_no_warnings() {
        let msg = parse_message(VALID_ADT_A01, "127.0.0.1:9999").unwrap();
        let warnings = validate_message(&msg);
        assert!(
            warnings.is_empty(),
            "Expected no warnings, got: {:?}",
            warnings
        );
    }

    #[test]
    fn test_missing_pid3_triggers_warning() {
        let msg = parse_message(MISSING_PID3, "127.0.0.1:9999").unwrap();
        let warnings = validate_message(&msg);
        assert!(
            warnings
                .iter()
                .any(|w| w.segment == "PID" && w.field == Some(3)),
            "Expected PID-3 warning, got: {:?}",
            warnings
        );
    }

    #[test]
    fn test_missing_pv1_triggers_warning() {
        let msg = parse_message(NO_PV1, "127.0.0.1:9999").unwrap();
        let warnings = validate_message(&msg);
        assert!(
            warnings
                .iter()
                .any(|w| w.segment == "PV1" && w.code == "MISSING_SEGMENT"),
            "Expected PV1 MISSING_SEGMENT warning, got: {:?}",
            warnings
        );
    }

    #[test]
    fn test_unknown_type_no_warnings() {
        let raw = "MSH|^~\\&|APP|FAC|R|R|20240101||ZZZ^Z01|MSG001|P|2.5";
        let msg = parse_message(raw, "127.0.0.1:9999").unwrap();
        // Only MSH field warnings possible; no message-type rules for ZZZ
        let warnings = validate_message(&msg);
        // Should not have segment-level warnings (only possibly MSH field ones)
        assert!(
            warnings.iter().all(|w| w.segment == "MSH"),
            "Unknown type should produce only MSH warnings, got: {:?}",
            warnings
        );
    }
}
