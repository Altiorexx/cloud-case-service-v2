use mongodb::{bson::{self, doc}, error::Error, options::{ClientOptions, FindOptions, UpdateOptions}, Client, Collection};
use rocket::futures::TryStreamExt;
use std::env;
use uuid::Uuid;


use crate::types::case_database::{CIS18Case, Case};
use crate::types::collaboration_handler::{Message, Change, TextOrIntValue};


pub struct CaseDatabase {
    cases: Collection<Case>,
    cis18_template: Collection<CIS18Case>
}

pub async fn new_case_database() -> CaseDatabase {

    let client_uri = env::var("MONGODB_CONNECTION_STRING").unwrap();
    let database_name = "core";

    let client_options = ClientOptions::parse(client_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let database = client.database(database_name);
    let cases = database.collection::<Case>("cases");
    let cis18_template = database.collection::<CIS18Case>("templates");

    CaseDatabase {
        cases,
        cis18_template
    }
}

impl CaseDatabase {

    pub async fn create_cis18_case(&self, group_id: &String, name: &String, implementation_group: i32) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let filter = doc! { "framework": "cis18" };
        let result = self.cis18_template.find_one(filter, None).await?;
        let mut template = match result {
            Some(v) => v,
            None => return Ok(None)
        };

        let case_id = Uuid::new_v4().to_string();
        template.case_id = case_id.clone();
        template.group_id = group_id.to_string();
        template.name = name.to_string();
        template.framework = "cis18".to_owned();
        template.implementation_group = implementation_group;

        self.cases.insert_one(Case::CIS18(template), None).await?;
        Ok(Some(case_id))
    }

    pub async fn read_case_by_id(&self, case_id: String) -> Result<Option<Case>, Error> {
        let filter = doc! { "case_id": case_id }; 
        let result = self.cases.find_one(filter, None).await?;
        Ok(result)
    }

    pub async fn rename_case(&self, case_id: &String, name: &String) -> Result<Option<()>, Error> {
        let filter = doc! { "case_id": case_id };
        let change = doc! { "$set": { "name": name} };
        let options = UpdateOptions::builder().upsert(false).build();
        self.cases.update_one(filter, change, options).await?;
        Ok(Some(()))
    }

    pub async fn delete_case(&self, case_id: &String) -> Result<Option<()>, Error> {
        let query = doc! { "case_id": case_id };
        self.cases.delete_one(query, None).await?;
        Ok(Some(()))
    }

    pub async fn read_cases_by_group_id(&self, group_id: String) -> Result<Vec<Case>, Error> {
        let filter = doc! { "group_id": group_id };
        let options = FindOptions::builder()
            .projection(doc! { "case_id": 1, "group_id": 1, "name": 1, "framework": 1, "implementation_group": 1 })
            .build();
        let mut cursor = self.cases.find(filter, options).await?;
        let mut cases = Vec::new();
        while let Some(case) = cursor.try_next().await? {
            println!("{:?}", case);
            cases.push(case)
        }
        Ok(cases)
    }

    pub async fn read_case_framework(&self, case_id: &String) -> Result<String, Box<dyn std::error::Error>> {
        let filter = doc! { "case_id": case_id.to_string() };
        let result = self.cases.find_one(filter, None).await?;
        match result {
            Some(Case::CIS18(case)) => Ok(case.framework),
            Some(Case::NIS2(case)) => Ok(case.framework),
            None => Err("no case found".into())
        }
    }

    pub async fn update_cis18_content(&self, case_id: &String, message: &Message) -> Result<(), Box<dyn  std::error::Error>> {
        if let Change::CIS18Change(ref change) = message.data {
            let filter = doc! {
                "case_id": &case_id,
                "controls.id": &change.control_id,
                "controls.subcontrols.id": &change.subcontrol_id,
            };

            let update = doc! {
                "$set": {
                    format!("controls.$.subcontrols.$[elem].{}", &change.field): match &change.value {
                        TextOrIntValue::String(val) => bson::to_bson(val)?,
                        TextOrIntValue::Number(val) => bson::to_bson(val)?,
                    }
                }
            };

            let array_filters = vec![
                doc! { "elem.id": &change.subcontrol_id }
            ];

            let update_options = UpdateOptions::builder()
                .array_filters(array_filters)
                .build();

            self.cases.update_one(filter, update, update_options).await?;

            Ok(())
        } else {
            Err("Invalid change type for CIS18".into())
        }
    }


}

