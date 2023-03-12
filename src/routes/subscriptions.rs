use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{self, Instrument};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UserData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<UserData>, db_pool: web::Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    tracing::info!("Starting insert subscriber");
    let query_span = tracing::info_span!("Entering insert query");
    match sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("Insert successful for {}", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query for id: {} - {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
