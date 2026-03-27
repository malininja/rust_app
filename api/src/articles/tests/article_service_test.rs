use crate::articles::{
    article_error::ArticleError,
    article_model::{ArticleModel, UnitOfMeasure},
    article_service::{
        create_article, get_all_articles, get_article_by_id, soft_delete_article, update_article,
    },
    dtos::{
        article_create_dto::ArticleCreateDto, article_response_dto::ArticleResponseDto,
        article_update_dto::ArticleUpdateDto,
    },
    tests::article_mock_repository::ArticleMockRepository,
};
use chrono::Utc;
use uuid::Uuid;

//############# get_all_articles

fn get_article() -> ArticleModel {
    ArticleModel {
        id: Uuid::new_v4(),
        name: "test article".to_string(),
        unit_of_measure: UnitOfMeasure::Piece,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    }
}

#[tokio::test]
async fn get_all_articles_success() {
    let article = get_article();

    let mut repo = ArticleMockRepository::new();
    repo.get_all_result = Some(Ok(vec![article.clone()]));

    let result = get_all_articles(&repo).await.unwrap();

    assert_eq!(1, result.len());
    assert_eq!(ArticleResponseDto::from(article), result[0]);
}

#[tokio::test]
async fn get_all_articles_error() {
    let mut repo = ArticleMockRepository::new();
    repo.get_all_result = Some(Err(()));

    let result = get_all_articles(&repo).await.unwrap_err();

    assert_eq!(ArticleError::GetArticlesError, result);
}

//############# get_article_by_id

#[tokio::test]
async fn get_article_by_id_success() {
    let id = Uuid::new_v4();
    let article = get_article();

    let mut repo = ArticleMockRepository::new();
    repo.get_by_id_result = Some(Ok(article.clone()));

    let result = get_article_by_id(&repo, id).await.unwrap();

    let captured_id = (*repo.captured_get_by_id_args.lock().unwrap()).unwrap();

    assert_eq!(id, captured_id);
    assert_eq!(ArticleResponseDto::from(article), result);
}

#[tokio::test]
async fn get_article_by_id_not_found() {
    let id = Uuid::new_v4();

    let repo = ArticleMockRepository::new();

    let result = get_article_by_id(&repo, id).await.unwrap_err();

    let captured_id = (*repo.captured_get_by_id_args.lock().unwrap()).unwrap();

    assert_eq!(id, captured_id);
    assert_eq!(ArticleError::ArticleNotFoundError, result);
}

#[tokio::test]
async fn get_article_by_id_error() {
    let id = Uuid::new_v4();

    let mut repo = ArticleMockRepository::new();
    repo.get_by_id_result = Some(Err(()));

    let result = get_article_by_id(&repo, id).await.unwrap_err();

    let captured_id = (*repo.captured_get_by_id_args.lock().unwrap()).unwrap();

    assert_eq!(id, captured_id);
    assert_eq!(ArticleError::GetArticleError, result);
}

//############# create_article

#[tokio::test]
async fn create_article_success() {
    let article_create_dto = ArticleCreateDto {
        name: "some name".to_string(),
        unit_of_measure: UnitOfMeasure::Litre,
    };

    let article_model = get_article();

    let mut repo = ArticleMockRepository::new();
    repo.create_result = Some(Ok(article_model.clone()));

    let result = create_article(&repo, article_create_dto.clone())
        .await
        .unwrap();

    let captured_args = repo.captured_create_args.lock().unwrap().take().unwrap();

    assert_eq!(article_create_dto.name, captured_args.0);
    assert_eq!(article_create_dto.unit_of_measure, captured_args.1);
    assert_eq!(ArticleResponseDto::from(article_model), result);
}

#[tokio::test]
async fn create_article_error() {
    let article_create_dto = ArticleCreateDto {
        name: "some name".to_string(),
        unit_of_measure: UnitOfMeasure::Litre,
    };

    let mut repo = ArticleMockRepository::new();
    repo.create_result = Some(Err(()));

    let result = create_article(&repo, article_create_dto.clone())
        .await
        .unwrap_err();

    let captured_args = repo.captured_create_args.lock().unwrap().take().unwrap();

    assert_eq!(article_create_dto.name, captured_args.0);
    assert_eq!(article_create_dto.unit_of_measure, captured_args.1);
    assert_eq!(ArticleError::CreateArticleError, result);
}

