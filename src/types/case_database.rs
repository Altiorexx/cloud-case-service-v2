
//use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Case {
    CIS18(CIS18Case),
    NIS2(NIS2Case)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CIS18Case {
    pub case_id: String,
    pub group_id: String,
    pub name: String,
    pub framework: String,
    pub implementation_group: i32,
    pub controls: Vec<CIS18Control>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CIS18Control {
    pub id: String,
    pub title: String,
    pub description: String,
    pub subcontrols: Vec<CIS18SubControl>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CIS18SubControl {
    pub id: String,
    pub title: String,
    pub description: String,
    pub observation: String,
    pub as_is_score: i32,
    pub plan: String,
    pub to_be_score: i32,
    pub soa: String,
    pub implementation_group: Vec<i32>,
    pub documentation: Vec<Documentation>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Documentation {
    name: String,
    src: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NIS2Case {
    pub case_id: String,
    pub name: String,
    pub group_id: String,
    pub framework: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupCases {
    pub group_id: String,
    pub cases: Vec<CaseMetadata>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CaseMetadata {
    pub case_id: String,
    pub group_id: String,
    pub name: String,
    pub framework: String,
    pub implementation_group: Option<i32>
}