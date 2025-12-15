use anyhow::Context;
use sqlx::{Row, SqlitePool};
use tracing::instrument;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCount {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategorySale {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryFavor {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopSale {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressSale {
    pub address: String,
    pub count: i64,
}

#[instrument(skip(pool))]
pub async fn get_category_counts(pool: &SqlitePool) -> anyhow::Result<Vec<CategoryCount>> {
    // Return category counts - since goods table doesn't have category_id, return category names with 0
    let results = sqlx::query(
        r#"
        SELECT name, 0 as value
        FROM categories
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await
    .context("get category counts")?;

    let mut counts = Vec::new();
    for row in results {
        counts.push(CategoryCount {
            name: row.get("name"),
            value: row.get("value"),
        });
    }
    Ok(counts)
}

#[instrument(skip(pool))]
pub async fn get_category_sales(pool: &SqlitePool) -> anyhow::Result<Vec<CategorySale>> {
    // Return category sales - since goods table doesn't have category_id, return category names with 0
    let results = sqlx::query(
        r#"
        SELECT name, 0 as value
        FROM categories
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await
    .context("get category sales")?;

    let mut sales = Vec::new();
    for row in results {
        sales.push(CategorySale {
            name: row.get("name"),
            value: row.get("value"),
        });
    }
    Ok(sales)
}

#[instrument(skip(pool))]
pub async fn get_category_favors(pool: &SqlitePool) -> anyhow::Result<Vec<CategoryFavor>> {
    // Return category favors - since goods table doesn't have category_id, return category names with 0
    let results = sqlx::query(
        r#"
        SELECT name, 0 as value
        FROM categories
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await
    .context("get category favors")?;

    let mut favors = Vec::new();
    for row in results {
        favors.push(CategoryFavor {
            name: row.get("name"),
            value: row.get("value"),
        });
    }
    Ok(favors)
}

#[instrument(skip(pool))]
pub async fn get_top_sales(pool: &SqlitePool) -> anyhow::Result<Vec<TopSale>> {
    let results = sqlx::query(
        r#"
        SELECT name, COALESCE(sale_count, 0) as value
        FROM goods
        ORDER BY COALESCE(sale_count, 0) DESC
        LIMIT 10
        "#,
    )
    .fetch_all(pool)
    .await
    .context("get top sales")?;

    let mut top_sales = Vec::new();
    for row in results {
        top_sales.push(TopSale {
            name: row.get("name"),
            value: row.get("value"),
        });
    }
    Ok(top_sales)
}

#[instrument(skip(pool))]
pub async fn get_address_sales(pool: &SqlitePool) -> anyhow::Result<Vec<AddressSale>> {
    let results = sqlx::query(
        r#"
        SELECT address, COUNT(*) as count
        FROM goods
        WHERE address IS NOT NULL AND address != ''
        GROUP BY address
        ORDER BY count DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .context("get address sales")?;

    let mut address_sales = Vec::new();
    for row in results {
        address_sales.push(AddressSale {
            address: row.get("address"),
            count: row.get("count"),
        });
    }
    Ok(address_sales)
}

#[instrument(skip(pool))]
pub async fn get_goods_amount_list(pool: &SqlitePool) -> anyhow::Result<Vec<CategoryCount>> {
    // This seems to be the same as category counts
    get_category_counts(pool).await
}

