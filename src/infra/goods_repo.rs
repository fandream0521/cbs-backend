use crate::domain::goods::{Category, CategoryPagination, Goods, GoodsPagination};
use anyhow::Context;
use sqlx::{Row, SqlitePool};
use tracing::instrument;

pub async fn create_goods(
    pool: &SqlitePool,
    name: &str,
    old_price: Option<f64>,
    new_price: Option<f64>,
    desc: Option<&str>,
    status: Option<i32>,
    img_url: Option<&str>,
    inventory_count: Option<i64>,
    sale_count: Option<i64>,
    favor_count: Option<i64>,
    address: Option<&str>,
) -> anyhow::Result<i64> {
    let rec = sqlx::query(
        r#"
        INSERT INTO goods (name, old_price, new_price, "desc", status, img_url, inventory_count, sale_count, favor_count, address)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(old_price)
    .bind(new_price)
    .bind(desc)
    .bind(status)
    .bind(img_url)
    .bind(inventory_count)
    .bind(sale_count)
    .bind(favor_count)
    .bind(address)
    .fetch_one(pool)
    .await
    .context("insert goods")?;
    Ok(rec.get("id"))
}

pub async fn update_goods(
    pool: &SqlitePool,
    id: i64,
    name: Option<&str>,
    old_price: Option<f64>,
    new_price: Option<f64>,
    desc: Option<&str>,
    status: Option<i32>,
    img_url: Option<&str>,
    inventory_count: Option<i64>,
    sale_count: Option<i64>,
    favor_count: Option<i64>,
    address: Option<&str>,
) -> anyhow::Result<u64> {
    let res = sqlx::query(
        r#"
        UPDATE goods
        SET name = COALESCE(?2, name),
            old_price = COALESCE(?3, old_price),
            new_price = COALESCE(?4, new_price),
            "desc" = COALESCE(?5, "desc"),
            status = COALESCE(?6, status),
            img_url = COALESCE(?7, img_url),
            inventory_count = COALESCE(?8, inventory_count),
            sale_count = COALESCE(?9, sale_count),
            favor_count = COALESCE(?10, favor_count),
            address = COALESCE(?11, address),
            update_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(old_price)
    .bind(new_price)
    .bind(desc)
    .bind(status)
    .bind(img_url)
    .bind(inventory_count)
    .bind(sale_count)
    .bind(favor_count)
    .bind(address)
    .execute(pool)
    .await
    .context("update goods")?;
    Ok(res.rows_affected())
}

pub async fn delete_goods(pool: &SqlitePool, id: i64) -> anyhow::Result<u64> {
    let res = sqlx::query("DELETE FROM goods WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await
        .context("delete goods")?;
    Ok(res.rows_affected())
}

pub async fn get_goods(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Goods>> {
    let rec = sqlx::query_as::<_, Goods>(
        r#"
        SELECT id, name, old_price, new_price, "desc", status, img_url, inventory_count, sale_count, favor_count, address, create_at, update_at
        FROM goods
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .context("get goods")?;
    Ok(rec)
}

#[instrument(skip(pool, pagination))]
pub async fn list_goods(
    pool: &SqlitePool,
    pagination: &GoodsPagination,
) -> anyhow::Result<(Vec<Goods>, i64)> {
    let list = sqlx::query_as::<_, Goods>(
        r#"
        SELECT id, name, old_price, new_price, "desc", status, img_url, inventory_count, sale_count, favor_count, address, create_at, update_at
        FROM goods
        LIMIT ?1 OFFSET ?2
        "#,
    )
    .bind(pagination.size)
    .bind(pagination.offset)
    .fetch_all(pool)
    .await
    .context("list goods")?;

    let total: i64 = sqlx::query("SELECT COUNT(*) as count FROM goods")
        .fetch_one(pool)
        .await
        .context("count goods")?
        .get("count");

    Ok((list, total))
}

pub async fn create_category(
    pool: &SqlitePool,
    name: &str,
    parent_id: Option<i64>,
) -> anyhow::Result<i64> {
    let rec = sqlx::query(
        r#"
        INSERT INTO categories (name, parent_id)
        VALUES (?1, ?2)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(parent_id)
    .fetch_one(pool)
    .await
    .context("insert category")?;
    Ok(rec.get("id"))
}

pub async fn update_category(
    pool: &SqlitePool,
    id: i64,
    name: Option<&str>,
    parent_id: Option<i64>,
) -> anyhow::Result<u64> {
    let res = sqlx::query(
        r#"
        UPDATE categories
        SET name = COALESCE(?2, name),
            parent_id = COALESCE(?3, parent_id),
            update_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(parent_id)
    .execute(pool)
    .await
    .context("update category")?;
    Ok(res.rows_affected())
}

pub async fn delete_category(pool: &SqlitePool, id: i64) -> anyhow::Result<u64> {
    let res = sqlx::query("DELETE FROM categories WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await
        .context("delete category")?;
    Ok(res.rows_affected())
}

pub async fn get_category(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Category>> {
    let rec = sqlx::query_as::<_, Category>(
        r#"
        SELECT id, name, parent_id, create_at, update_at
        FROM categories
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .context("get category")?;
    Ok(rec)
}

#[instrument(skip(pool, pagination))]
pub async fn list_categories(
    pool: &SqlitePool,
    pagination: &CategoryPagination,
) -> anyhow::Result<(Vec<Category>, i64)> {
    let name_filter = pagination.name.clone().unwrap_or_default();
    let like = format!("%{}%", name_filter);
    let list = sqlx::query_as::<_, Category>(
        r#"
        SELECT id, name, parent_id, create_at, update_at
        FROM categories
        WHERE (?3 = '' OR name LIKE ?3)
        LIMIT ?1 OFFSET ?2
        "#,
    )
    .bind(pagination.size)
    .bind(pagination.offset)
    .bind(like.clone())
    .fetch_all(pool)
    .await
    .context("list categories")?;

    let total: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM categories
        WHERE (?1 = '' OR name LIKE ?1)
        "#,
    )
    .bind(like)
    .fetch_one(pool)
    .await
    .context("count categories")?
    .get("count");

    Ok((list, total))
}

