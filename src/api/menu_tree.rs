use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    api::ApiResponse,
    domain::system::Menu,
    infra::{state::AppState, system_repo},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignRoleMenusBody {
    pub role_id: i64,
    pub menu_list: Vec<i64>,
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct MenuTree {
    id: i64,
    name: String,
    r#type: i32,
    url: Option<String>,
    icon: Option<String>,
    sort: Option<i32>,
    parent_id: Option<i64>,
    create_at: Option<String>,
    update_at: Option<String>,
    children: Vec<MenuTree>,
}

#[tracing::instrument(skip(menus))]
fn build_menu_tree(menus: &[Menu]) -> Vec<MenuTree> {
    let mut tree: Vec<MenuTree> = Vec::new();
    let mut menu_map: std::collections::HashMap<i64, Vec<MenuTree>> = std::collections::HashMap::new();

    // First pass: create all menu nodes
    for menu in menus {
        let menu_node = MenuTree {
            id: menu.id,
            name: menu.name.clone(),
            r#type: menu.r#type,
            url: menu.url.clone(),
            icon: menu.icon.clone(),
            sort: menu.sort,
            parent_id: menu.parent_id,
            create_at: menu.create_at.clone(),
            update_at: menu.update_at.clone(),
            children: Vec::new(),
        };

        if let Some(parent_id) = menu.parent_id {
            menu_map.entry(parent_id).or_insert_with(Vec::new).push(menu_node);
        } else {
            tree.push(menu_node);
        }
    }

    // Second pass: build tree recursively
    fn build_children(
        parent_id: i64,
        menu_map: &std::collections::HashMap<i64, Vec<MenuTree>>,
    ) -> Vec<MenuTree> {
        menu_map
            .get(&parent_id)
            .map(|children| {
                children
                    .iter()
                    .map(|child| MenuTree {
                        children: build_children(child.id, menu_map),
                        ..child.clone()
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    tree.iter_mut()
        .for_each(|node| {
            node.children = build_children(node.id, &menu_map);
        });

    tree.sort_by_key(|m| m.sort.unwrap_or(0));
    tree
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/menu/tree", post(get_menu_tree))
        .route("/role/:role_id/menu", get(get_role_menu_tree))
        .route("/role/:role_id/menuIds", get(get_role_menu_ids))
        .route("/role/assign", post(assign_role_menus))
}

async fn get_menu_tree(
    State(state): State<AppState>,
    Json(_body): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<Vec<MenuTree>>>, crate::domain::error::DomainError> {
    let menus = system_repo::get_all_menus(&state.pool)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    let tree = build_menu_tree(&menus);
    Ok(Json(ApiResponse::success(tree)))
}

async fn get_role_menu_tree(
    State(state): State<AppState>,
    Path(role_id): Path<i64>,
) -> Result<Json<ApiResponse<Vec<MenuTree>>>, crate::domain::error::DomainError> {
    let menu_ids = system_repo::get_role_menu_ids(&state.pool, role_id)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    
    if menu_ids.is_empty() {
        return Ok(Json(ApiResponse::success(Vec::new())));
    }

    let all_menus = system_repo::get_all_menus(&state.pool)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    
    // Filter menus to only those assigned to the role
    let role_menus: Vec<Menu> = all_menus
        .into_iter()
        .filter(|m| menu_ids.contains(&m.id))
        .collect();
    
    let tree = build_menu_tree(&role_menus);
    Ok(Json(ApiResponse::success(tree)))
}

async fn get_role_menu_ids(
    State(state): State<AppState>,
    Path(role_id): Path<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    let menu_ids = system_repo::get_role_menu_ids(&state.pool, role_id)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "menuIds": menu_ids }),
    )))
}

async fn assign_role_menus(
    State(state): State<AppState>,
    Json(body): Json<AssignRoleMenusBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    system_repo::set_role_menus(&state.pool, body.role_id, &body.menu_list)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    
    let menu_ids = system_repo::get_role_menu_ids(&state.pool, body.role_id)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    
    // Build simplified tree for response
    let all_menus = system_repo::get_all_menus(&state.pool)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    let role_menus: Vec<Menu> = all_menus
        .into_iter()
        .filter(|m| menu_ids.contains(&m.id))
        .collect();
    let tree = build_menu_tree(&role_menus);
    
    Ok(Json(ApiResponse::success(
        serde_json::json!({
            "menuIds": menu_ids,
            "tree": tree
        }),
    )))
}

