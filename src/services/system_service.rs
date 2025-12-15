use crate::{
    domain::error::DomainError,
    domain::system::{Department, Menu, Pagination, Role, User},
    infra::system_repo,
};
use sqlx::SqlitePool;

pub struct SystemService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SystemService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        name: &str,
        realname: &str,
        password: &str,
        cellphone: Option<&str>,
        department_id: Option<i64>,
        role_id: Option<i64>,
    ) -> Result<i64, DomainError> {
        system_repo::create_user(
            self.pool,
            name,
            realname,
            password,
            cellphone,
            department_id,
            role_id,
        )
        .await
        .map_err(DomainError::Internal)
    }

    pub async fn update_user(
        &self,
        id: i64,
        password: Option<&str>,
        cellphone: Option<&str>,
    ) -> Result<(), DomainError> {
        let rows = system_repo::update_user(self.pool, id, password, cellphone)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), DomainError> {
        let rows = system_repo::delete_user(self.pool, id)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn get_user(&self, id: i64) -> Result<User, DomainError> {
        system_repo::get_user(self.pool, id)
            .await
            .map_err(DomainError::Internal)?
            .ok_or(DomainError::NotFound)
    }

    pub async fn list_users(
        &self,
        pagination: &Pagination,
    ) -> Result<(Vec<User>, i64), DomainError> {
        system_repo::list_users(self.pool, pagination)
            .await
            .map_err(DomainError::Internal)
    }

    pub async fn create_department(
        &self,
        name: &str,
        parent_id: Option<i64>,
        leader: Option<&str>,
    ) -> Result<i64, DomainError> {
        system_repo::create_department(self.pool, name, parent_id, leader)
            .await
            .map_err(DomainError::Internal)
    }

    pub async fn update_department(
        &self,
        id: i64,
        parent_id: Option<i64>,
        leader: Option<&str>,
    ) -> Result<(), DomainError> {
        let rows = system_repo::update_department(self.pool, id, parent_id, leader)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn delete_department(&self, id: i64) -> Result<(), DomainError> {
        let rows = system_repo::delete_department(self.pool, id)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn get_department(&self, id: i64) -> Result<Department, DomainError> {
        system_repo::get_department(self.pool, id)
            .await
            .map_err(DomainError::Internal)?
            .ok_or(DomainError::NotFound)
    }

    pub async fn list_departments(
        &self,
        pagination: &Pagination,
    ) -> Result<(Vec<Department>, i64), DomainError> {
        system_repo::list_departments(self.pool, pagination)
            .await
            .map_err(DomainError::Internal)
    }

    pub async fn create_role(&self, name: &str, intro: Option<&str>) -> Result<i64, DomainError> {
        system_repo::create_role(self.pool, name, intro)
            .await
            .map_err(DomainError::Internal)
    }

    pub async fn update_role(&self, id: i64, intro: Option<&str>) -> Result<(), DomainError> {
        let rows = system_repo::update_role(self.pool, id, intro)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn delete_role(&self, id: i64) -> Result<(), DomainError> {
        let rows = system_repo::delete_role(self.pool, id)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn get_role(&self, id: i64) -> Result<Role, DomainError> {
        system_repo::get_role(self.pool, id)
            .await
            .map_err(DomainError::Internal)?
            .ok_or(DomainError::NotFound)
    }

    pub async fn list_roles(
        &self,
        pagination: &Pagination,
    ) -> Result<(Vec<Role>, i64), DomainError> {
        system_repo::list_roles(self.pool, pagination)
            .await
            .map_err(DomainError::Internal)
    }

    pub async fn create_menu(
        &self,
        name: &str,
        menu_type: i32,
        url: Option<&str>,
        icon: Option<&str>,
        sort: Option<i32>,
        parent_id: Option<i64>,
    ) -> Result<i64, DomainError> {
        system_repo::create_menu(self.pool, name, menu_type, url, icon, sort, parent_id)
            .await
            .map_err(DomainError::Internal)
    }

    pub async fn update_menu(
        &self,
        id: i64,
        name: Option<&str>,
        menu_type: Option<i32>,
        url: Option<&str>,
        icon: Option<&str>,
        sort: Option<i32>,
        parent_id: Option<i64>,
    ) -> Result<(), DomainError> {
        let rows =
            system_repo::update_menu(self.pool, id, name, menu_type, url, icon, sort, parent_id)
                .await
                .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn delete_menu(&self, id: i64) -> Result<(), DomainError> {
        let rows = system_repo::delete_menu(self.pool, id)
            .await
            .map_err(DomainError::Internal)?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    pub async fn get_menu(&self, id: i64) -> Result<Menu, DomainError> {
        system_repo::get_menu(self.pool, id)
            .await
            .map_err(DomainError::Internal)?
            .ok_or(DomainError::NotFound)
    }

    pub async fn list_menus(
        &self,
        pagination: &Pagination,
    ) -> Result<(Vec<Menu>, i64), DomainError> {
        system_repo::list_menus(self.pool, pagination)
            .await
            .map_err(DomainError::Internal)
    }
}
