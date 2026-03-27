use async_trait::async_trait;
use chrono::Utc;
use sqlx::Error;
use std::sync::Mutex;
use uuid::Uuid;

use crate::articles::{
    article_model::{ArticleModel, UnitOfMeasure},
    article_repository::ArticleRepository,
};

fn get_invalid_article() -> ArticleModel {
    ArticleModel {
        id: Uuid::new_v4(),
        name: "invalid name".to_string(),
        unit_of_measure: UnitOfMeasure::Kg,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    }
}

pub struct ArticleMockRepository {
    pub get_all_result: Option<Result<Vec<ArticleModel>, ()>>,
    pub get_by_id_result: Option<Result<ArticleModel, ()>>,
    pub create_result: Option<Result<ArticleModel, ()>>,
    pub update_result: Option<Result<ArticleModel, ()>>,
    pub soft_delete_result: Option<Result<ArticleModel, ()>>,
    pub captured_get_by_id_args: Mutex<Option<Uuid>>,
    pub captured_create_args: Mutex<Option<(String, UnitOfMeasure)>>,
    pub captured_update_args: Mutex<Option<(Uuid, Option<String>, Option<UnitOfMeasure>)>>,
    pub captured_soft_delete_args: Mutex<Option<Uuid>>,
}

impl ArticleMockRepository {
    pub fn new() -> Self {
        Self {
            get_all_result: None,
            get_by_id_result: None,
            create_result: None,
            update_result: None,
            soft_delete_result: None,
            captured_get_by_id_args: None.into(),
            captured_create_args: None.into(),
            captured_update_args: None.into(),
            captured_soft_delete_args: None.into(),
        }
    }
}

#[async_trait]
impl ArticleRepository for &ArticleMockRepository {
    async fn get_all_articles(&self) -> Result<Vec<ArticleModel>, Error> {
        match &self.get_all_result {
            Some(Ok(res)) => Ok(res.clone()),
            Some(Err(_)) => Err(Error::RowNotFound),
            None => Ok(vec![get_invalid_article()]),
        }
    }

    async fn get_article_by_id(&self, id: Uuid) -> Result<Option<ArticleModel>, Error> {
        *self.captured_get_by_id_args.lock().unwrap() = Some(id);

        match &self.get_by_id_result {
            Some(Ok(res)) => Ok(Some(res.clone())),
            Some(Err(_)) => Err(Error::RowNotFound),
            None => Ok(None),
        }
    }

    async fn create_article(
        &self,
        name: String,
        unit_of_measure: UnitOfMeasure,
    ) -> Result<ArticleModel, Error> {
        *self.captured_create_args.lock().unwrap() = Some((name, unit_of_measure));

        match &self.create_result {
            Some(Ok(res)) => Ok(res.clone()),
            Some(Err(_)) => Err(Error::RowNotFound),
            None => Ok(get_invalid_article()),
        }
    }

    async fn update_article(
        &self,
        id: Uuid,
        name: Option<String>,
        unit_of_measure: Option<UnitOfMeasure>,
    ) -> Result<Option<ArticleModel>, Error> {
        *self.captured_update_args.lock().unwrap() = Some((id, name, unit_of_measure));

        match &self.update_result {
            Some(Ok(res)) => Ok(Some(res.clone())),
            Some(Err(_)) => Err(Error::RowNotFound),
            None => Ok(None),
        }
    }

    async fn soft_delete_article(&self, id: Uuid) -> Result<Option<ArticleModel>, Error> {
        *self.captured_soft_delete_args.lock().unwrap() = Some(id);

        match &self.soft_delete_result {
            Some(Ok(res)) => Ok(Some(res.clone())),
            Some(Err(_)) => Err(Error::RowNotFound),
            None => Ok(None),
        }
    }
}
