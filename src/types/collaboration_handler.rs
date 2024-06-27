use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Message {
    pub user_id: String,
    pub event: Event,
    pub data: Change
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    TextChange,
    DropdownChange,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Change {
    CIS18Change(CIS18Change),
    NIS2Change
}

#[derive(Debug, Deserialize)]
pub struct CIS18Change {
    pub control_id: String,
    pub subcontrol_id: String,
    pub field: String,
    pub value: TextOrIntValue
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TextOrIntValue {
    String(String),
    Number(i32)
}