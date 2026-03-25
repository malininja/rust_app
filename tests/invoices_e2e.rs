use reqwest::StatusCode;
use rust_app::{
    AppState,
    articles::{
        article_model::UnitOfMeasure,
        dtos::{article_create_dto::ArticleCreateDto, article_response_dto::ArticleResponseDto},
    },
    invoices::dtos::{
        invoice_head_create_dto::InvoiceHeadCreateDto,
        invoice_head_response_dto::InvoiceHeadResponseDto,
        invoice_head_update_dto::InvoiceHeadUpdateDto,
        invoice_item_create_dto::InvoiceItemCreateDto,
    },
};
use rust_decimal::dec;
use sqlx::PgPool;
use uuid::Uuid;

use crate::common::{TEST_JWT_SECRET, create_test_app, get_user_token};

mod common;

#[sqlx::test]
async fn test_invoice_router(pool: PgPool) {
    let test_app = create_test_app(AppState {
        pool,
        jwt_secret: TEST_JWT_SECRET.to_string(),
    })
    .await;

    let token = get_user_token(&test_app).await;

    let articles_url = format!("http://{}:{}/articles", test_app.base_url, test_app.port);

    let invoices_url = format!(
        "http://{}:{}/invoices",
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

    //############# CREATE INVOICE
    let invoice_item_create_dtos = create_invoice_item_dtos(article_ids);

    let invoice_head_create_dto = InvoiceHeadCreateDto {
        customer_name: "supplier one".to_string(),
        items: invoice_item_create_dtos.clone(),
    };

    let invoice_create_response = reqwest_client
        .post(&invoices_url)
        .header(header_name, &header_value)
        .json(&invoice_head_create_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::CREATED, invoice_create_response.status());

    let invoice_create_result = invoice_create_response
        .json::<InvoiceHeadResponseDto>()
        .await
        .unwrap();

    assert_eq!(
        "supplier one".to_string(),
        invoice_create_result.customer_name
    );
    assert_eq!(false, invoice_create_result.confirmed);
    assert_eq!(
        invoice_item_create_dtos[0].article_id,
        invoice_create_result
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
        invoice_create_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 1)
            .unwrap()
            .quantity
    );
    assert_eq!(
        invoice_item_create_dtos[1].article_id,
        invoice_create_result
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
        invoice_create_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 2)
            .unwrap()
            .quantity
    );

    //############# UPDATE INVOICE
    let mut invoice_item_update_dtos = invoice_item_create_dtos.clone();
    invoice_item_update_dtos[0].quantity = dec!(5.55);
    invoice_item_update_dtos[1].quantity = dec!(66.6);

    let invoice_update_dto = InvoiceHeadUpdateDto {
        customer_name: Some("some other supplier".to_string()),
        confirmed: None,
        items: Some(invoice_item_update_dtos),
    };

    let invoice_update_response = reqwest_client
        .patch(format!(
            "{}/{}",
            &invoices_url, invoice_create_result.id
        ))
        .header(header_name, &header_value)
        .json(&invoice_update_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, invoice_update_response.status());

    let invoice_update_result = invoice_update_response
        .json::<InvoiceHeadResponseDto>()
        .await
        .unwrap();

    assert_eq!(
        "some other supplier".to_string(),
        invoice_update_result.customer_name
    );
    assert_eq!(false, invoice_update_result.confirmed);
    assert_eq!(
        invoice_item_create_dtos[0].article_id,
        invoice_update_result
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
        invoice_update_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 1)
            .unwrap()
            .quantity
    );
    assert_eq!(
        invoice_item_create_dtos[1].article_id,
        invoice_update_result
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
        invoice_update_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 2)
            .unwrap()
            .quantity
    );

    //############# GET INVOICE

    let get_invoice_response = reqwest_client
        .get(format!(
            "{}/{}",
            &invoices_url, invoice_create_result.id
        ))
        .header(header_name, &header_value)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, get_invoice_response.status());

    let get_invoice_result = get_invoice_response
        .json::<InvoiceHeadResponseDto>()
        .await
        .unwrap();

    assert_eq!(
        "some other supplier".to_string(),
        get_invoice_result.customer_name
    );
    assert_eq!(false, get_invoice_result.confirmed);
    assert_eq!(
        invoice_item_create_dtos[0].article_id,
        get_invoice_result
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
        get_invoice_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 1)
            .unwrap()
            .quantity
    );
    assert_eq!(
        invoice_item_create_dtos[1].article_id,
        get_invoice_result
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
        get_invoice_result
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|i| i.ordinal == 2)
            .unwrap()
            .quantity
    );

    //############# GET ALL INVOICE

    let get_all_response = reqwest_client
        .get(invoices_url)
        .header(header_name, header_value)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, get_all_response.status());

    let get_all_result = get_all_response
        .json::<Vec<InvoiceHeadResponseDto>>()
        .await
        .unwrap();

    assert_eq!(1, get_all_result.len());
    assert_eq!(
        "some other supplier".to_string(),
        get_all_result[0].customer_name
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

fn create_invoice_item_dtos(ids: Vec<Uuid>) -> Vec<InvoiceItemCreateDto> {
    vec![
        InvoiceItemCreateDto {
            article_id: ids[0],
            ordinal: 1,
            quantity: dec!(12.3),
        },
        InvoiceItemCreateDto {
            article_id: ids[1],
            ordinal: 2,
            quantity: dec!(3.21),
        },
    ]
}