#[tokio::test]
async fn update_article_success() {
    let id = Uuid::new_v4();
    let article_update_dto = ArticleUpdateDto {
        name: Some("some name".to_string()),
        unit_of_measure: Some(UnitOfMeasure::Metre),
    };

    let article_model = get_article();

    let mut repo = ArticleMockRepository::new();
    repo.update_result = Some(Ok(article_model.clone()));

    let result = update_article(&repo, id, article_update_dto.clone())
        .await
        .unwrap();

    let captured_args = repo.captured_update_args.lock().unwrap().take().unwrap();

    let expected_args = (
        id,
        article_update_dto.name,
        article_update_dto.unit_of_measure,
    );

    assert_eq!(expected_args, captured_args);
    assert_eq!(ArticleResponseDto::from(article_model), result);
}

#[tokio::test]
async fn update_article_not_found() {
    let id = Uuid::new_v4();
    let article_update_dto = ArticleUpdateDto {
        name: Some("some name".to_string()),
        unit_of_measure: Some(UnitOfMeasure::Metre),
    };

    let repo = ArticleMockRepository::new();

    let result = update_article(&repo, id, article_update_dto.clone())
        .await
        .unwrap_err();

    let captured_args = repo.captured_update_args.lock().unwrap().take().unwrap();

    let expected_args = (
        id,
        article_update_dto.name,
        article_update_dto.unit_of_measure,
    );

    assert_eq!(expected_args, captured_args);
    assert_eq!(ArticleError::ArticleNotFoundError, result);
}

#[tokio::test]
async fn update_article_error() {
    let id = Uuid::new_v4();
    let article_update_dto = ArticleUpdateDto {
        name: Some("some name".to_string()),
        unit_of_measure: Some(UnitOfMeasure::Metre),
    };

    let mut repo = ArticleMockRepository::new();
    repo.update_result = Some(Err(()));

    let result = update_article(&repo, id, article_update_dto.clone())
        .await
        .unwrap_err();

    let captured_args = repo.captured_update_args.lock().unwrap().take().unwrap();

    let expected_args = (
        id,
        article_update_dto.name,
        article_update_dto.unit_of_measure,
    );

    assert_eq!(expected_args, captured_args);
    assert_eq!(ArticleError::UpdateArticleError, result);
}

//############# soft_delete_article

#[tokio::test]
async fn soft_delete_article_success() {
    let id = Uuid::new_v4();

    let mut repo = ArticleMockRepository::new();
    repo.soft_delete_result = Some(Ok(get_article()));

    let result = soft_delete_article(&repo, id).await.unwrap();

    let captured_args = repo
        .captured_soft_delete_args
        .lock()
        .unwrap()
        .take()
        .unwrap();

    assert_eq!(id, captured_args);
    assert_eq!((), result);
}

#[tokio::test]
async fn soft_delete_not_found() {
    let id = Uuid::new_v4();

    let repo = ArticleMockRepository::new();

    let result = soft_delete_article(&repo, id).await.unwrap_err();

    let captured_args = repo
        .captured_soft_delete_args
        .lock()
        .unwrap()
        .take()
        .unwrap();

    assert_eq!(id, captured_args);
    assert_eq!(ArticleError::ArticleNotFoundError, result);
}

#[tokio::test]
async fn soft_delete_error() {
    let id = Uuid::new_v4();

    let mut repo = ArticleMockRepository::new();
    repo.soft_delete_result = Some(Err(()));

    let result = soft_delete_article(&repo, id).await.unwrap_err();

    let captured_args = repo
        .captured_soft_delete_args
        .lock()
        .unwrap()
        .take()
        .unwrap();

    assert_eq!(id, captured_args);
    assert_eq!(ArticleError::DeleteArticleError, result);
}
