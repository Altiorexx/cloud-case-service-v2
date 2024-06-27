use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;

use crate::types::case_database::Case;
use crate::types::case_handler::{CreateCIS18CaseBody, CreateCaseResponse, RenameCaseBody};
use crate::types::ErrorResponse;
use crate::database::case::{CaseDatabase, new_case_database};

pub struct CaseHandler {
    case: CaseDatabase
}

pub async fn new_case_handler() -> CaseHandler {
    CaseHandler {
        case: new_case_database().await
    }
}
    
#[post("/api/case/cis18/create", data = "<data>")]
pub async fn create_cis18_case(handler: &State<CaseHandler>, data: Json<CreateCIS18CaseBody>) -> Result<Custom<Json<CreateCaseResponse>>, Custom<Json<ErrorResponse>>> {
    match handler.case.create_cis18_case(&data.group_id, &data.name, data.implementation_group).await {
        Ok(result) => match result {
            Some(case_id) => Ok(Custom(Status::Ok, Json(CreateCaseResponse{case_id}))),
            None => Err(Custom(Status::NotFound, Json(ErrorResponse{error: format!("no matching template found")})))
        },
        Err(e) => {
            eprintln!("error creating case: {}", e);
            Err(Custom(Status::InternalServerError, Json(ErrorResponse{error: format!("error creating case: {}", e)})))
        }
    }
}

#[post("/api/case/<case_id>/rename", data = "<data>")]
pub async fn rename_case(handler: &State<CaseHandler>, case_id: String, data: Json<RenameCaseBody>) -> Result<Custom<String>, Custom<Json<ErrorResponse>>> {
    match handler.case.rename_case(&case_id, &data.name).await {
        Ok(result) => {
            match result {
                Some(_) => Ok(Custom(Status::Ok, "successfully renamed the case".into())),
                None => Ok(Custom(Status::NotFound, "no case found".into()))
            } 
        },
        Err(e) => {
            eprintln!("error renaming case: {}", e);
            Err(Custom(Status::InternalServerError, Json(ErrorResponse{error: "error renaming case".into()})))
        }
    }
}

#[delete("/api/case/<case_id>/delete")]
pub async fn delete_case(handler: &State<CaseHandler>, case_id: String) -> Result<Custom<String>, Custom<Json<ErrorResponse>>> {
    match handler.case.delete_case(&case_id).await {
        Ok(result) => {
            match result {
                Some(_) => Ok(Custom(Status::Ok, "successfully deleted case".into())),
                None => Ok(Custom(Status::NotFound, "no case found".into()))
            }
        },
        Err(e) => {
            eprintln!("error deleting case: {}", e);
            Err(Custom(Status::InternalServerError, Json(ErrorResponse{error: "error deleting case".into()})))
        }
    }
}

#[get("/api/case/<case_id>")]
pub async fn get_case(handler: &State<CaseHandler>, case_id: &str) -> Result<Custom<Json<Case>>, Custom<Json<ErrorResponse>> > {
    match handler.case.read_case_by_id(case_id.into()).await {
        Ok(r) => {
            match r {
                Some(case) => Ok(Custom(Status::Ok, Json(case))),
                None => Err(Custom(Status::NotFound, Json(ErrorResponse{error: "no case found".into()})))
            }
        },
        Err(e) => {
            eprintln!("error reading case by id: {}", e);
            Err(Custom(Status::InternalServerError, Json(ErrorResponse { error: format!("error reading case by id: {}", e.kind) })))
        }
    }
}

#[get("/api/case/list/<group_id>")]
pub async fn get_cases(handler: &State<CaseHandler>, group_id: &str) -> Result<Custom<Json<Vec<Case>>>, Custom<Json<ErrorResponse>>> {
    match handler.case.read_cases_by_group_id(group_id.into()).await {
        Ok(cases) => Ok(Custom(Status::Ok, Json(cases))),
        Err(e) => {
            eprintln!("error reading cases by group id: {}", e);
            Err(Custom(Status::InternalServerError, Json(ErrorResponse{error: "error reading cases by group id".into()})))
        }
    }

}