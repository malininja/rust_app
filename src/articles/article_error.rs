use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ArticleError {
    #[error("Get articles error")]
    GetArticlesError,

    #[error("Get article error")]
    GetArticleError,

    #[error("Article is not found")]
    ArticleNotFoundError,

    #[error("Create article error")]
    CreateArticleError,

    #[error("Update article error")]
    UpdateArticleError,

    #[error("Delete article error")]
    DeleteArticleError,
}
