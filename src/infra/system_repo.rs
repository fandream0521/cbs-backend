use crate::domain::system::{Department, Menu, Pagination, Role, User};
use anyhow::Context;
use sqlx::{Row, SqlitePool};
use tracing::instrument;

pub async fn create_user(
    pool: &SqlitePool,
    name: &str,
    realname: &str,
    password: &str,
    cellphone: Option<&str>,
    department_id: Option<i64>,
    role_id: Option<i64>,
) -> anyhow::Result<i64> {
    let rec = sqlx::query(
        r#"
        INSERT INTO users (name, realname, password, cellphone, department_id, role_id, enable)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(realname)
    .bind(password)
    .bind(cellphone)
    .bind(department_id)
    .bind(role_id)
    .fetch_one(pool)
    .await
    .context("insert user")?;
    Ok(rec.get("id"))
}

pub async fn update_user(
    pool: &SqlitePool,
    id: i64,
    password: Option<&str>,
    cellphone: Option<&str>,
) -> anyhow::Result<u64> {
    let res = sqlx::query(
        r#"
        UPDATE users
        SET password = COALESCE(?2, password),
            cellphone = COALESCE(?3, cellphone),
            update_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .bind(password)
    .bind(cellphone)
    .execute(pool)
    .await
    .context("update user")?;
    Ok(res.rows_affected())
}

pub async fn delete_user(pool: &SqlitePool, id: i64) -> anyhow::Result<u64> {
    let res = sqlx::query("DELETE FROM users WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await
        .context("delete user")?;
    Ok(res.rows_affected())
}

pub async fn get_user(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<User>> {
    let rec = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, realname, cellphone, enable, department_id, role_id, create_at, update_at
        FROM users
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .context("get user")?;
    Ok(rec)
}

pub async fn find_user_by_credentials(
    pool: &SqlitePool,
    name: &str,
    password: &str,
) -> anyhow::Result<Option<User>> {
    let rec = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, realname, cellphone, enable, department_id, role_id, create_at, update_at
        FROM users
        WHERE name = ?1 AND password = ?2 AND enable = 1
        "#,
    )
    .bind(name)
    .bind(password)
    .fetch_optional(pool)
    .await
    .context("find user by credentials")?;
    Ok(rec)
}

#[instrument(skip(pool, pagination))]
pub async fn list_users(
    pool: &SqlitePool,
    pagination: &Pagination,
) -> anyhow::Result<(Vec<User>, i64)> {
    let name_filter = pagination.name.clone().unwrap_or_default();
    let like = format!("%{}%", name_filter);
    let list = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, realname, cellphone, enable, department_id, role_id, create_at, update_at
        FROM users
        WHERE (?3 = '' OR name LIKE ?3)
        LIMIT ?1 OFFSET ?2
        "#,
    )
    .bind(pagination.size)
    .bind(pagination.offset)
    .bind(like.clone())
    .fetch_all(pool)
    .await
    .context("list users")?;

    let total: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM users
        WHERE (?1 = '' OR name LIKE ?1)
        "#,
    )
    .bind(like)
    .fetch_one(pool)
    .await
    .context("count users")?
    .get("count");

    Ok((list, total))
}

#[instrument(skip(pool))]
pub async fn get_all_menus(pool: &SqlitePool) -> anyhow::Result<Vec<Menu>> {
    let list = sqlx::query_as::<_, Menu>(
        r#"
        SELECT id, name, type as "r#type", url, icon, sort, parent_id, create_at, update_at
        FROM menus
        ORDER BY sort ASC, id ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .context("get all menus")?;
    Ok(list)
}

pub async fn get_role_menu_ids(pool: &SqlitePool, role_id: i64) -> anyhow::Result<Vec<i64>> {
    let menu_ids = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT menu_id
        FROM role_menus
        WHERE role_id = ?1
        ORDER BY menu_id
        "#,
    )
    .bind(role_id)
    .fetch_all(pool)
    .await
    .context("get role menu ids")?;
    Ok(menu_ids)
}

pub async fn set_role_menus(
    pool: &SqlitePool,
    role_id: i64,
    menu_ids: &[i64],
) -> anyhow::Result<()> {
    // Delete existing role menus
    sqlx::query("DELETE FROM role_menus WHERE role_id = ?1")
        .bind(role_id)
        .execute(pool)
        .await
        .context("delete role menus")?;

    // Insert new role menus
    for menu_id in menu_ids {
        sqlx::query("INSERT INTO role_menus (role_id, menu_id) VALUES (?1, ?2)")
            .bind(role_id)
            .bind(menu_id)
            .execute(pool)
            .await
            .context("insert role menu")?;
    }

    Ok(())
}

pub async fn create_department(
    pool: &SqlitePool,
    name: &str,
    parent_id: Option<i64>,
    leader: Option<&str>,
) -> anyhow::Result<i64> {
    let rec = sqlx::query(
        r#"
        INSERT INTO departments (name, parent_id, leader)
        VALUES (?1, ?2, ?3)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(parent_id)
    .bind(leader)
    .fetch_one(pool)
    .await
    .context("insert department")?;
    Ok(rec.get("id"))
}

pub async fn update_department(
    pool: &SqlitePool,
    id: i64,
    parent_id: Option<i64>,
    leader: Option<&str>,
) -> anyhow::Result<u64> {
    let res = sqlx::query(
        r#"
        UPDATE departments
        SET parent_id = COALESCE(?2, parent_id),
            leader = COALESCE(?3, leader),
            update_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .bind(parent_id)
    .bind(leader)
    .execute(pool)
    .await
    .context("update department")?;
    Ok(res.rows_affected())
}

pub async fn delete_department(pool: &SqlitePool, id: i64) -> anyhow::Result<u64> {
    let res = sqlx::query("DELETE FROM departments WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await
        .context("delete department")?;
    Ok(res.rows_affected())
}

pub async fn get_department(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Department>> {
    let rec = sqlx::query_as::<_, Department>(
        r#"
        SELECT id, name, parent_id, leader, create_at, update_at
        FROM departments
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .context("get department")?;
    Ok(rec)
}

#[instrument(skip(pool, pagination))]
pub async fn list_departments(
    pool: &SqlitePool,
    pagination: &Pagination,
) -> anyhow::Result<(Vec<Department>, i64)> {
    let like = format!("%{}%", pagination.name.clone().unwrap_or_default());
    let list = sqlx::query_as::<_, Department>(
        r#"
        SELECT id, name, parent_id, leader, create_at, update_at
        FROM departments
        WHERE (?3 = '' OR name LIKE ?3)
        LIMIT ?1 OFFSET ?2
        "#,
    )
    .bind(pagination.size)
    .bind(pagination.offset)
    .bind(like.clone())
    .fetch_all(pool)
    .await
    .context("list departments")?;

    let total: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM departments
        WHERE (?1 = '' OR name LIKE ?1)
        "#,
    )
    .bind(like)
    .fetch_one(pool)
    .await
    .context("count departments")?
    .get("count");

    Ok((list, total))
}

pub async fn create_role(
    pool: &SqlitePool,
    name: &str,
    intro: Option<&str>,
) -> anyhow::Result<i64> {
    let rec = sqlx::query(
        r#"
        INSERT INTO roles (name, intro)
        VALUES (?1, ?2)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(intro)
    .fetch_one(pool)
    .await
    .context("insert role")?;
    Ok(rec.get("id"))
}

pub async fn update_role(pool: &SqlitePool, id: i64, intro: Option<&str>) -> anyhow::Result<u64> {
    let res = sqlx::query(
        r#"
        UPDATE roles
        SET intro = COALESCE(?2, intro),
            update_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .bind(intro)
    .execute(pool)
    .await
    .context("update role")?;
    Ok(res.rows_affected())
}

pub async fn delete_role(pool: &SqlitePool, id: i64) -> anyhow::Result<u64> {
    let res = sqlx::query("DELETE FROM roles WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await
        .context("delete role")?;
    Ok(res.rows_affected())
}

pub async fn get_role(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Role>> {
    let rec = sqlx::query_as::<_, Role>(
        r#"
        SELECT id, name, intro, create_at, update_at
        FROM roles
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .context("get role")?;
    Ok(rec)
}

#[instrument(skip(pool, pagination))]
pub async fn list_roles(
    pool: &SqlitePool,
    pagination: &Pagination,
) -> anyhow::Result<(Vec<Role>, i64)> {
    let like = format!("%{}%", pagination.name.clone().unwrap_or_default());
    let list = sqlx::query_as::<_, Role>(
        r#"
        SELECT id, name, intro, create_at, update_at
        FROM roles
        WHERE (?3 = '' OR name LIKE ?3)
        LIMIT ?1 OFFSET ?2
        "#,
    )
    .bind(pagination.size)
    .bind(pagination.offset)
    .bind(like.clone())
    .fetch_all(pool)
    .await
    .context("list roles")?;

    let total: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM roles
        WHERE (?1 = '' OR name LIKE ?1)
        "#,
    )
    .bind(like)
    .fetch_one(pool)
    .await
    .context("count roles")?
    .get("count");

    Ok((list, total))
}

pub async fn create_menu(
    pool: &SqlitePool,
    name: &str,
    menu_type: i32,
    url: Option<&str>,
    icon: Option<&str>,
    sort: Option<i32>,
    parent_id: Option<i64>,
) -> anyhow::Result<i64> {
    let rec = sqlx::query(
        r#"
        INSERT INTO menus (name, type, url, icon, sort, parent_id)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(menu_type)
    .bind(url)
    .bind(icon)
    .bind(sort)
    .bind(parent_id)
    .fetch_one(pool)
    .await
    .context("insert menu")?;
    Ok(rec.get("id"))
}

pub async fn update_menu(
    pool: &SqlitePool,
    id: i64,
    name: Option<&str>,
    menu_type: Option<i32>,
    url: Option<&str>,
    icon: Option<&str>,
    sort: Option<i32>,
    parent_id: Option<i64>,
) -> anyhow::Result<u64> {
    let res = sqlx::query(
        r#"
        UPDATE menus
        SET name = COALESCE(?2, name),
            type = COALESCE(?3, type),
            url = COALESCE(?4, url),
            icon = COALESCE(?5, icon),
            sort = COALESCE(?6, sort),
            parent_id = COALESCE(?7, parent_id),
            update_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(menu_type)
    .bind(url)
    .bind(icon)
    .bind(sort)
    .bind(parent_id)
    .execute(pool)
    .await
    .context("update menu")?;
    Ok(res.rows_affected())
}

pub async fn delete_menu(pool: &SqlitePool, id: i64) -> anyhow::Result<u64> {
    let res = sqlx::query("DELETE FROM menus WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await
        .context("delete menu")?;
    Ok(res.rows_affected())
}

pub async fn get_menu(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Menu>> {
    let rec = sqlx::query_as::<_, Menu>(
        r#"
        SELECT id, name, type as "r#type", url, icon, sort, parent_id, create_at, update_at
        FROM menus
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .context("get menu")?;
    Ok(rec)
}

#[instrument(skip(pool, pagination))]
pub async fn list_menus(
    pool: &SqlitePool,
    pagination: &Pagination,
) -> anyhow::Result<(Vec<Menu>, i64)> {
    let like = format!("%{}%", pagination.name.clone().unwrap_or_default());
    let list = sqlx::query_as::<_, Menu>(
        r#"
        SELECT id, name, type as "r#type", url, icon, sort, parent_id, create_at, update_at
        FROM menus
        WHERE (?3 = '' OR name LIKE ?3)
        LIMIT ?1 OFFSET ?2
        "#,
    )
    .bind(pagination.size)
    .bind(pagination.offset)
    .bind(like.clone())
    .fetch_all(pool)
    .await
    .context("list menus")?;

    let total: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM menus
        WHERE (?1 = '' OR name LIKE ?1)
        "#,
    )
    .bind(like)
    .fetch_one(pool)
    .await
    .context("count menus")?
    .get("count");

    Ok((list, total))
}
