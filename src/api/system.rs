use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{patch, post},
};
use serde::Deserialize;

use crate::{
    api::ApiResponse,
    domain::system::{Department, Menu, Pagination, Role, User},
    infra::state::AppState,
    services::system_service::SystemService,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserBody {
    pub name: String,
    pub realname: String,
    pub password: String,
    pub cellphone: Option<String>,
    pub department_id: Option<i64>,
    pub role_id: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserBody {
    pub password: Option<String>,
    pub cellphone: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDepartmentBody {
    pub name: String,
    pub parent_id: Option<i64>,
    pub leader: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDepartmentBody {
    pub parent_id: Option<i64>,
    pub leader: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleBody {
    pub name: String,
    pub intro: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleBody {
    pub intro: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMenuBody {
    pub name: String,
    pub r#type: i32,
    pub url: Option<String>,
    pub icon: Option<String>,
    pub sort: Option<i32>,
    pub parent_id: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMenuBody {
    pub name: Option<String>,
    pub r#type: Option<i32>,
    pub url: Option<String>,
    pub icon: Option<String>,
    pub sort: Option<i32>,
    pub parent_id: Option<i64>,
}

fn service(state: &AppState) -> SystemService<'_> {
    SystemService::new(&state.pool)
}

fn validate_pagination(p: &Pagination) -> Result<(), crate::domain::error::DomainError> {
    if p.size <= 0 || p.size > 100 {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "size must be between 1 and 100".into(),
        ));
    }
    if p.offset < 0 {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "offset must be >= 0".into(),
        ));
    }
    Ok(())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        // Users
        .route("/users", post(create_user))
        .route(
            "/users/:id",
            patch(update_user).delete(delete_user).get(get_user),
        )
        .route("/users/list", post(list_users))
        // Departments
        .route("/department", post(create_department))
        .route(
            "/department/:id",
            patch(update_department)
                .delete(delete_department)
                .get(get_department),
        )
        .route("/department/list", post(list_departments))
        // Roles
        .route("/role", post(create_role))
        .route(
            "/role/:id",
            patch(update_role).delete(delete_role).get(get_role),
        )
        .route("/role/list", post(list_roles))
        // Menus
        .route("/menu", post(create_menu))
        .route(
            "/menu/:id",
            patch(update_menu).delete(delete_menu).get(get_menu),
        )
        .route("/menu/list", post(list_menus))
}

async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    if body.name.trim().is_empty() || body.password.trim().is_empty() {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "name and password are required".into(),
        ));
    }

    let id = service(&state)
        .create_user(
            &body.name,
            &body.realname,
            &body.password,
            body.cellphone.as_deref(),
            body.department_id,
            body.role_id,
        )
        .await?;
    Ok(Json(ApiResponse::success(serde_json::json!({ "id": id }))))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateUserBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state)
        .update_user(id, body.password.as_deref(), body.cellphone.as_deref())
        .await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "updated": true }),
    )))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state).delete_user(id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "deleted": true }),
    )))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<User>>, crate::domain::error::DomainError> {
    let user = service(&state).get_user(id).await?;
    Ok(Json(ApiResponse::success(user)))
}

async fn list_users(
    State(state): State<AppState>,
    Json(pagination): Json<Pagination>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    validate_pagination(&pagination)?;
    let (list, total) = service(&state).list_users(&pagination).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "list": list, "totalCount": total }),
    )))
}

async fn create_department(
    State(state): State<AppState>,
    Json(body): Json<CreateDepartmentBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    if body.name.trim().is_empty() {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "department name is required".into(),
        ));
    }
    let id = service(&state)
        .create_department(&body.name, body.parent_id, body.leader.as_deref())
        .await?;
    Ok(Json(ApiResponse::success(serde_json::json!({ "id": id }))))
}

async fn update_department(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateDepartmentBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state)
        .update_department(id, body.parent_id, body.leader.as_deref())
        .await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "updated": true }),
    )))
}

async fn delete_department(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state).delete_department(id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "deleted": true }),
    )))
}

async fn get_department(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Department>>, crate::domain::error::DomainError> {
    let dept = service(&state).get_department(id).await?;
    Ok(Json(ApiResponse::success(dept)))
}

async fn list_departments(
    State(state): State<AppState>,
    Json(pagination): Json<Pagination>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    validate_pagination(&pagination)?;
    let (list, total) = service(&state).list_departments(&pagination).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "list": list, "totalCount": total }),
    )))
}

async fn create_role(
    State(state): State<AppState>,
    Json(body): Json<CreateRoleBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    if body.name.trim().is_empty() {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "role name is required".into(),
        ));
    }
    let id = service(&state)
        .create_role(&body.name, body.intro.as_deref())
        .await?;
    Ok(Json(ApiResponse::success(serde_json::json!({ "id": id }))))
}

async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateRoleBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state)
        .update_role(id, body.intro.as_deref())
        .await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "updated": true }),
    )))
}

async fn delete_role(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state).delete_role(id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "deleted": true }),
    )))
}

async fn get_role(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Role>>, crate::domain::error::DomainError> {
    let role = service(&state).get_role(id).await?;
    Ok(Json(ApiResponse::success(role)))
}

async fn list_roles(
    State(state): State<AppState>,
    Json(pagination): Json<Pagination>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    validate_pagination(&pagination)?;
    let (list, total) = service(&state).list_roles(&pagination).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "list": list, "totalCount": total }),
    )))
}

async fn create_menu(
    State(state): State<AppState>,
    Json(body): Json<CreateMenuBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    if body.name.trim().is_empty() {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "menu name is required".into(),
        ));
    }
    if !(1..=3).contains(&body.r#type) {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "menu type must be 1, 2, or 3".into(),
        ));
    }
    let id = service(&state)
        .create_menu(
            &body.name,
            body.r#type,
            body.url.as_deref(),
            body.icon.as_deref(),
            body.sort,
            body.parent_id,
        )
        .await?;
    Ok(Json(ApiResponse::success(serde_json::json!({ "id": id }))))
}

async fn update_menu(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateMenuBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    if let Some(t) = body.r#type {
        if !(1..=3).contains(&t) {
            return Err(crate::domain::error::DomainError::InvalidInput(
                "menu type must be 1, 2, or 3".into(),
            ));
        }
    }
    service(&state)
        .update_menu(
            id,
            body.name.as_deref(),
            body.r#type,
            body.url.as_deref(),
            body.icon.as_deref(),
            body.sort,
            body.parent_id,
        )
        .await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "updated": true }),
    )))
}

async fn delete_menu(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state).delete_menu(id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "deleted": true }),
    )))
}

async fn get_menu(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Menu>>, crate::domain::error::DomainError> {
    let menu = service(&state).get_menu(id).await?;
    Ok(Json(ApiResponse::success(menu)))
}

async fn list_menus(
    State(state): State<AppState>,
    Json(pagination): Json<Pagination>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    validate_pagination(&pagination)?;
    let (list, total) = service(&state).list_menus(&pagination).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "list": list, "totalCount": total }),
    )))
}
