use axum::extract::State;
use chrono::Utc;
use sea_orm::*;

use crate::entity::plusplus;
use crate::entity::plusplus::Entity as Plusplus;

use crate::AppState;

pub async fn get(state: &State<AppState>, slack_id: &str) -> Option<plusplus::Model> {
    tracing::info!("plusplus__get");
    match Plusplus::find()
        .filter(plusplus::Column::SlackId.eq(slack_id))
        .one(&state.conn)
        .await
    {
        Ok(res) => {
            tracing::info!("{:?}", res);
            return res;
        }
        Err(e) => {
            tracing::error!("{:?}", e);
            return None;
        }
    };
}

pub async fn create(state: State<AppState>, slack_id: &str) -> plusplus::Model {
    tracing::info!("plusplus__create");
    let plus_active = plusplus::ActiveModel {
        slack_id: ActiveValue::Set(slack_id.to_string()),
        counter: ActiveValue::Set(1),
        created_at: ActiveValue::Set(Utc::now()),
        updated_at: ActiveValue::Set(Utc::now()),
        ..Default::default()
    };

    let plus_model = plus_active.insert(&state.conn).await.unwrap();

    plus_model

    // match result {
    //     Ok (plus_model) => {
    //         tracing::info!("{:?}", plus_model);
    //         return plus_model
    //     },
    //     Err(e) => {
    //         tracing::error!("{:?}", e);
    //         return None
    //     },
    // };
}

pub async fn update(state: State<AppState>, plusplus: plusplus::Model) -> plusplus::Model {
    tracing::info!("plusplus__update");
    let one_up = &plusplus.counter + 1;
    let mut plus_active: plusplus::ActiveModel = plusplus.into();
    plus_active.counter = ActiveValue::Set(one_up);
    plus_active.updated_at = ActiveValue::Set(Utc::now());

    let plus_model = plus_active.update(&state.conn).await.unwrap();

    // match result {
    //     Ok (plus_model) => {
    //         tracing::info!("{:?}", plus_model);
    //         return plus_model
    //     },
    //     Err(e) => {
    //         tracing::error!("{:?}", e);
    //         return None
    //     },
    // };

    plus_model
}
