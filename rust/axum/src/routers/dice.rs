use hyper::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};
use serde::Serialize;
use utoipa::ToSchema;
use axum::Json;
use axum::extract::Query;
use std::collections::HashMap;
use statistical::median;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
    .routes(routes!(roll_dice))
    .routes(routes!(multi_roll_dice))
}

#[derive(Serialize, ToSchema)]
struct SimpleRollResponseModel {
    dice_sides: u32, // 6
    result: u32, // 4
}

#[derive(Serialize, ToSchema)]
struct MultiRollResponseModel {
    dice_sides: u32, // 6
    results: Vec<u32>, // [4, 2, 5]
    min_value: u32, // 2
    max_value: u32, // 5
    average: f64, // 3.67
    median: u32, // 4
    total: u32, // 11
}

#[utoipa::path(
        get,
        path = "/roll",
        tag = "Dice",
        params(
            ("sides" = Option<u32>, Query, description = "Number of sides on the dice")
        ),
        responses(
            (status = 200, description = "Dice rolled successfully", body = SimpleRollResponseModel, example = json!({"dice_sides": 6, "result": 4})),
            (status = 400, description = "Invalid number of sides", body = String),
            (status = 500, description = "Internal server error", body = String)
))]
async fn roll_dice(Query(params) : Query<HashMap<String ,String>>) -> Result<Json<SimpleRollResponseModel>, StatusCode> {
    let sides = params.get("sides").and_then(|s| s.parse().ok()).unwrap_or(6);
    if sides < 1 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let dice = crate::dice::Dice::new(sides);
    let result = dice.roll();
    Ok(Json(SimpleRollResponseModel { dice_sides: sides, result }))
}

#[utoipa::path(
        get,
        path = "/multi-roll",
        tag = "Dice",
        params(
            ("sides" = Option<u32>, Query, description = "Number of sides on the dice"),
            ("rolls" = u32, Query, description = "Number of rolls to perform")
        ),
        responses(
            (status = 200, description = "Dice rolled successfully", body = MultiRollResponseModel, example = json!({"dice_sides": 6, "results": [4, 2, 5], "min_value": 2, "max_value": 5, "average": 3.67, "median": 4, "total": 11})),
            (status = 400, description = "Invalid number of sides", body = String),
            (status = 500, description = "Internal server error", body = String)
))]
async fn multi_roll_dice(Query(params) : Query<HashMap<String ,String>>) -> Result<Json<MultiRollResponseModel>, StatusCode> {
    let sides = params.get("sides").and_then(|s| s.parse().ok()).unwrap_or(6);
    let rolls = params.get("rolls").and_then(|r| r.parse().ok()).unwrap_or(1);
    if sides < 1 || rolls < 1 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let dice = crate::dice::Dice::new(sides);
    let results = (0..rolls).map(|_| dice.roll()).collect::<Vec<_>>();
    let min_value = *results.iter().min().unwrap_or(&0);
    let max_value = *results.iter().max().unwrap_or(&0);
    let total: u32 = results.iter().sum();
    let average = total as f64 / rolls as f64;
    let median = median(&results);
    Ok(Json(MultiRollResponseModel { dice_sides: sides, results, min_value, max_value, average, median, total }))
}