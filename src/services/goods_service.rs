use crate::{
    domain::error::DomainError,
    domain::goods::{Category, CategoryPagination, Goods, GoodsPagination},
    infra::goods_repo,
};
use sqlx::SqlitePool;

pub struct GoodsService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> GoodsService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_goods(
        &self,
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
    ) -> Result<i64, DomainError> {
        goods_repo::create_goods(
            self.pool,
            name,
            old_price,
            new_price,
            desc,
            status,
            img_url,
            inventory_count,
            sale_count,
            favor_count,
            address,
        )
        .await
        .map_err(DomainError::Internal)
    }

    pub async fn update_goods(
        &self,
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
    ) -> Result<(), DomainError> {
        let rows = goods_repo::update_goods(
            self.pool,
            id,
            name,
            old_price,
            new_price,
            desc,
            status,
            img_url,
            inventory_count,
            sale_count,
            favor_count,
            address,
        )
        .await
        .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn delete_goods(&self, id: i64) -> Result<(), DomainError> {
        let rows = goods_repo::delete_goods(self.pool, id)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn get_goods(&self, id: i64) -> Result<Goods, DomainError> {
        goods_repo::get_goods(self.pool, id)
            .await
            .map_err(DomainError::Internal)?
            .ok_or(DomainError::NotFound)
    }

    pub async fn list_goods(
        &self,
        pagination: &GoodsPagination,
    ) -> Result<(Vec<Goods>, i64), DomainError> {
        goods_repo::list_goods(self.pool, pagination)
            .await
            .map_err(DomainError::Internal)
    }

    pub async fn create_category(
        &self,
        name: &str,
        parent_id: Option<i64>,
    ) -> Result<i64, DomainError> {
        goods_repo::create_category(self.pool, name, parent_id)
            .await
            .map_err(DomainError::Internal)
    }

    pub async fn update_category(
        &self,
        id: i64,
        name: Option<&str>,
        parent_id: Option<i64>,
    ) -> Result<(), DomainError> {
        let rows = goods_repo::update_category(self.pool, id, name, parent_id)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn delete_category(&self, id: i64) -> Result<(), DomainError> {
        let rows = goods_repo::delete_category(self.pool, id)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn get_category(&self, id: i64) -> Result<Category, DomainError> {
        goods_repo::get_category(self.pool, id)
            .await
            .map_err(DomainError::Internal)?
            .ok_or(DomainError::NotFound)
    }

    pub async fn list_categories(
        &self,
        pagination: &CategoryPagination,
    ) -> Result<(Vec<Category>, i64), DomainError> {
        goods_repo::list_categories(self.pool, pagination)
            .await
            .map_err(DomainError::Internal)
    }
}

