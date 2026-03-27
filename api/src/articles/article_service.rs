use uuid::Uuid;

use crate::articles::{
    article_error::ArticleError,
    article_repository::ArticleRepository,
    dtos::{
        article_create_dto::ArticleCreateDto, article_response_dto::ArticleResponseDto,
        article_update_dto::ArticleUpdateDto,
    },
};

const LOG_CONTEXT: &str = "article_service";

pub async fn get_all_articles<R: ArticleRepository>(
    repo: R,
) -> Result<Vec<ArticleResponseDto>, ArticleError> {
    match repo.get_all_articles().await {
        Ok(articles) => Ok(articles.into_iter().map(ArticleResponseDto::from).collect()),
        Err(e) => {
            tracing::error!("{}: get_all_article error: {}", LOG_CONTEXT, e);
            Err(ArticleError::GetArticlesError)
        }
    }
}

pub async fn get_article_by_id<R: ArticleRepository>(
    repo: R,
    id: Uuid,
) -> Result<ArticleResponseDto, ArticleError> {
    match repo.get_article_by_id(id).await {
        Ok(Some(article)) => Ok(ArticleResponseDto::from(article)),
        Ok(None) => Err(ArticleError::ArticleNotFoundError),
        Err(e) => {
            tracing::error!("{}: get_article_by_id error: {}", LOG_CONTEXT, e);
            Err(ArticleError::GetArticleError)
        }
    }
}

pub async fn create_article<R: ArticleRepository>(
    repo: R,
    article_create_dto: ArticleCreateDto,
) -> Result<ArticleResponseDto, ArticleError> {
    let ArticleCreateDto {
        name,
        unit_of_measure,
    } = article_create_dto;

    match repo.create_article(name, unit_of_measure).await {
        Ok(article) => Ok(ArticleResponseDto::from(article)),
        Err(e) => {
            tracing::error!("{}: create_article error: {}", LOG_CONTEXT, e);
            Err(ArticleError::CreateArticleError)
        }
    }
}

pub async fn update_article<R: ArticleRepository>(
    repo: R,
    id: Uuid,
    article_update_dto: ArticleUpdateDto,
) -> Result<ArticleResponseDto, ArticleError> {
    let ArticleUpdateDto {
        name,
        unit_of_measure,
    } = article_update_dto;

    match repo.update_article(id, name, unit_of_measure).await {
        Ok(Some(article)) => Ok(ArticleResponseDto::from(article)),
        Ok(None) => Err(ArticleError::ArticleNotFoundError),
        Err(e) => {
            tracing::error!("{}: update_article error: {}", LOG_CONTEXT, e);
            Err(ArticleError::UpdateArticleError)
        }
    }
}

pub async fn soft_delete_article<R: ArticleRepository>(
    repo: R,
    id: Uuid,
) -> Result<(), ArticleError> {
    match repo.soft_delete_article(id).await {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(ArticleError::ArticleNotFoundError),
        Err(e) => {
            tracing::error!("{}: soft_delete_article error: {}", LOG_CONTEXT, e);
            Err(ArticleError::DeleteArticleError)
        }
    }
}
