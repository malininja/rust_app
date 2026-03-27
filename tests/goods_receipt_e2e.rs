use reqwest::StatusCode;
use rust_app::{
    AppState,
    articles::{
        article_model::UnitOfMeasure,
        dtos::{article_create_dto::ArticleCreateDto, article_response_dto::ArticleResponseDto},
    },
    goods_receipts::dtos::{
        goods_receipt_head_create_dto::GoodsReceiptHeadCreateDto,
        goods_receipt_head_response_dto::GoodsReceiptHeadResponseDto,
        goods_receipt_head_update_dto::GoodsReceiptHeadUpdateDto,
        goods_receipt_item_create_dto::GoodsReceiptItemCreateDto,
    },
};
use rust_decimal::dec;
use sqlx::PgPool;
use uuid::Uuid;

use crate::common::{TEST_JWT_SECRET, create_test_app, get_user_token};

mod common;

#[sqlx::test]
async fn test_goods_receipt_router(pool: PgPool) {
    let test_app = create_test_app(AppState {
        pool,
        jwt_secret: TEST_JWT_SECRET.to_string(),
    })
    .await;

    let token = get_user_token(&test_app).await;

    let articles_url = format!("http://{}:{}/articles", test_app.base_url, test_app.port);

    let goods_receipts_url = format!(
        "http://{}:{}/goods_receipts",
        test_app.base_url, test_app.port
    );

    let reqwest_client = reqwest::Client::new();

    let header_name = "Authorization";
    let header_value = format!("Bearer {}", token);

    //############# CREATE ARTICLES
    let articles_create_dtos = create_article_dtos();

    let article_1_response = reqwest_client
        .post(articles_url.clone())
        .header(header_name, &header_value)
        .json(&articles_create_dtos[0])
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::CREATED, article_1_response.status());

    let article_1 = article_1_response
        .json::<ArticleResponseDto>()
        .await
        .unwrap();

    let article_2_response = reqwest_client
        .post(articles_url)
        .header(header_name, &header_value)
        .json(&articles_create_dtos[0])
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::CREATED, article_2_response.status());

    let article_2 = article_2_response
        .json::<ArticleResponseDto>()
        .await
        .unwrap();

    let article_ids = vec![article_1.id, article_2.id];

    //############# CREATE GOODS RECEIPT
    let goods_receipt_item_create_dtos = create_goods_receipt_item_dtos(article_ids);

    let goods_receipt_head_create_dto = GoodsReceiptHeadCreateDto {
        supplier_name: "supplier one".to_string(),
        items: goods_receipt_item_create_dtos.clone(),
    };

    let goods_receipt_create_response = reqwest_client
        .post(&goods_receipts_url)
        .header(header_name, &header_value)
        .json(&goods_receipt_head_create_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::CREATED, goods_receipt_create_response.status());

    let goods_receipt_create_result = goods_receipt_create_response
        .json::<GoodsReceiptHeadResponseDto>()
        .await
        .unwrap();

    assert_eq!(
        "supplier one".to_string(),
        goods_receipt_create_result.supplier_name
    );
    assert_eq!(false, goods_receipt_create_result.confirmed);
    assert_eq!(
        goods_receipt_item_create_dtos[0].article_id,
        goods_receipt_create_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 1)
            .unwrap()
            .article_id
    );
    assert_eq!(
        dec!(12.3),
        goods_receipt_create_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 1)
            .unwrap()
            .quantity
    );
    assert_eq!(
        goods_receipt_item_create_dtos[1].article_id,
        goods_receipt_create_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 2)
            .unwrap()
            .article_id
    );
    assert_eq!(
        dec!(3.21),
        goods_receipt_create_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 2)
            .unwrap()
            .quantity
    );

    //############# UPDATE GOODS RECEIPT
    let mut goods_receipt_item_update_dtos = goods_receipt_item_create_dtos.clone();
    goods_receipt_item_update_dtos[0].quantity = dec!(5.55);
    goods_receipt_item_update_dtos[1].quantity = dec!(66.6);

    let goods_receipt_update_dto = GoodsReceiptHeadUpdateDto {
        supplier_name: Some("some other supplier".to_string()),
        items: Some(goods_receipt_item_update_dtos),
    };

    let goods_receipt_update_response = reqwest_client
        .patch(format!(
            "{}/{}",
            &goods_receipts_url, goods_receipt_create_result.id
        ))
        .header(header_name, &header_value)
        .json(&goods_receipt_update_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, goods_receipt_update_response.status());

    let goods_receipt_update_result = goods_receipt_update_response
        .json::<GoodsReceiptHeadResponseDto>()
        .await
        .unwrap();

    assert_eq!(
        "some other supplier".to_string(),
        goods_receipt_update_result.supplier_name
    );
    assert_eq!(false, goods_receipt_update_result.confirmed);
    assert_eq!(
        goods_receipt_item_create_dtos[0].article_id,
        goods_receipt_update_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 1)
            .unwrap()
            .article_id
    );
    assert_eq!(
        dec!(5.55),
        goods_receipt_update_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 1)
            .unwrap()
            .quantity
    );
    assert_eq!(
        goods_receipt_item_create_dtos[1].article_id,
        goods_receipt_update_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 2)
            .unwrap()
            .article_id
    );
    assert_eq!(
        dec!(66.6),
        goods_receipt_update_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 2)
            .unwrap()
            .quantity
    );

    //############# GET GOODS RECEIPT

    let get_goods_receipt_response = reqwest_client
        .get(format!(
            "{}/{}",
            &goods_receipts_url, goods_receipt_create_result.id
        ))
        .header(header_name, &header_value)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, get_goods_receipt_response.status());

    let get_goods_receipt_result = get_goods_receipt_response
        .json::<GoodsReceiptHeadResponseDto>()
        .await
        .unwrap();

    assert_eq!(
        "some other supplier".to_string(),
        get_goods_receipt_result.supplier_name
    );
    assert_eq!(false, get_goods_receipt_result.confirmed);
    assert_eq!(
        goods_receipt_item_create_dtos[0].article_id,
        get_goods_receipt_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 1)
            .unwrap()
            .article_id
    );
    assert_eq!(
        dec!(5.55),
        get_goods_receipt_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 1)
            .unwrap()
            .quantity
    );
    assert_eq!(
        goods_receipt_item_create_dtos[1].article_id,
        get_goods_receipt_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 2)
            .unwrap()
            .article_id
    );
    assert_eq!(
        dec!(66.6),
        get_goods_receipt_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 2)
            .unwrap()
            .quantity
    );

    //############# GET ALL GOODS RECEIPT

    let get_all_response = reqwest_client
        .get(goods_receipts_url)
        .header(header_name, header_value)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, get_all_response.status());

    let get_all_result = get_all_response
        .json::<Vec<GoodsReceiptHeadResponseDto>>()
        .await
        .unwrap();

    assert_eq!(1, get_all_result.len());
    assert_eq!(
        "some other supplier".to_string(),
        get_all_result[0].supplier_name
    );
    assert_eq!(None, get_all_result[0].items);
}

fn create_article_dtos() -> Vec<ArticleCreateDto> {
    vec![
        ArticleCreateDto {
            name: "article one".to_string(),
            unit_of_measure: UnitOfMeasure::Kg,
        },
        ArticleCreateDto {
            name: "article two".to_string(),
            unit_of_measure: UnitOfMeasure::Litre,
        },
    ]
}

fn create_goods_receipt_item_dtos(ids: Vec<Uuid>) -> Vec<GoodsReceiptItemCreateDto> {
    vec![
        GoodsReceiptItemCreateDto {
            article_id: ids[0],
            ordinal: 1,
            quantity: dec!(12.3),
        },
        GoodsReceiptItemCreateDto {
            article_id: ids[1],
            ordinal: 2,
            quantity: dec!(3.21),
        },
    ]
}
