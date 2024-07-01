use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::http::ContentType;

use crate::types::case_database::Case;
use crate::types::case_handler::{CreateCIS18CaseBody, CreateCaseResponse, RenameCaseBody};
use crate::types::ErrorResponse;
use crate::types::case_database::GroupCases;
use crate::database::case::CaseDatabase;
use crate::api::middleware_handler::AuthorizeClientGuard;


#[post("/api/case/cis18/create", data = "<data>")]
pub async fn create_cis18_case(
    _guard: AuthorizeClientGuard,
    case_database: &State<CaseDatabase>,
    data: Json<CreateCIS18CaseBody>
) -> Result<Custom<Json<CreateCaseResponse>>, Custom<Json<ErrorResponse>>> {
    match case_database.create_cis18_case(&data.group_id, &data.name, data.implementation_group).await {
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
pub async fn rename_case(
    _guard: AuthorizeClientGuard,
    case_database: &State<CaseDatabase>, 
    case_id: String, 
    data: Json<RenameCaseBody>
) -> Result<Custom<String>, Custom<Json<ErrorResponse>>> {
    match case_database.rename_case(&case_id, &data.name).await {
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
pub async fn delete_case(_guard: AuthorizeClientGuard, case_database: &State<CaseDatabase>, case_id: String) -> Result<Custom<String>, Custom<Json<ErrorResponse>>> {
    match case_database.delete_case(&case_id).await {
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
pub async fn get_case(_guard: AuthorizeClientGuard, case_database: &State<CaseDatabase>, case_id: &str) -> Result<Custom<Json<Case>>, Custom<Json<ErrorResponse>> > {
    match case_database.read_case_by_id(case_id.into()).await {
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

#[post("/api/case/list", data = "<group_ids>")]
pub async fn get_cases(_guard: AuthorizeClientGuard, case_database: &State<CaseDatabase>, group_ids: Json<Vec<&str>>) -> Result<Custom<Json<Vec<GroupCases>>>, Custom<Json<ErrorResponse>>> {
    match case_database.read_cases_sorted_by_group(group_ids.to_vec()).await {
        Ok(cases) => Ok(Custom(Status::Ok, Json(cases))),
        Err(e) => {
            eprintln!("error reading cases by group id: {}", e);
            Err(Custom(Status::InternalServerError, Json(ErrorResponse{error: "error reading cases by group id".into()})))
        }
    }
}

#[get("/api/case/<case_id>/export/docx")]
pub async fn export_case_docx(_guard: AuthorizeClientGuard, case_database: &State<CaseDatabase>, case_id: &str, client: &State<reqwest::Client>) -> Result<(ContentType, Vec<u8>), Custom<Json<ErrorResponse>>> {

    let case = match case_database.read_case_by_id(case_id.to_string()).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("error reading case by id: {}", e);
            return Err(Custom(Status::InternalServerError, Json(ErrorResponse::new("error reading case by id"))))
        }
    };

    let json_parsed = match serde_json::to_string(&case) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error serializing case data: {}", e);
            return Err(Custom(Status::InternalServerError, Json(ErrorResponse::new("internal error"))))
        }
    };

    let export_service_domain = "https://export.service.altiore.io"; // local export has been node dependency fuckd

    let response = client.post(format!("{}/api/export/docx", export_service_domain))
        .header("content-type", "application/json")
        .body(json_parsed)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                // Read the entire response body as bytes
                match resp.bytes().await {
                    Ok(bytes) => {
                        Ok((ContentType::new("application", "vnd.openxmlformats-officedocument.wordprocessingml.document"), bytes.to_vec()))
                    },
                    Err(_) => Err(rocket::response::status::Custom(
                        rocket::http::Status::InternalServerError,
                        Json(ErrorResponse::new("failed to read document data")),
                    )),
                }
            } else {
                Err(rocket::response::status::Custom(
                    rocket::http::Status::BadRequest,
                    Json(ErrorResponse::new("bad response from export service")),
                ))
            }
        },
        Err(_) => Err(rocket::response::status::Custom(
            rocket::http::Status::InternalServerError,
            Json(ErrorResponse::new("failed to connect to document service")),
        )),
    }

}