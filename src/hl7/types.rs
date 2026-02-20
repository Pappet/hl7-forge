use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A parsed HL7 v2.x message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hl7Message {
    pub id: String,
    pub raw: String,
    pub received_at: DateTime<Utc>,
    pub source_addr: String,
    pub message_type: String,    // e.g. "ADT^A01"
    pub trigger_event: String,   // e.g. "A01"
    pub message_control_id: String,
    pub sending_application: String,
    pub sending_facility: String,
    pub receiving_application: String,
    pub receiving_facility: String,
    pub version: String,
    pub segments: Vec<Hl7Segment>,
    pub patient_name: Option<String>,
    pub patient_id: Option<String>,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hl7Segment {
    pub name: String,
    pub fields: Vec<Hl7Field>,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hl7Field {
    pub index: usize,
    pub value: String,
    pub components: Vec<String>,
}

/// Separators / encoding characters from MSH-1 and MSH-2
#[derive(Debug, Clone, Copy)]
pub struct Delimiters {
    pub field: char,
    pub component: char,
    pub repetition: char,
    pub escape: char,
    pub subcomponent: char,
}

impl Default for Delimiters {
    fn default() -> Self {
        Self {
            field: '|',
            component: '^',
            repetition: '~',
            escape: '\\',
            subcomponent: '&',
        }
    }
}

impl Hl7Message {
    pub fn new_empty(raw: String, source_addr: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            raw,
            received_at: Utc::now(),
            source_addr,
            message_type: String::new(),
            trigger_event: String::new(),
            message_control_id: String::new(),
            sending_application: String::new(),
            sending_facility: String::new(),
            receiving_application: String::new(),
            receiving_facility: String::new(),
            version: String::new(),
            segments: Vec::new(),
            patient_name: None,
            patient_id: None,
            parse_error: None,
        }
    }
}

/// Summary for the message list (lightweight, no raw/segments)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hl7MessageSummary {
    pub id: String,
    pub received_at: DateTime<Utc>,
    pub source_addr: String,
    pub message_type: String,
    pub trigger_event: String,
    pub message_control_id: String,
    pub sending_facility: String,
    pub patient_name: Option<String>,
    pub patient_id: Option<String>,
    pub segment_count: usize,
    pub parse_error: Option<String>,
}

impl From<&Hl7Message> for Hl7MessageSummary {
    fn from(msg: &Hl7Message) -> Self {
        Self {
            id: msg.id.clone(),
            received_at: msg.received_at,
            source_addr: msg.source_addr.clone(),
            message_type: msg.message_type.clone(),
            trigger_event: msg.trigger_event.clone(),
            message_control_id: msg.message_control_id.clone(),
            sending_facility: msg.sending_facility.clone(),
            patient_name: msg.patient_name.clone(),
            patient_id: msg.patient_id.clone(),
            segment_count: msg.segments.len(),
            parse_error: msg.parse_error.clone(),
        }
    }
}
