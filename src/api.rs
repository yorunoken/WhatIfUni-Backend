use std::{collections::HashMap, f64::consts::E, fmt::Write, sync::Arc};

use chrono::Utc;
use rosu_v2::prelude::*;
use rosu_v2::Osu;

use serde::Deserialize;
use sqlx::{Row, SqlitePool};
use warp::{reject::Rejection, reply::Reply};

use crate::methods::ValorantRank;
use crate::models::OsuResponse;
use crate::models::{Ayt, EstimateRankResponse, Tyt};

const TOTAL_OSU: u32 = 1_000_000;
const TOTAL_YKS: u32 = 2_819_362;

pub async fn get_tyt(
    query: HashMap<String, String>,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let mut sql_query = String::from("SELECT * FROM tytData");

    let custom_query = query.get("query");

    match custom_query {
        Some(standalone_query) => {
            let _ = write!(sql_query, " {}", standalone_query);
        }
        None => {
            if !query.is_empty() {
                let mut conditions: Vec<String> = Vec::new();
                for (key, value) in query {
                    conditions.push(format!("{} = '{}'", key, value))
                }
                let where_clause = conditions.join(" AND ");

                let _ = write!(sql_query, " WHERE {}", where_clause);
            }
        }
    }

    let rows = sqlx::query(&sql_query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("failed to get tyt query: {e}");
            warp::reject::not_found()
        })?;

    let tyt: Vec<Tyt> = rows
        .into_iter()
        .map(|row| Tyt {
            yop_code: row.get("yop_code"),
            university_name: row.get("university_name"),
            faculty: row.get("faculty"),
            class_name: row.get("class_name"),
            education_style: row.get("education_style"),
            education_duration: row.get("education_duration"),
            city: row.get("city"),
            university_style: row.get("university_style"),
            scholarship_rate: row.get("scholarship_rate"),
            student_status_2024: row.get("student_status_2024"),
            student_status_2023: row.get("student_status_2023"),
            student_quota_2024: row.get("student_quota_2024"),
            student_quota_2023: row.get("student_quota_2023"),
            tbs_2024: row.get("tbs_2024"),
            tbs_2023: row.get("tbs_2023"),
            base_score_2024: row.get("base_score_2024"),
            base_score_2023: row.get("base_score_2023"),
        })
        .collect();

    Ok(warp::reply::json(&tyt))
}

pub async fn get_ayt(
    exam_type: String,
    query: HashMap<String, String>,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let mut sql_query = format!("SELECT * FROM {}Data", exam_type);

    let custom_query = query.get("query");

    match custom_query {
        Some(standalone_query) => {
            let _ = write!(sql_query, " {}", standalone_query);
        }
        None => {
            if !query.is_empty() {
                let mut conditions: Vec<String> = Vec::new();
                for (key, value) in query {
                    conditions.push(format!("{} = '{}'", key, value))
                }
                let where_clause = conditions.join(" AND ");

                let _ = write!(sql_query, " WHERE {}", where_clause);
            }
        }
    }

    let rows = sqlx::query(&sql_query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("error while getting ayt query: {e}");
            warp::reject::not_found()
        })?;

    let ayt: Vec<Ayt> = rows
        .into_iter()
        .map(|row| Ayt {
            yop_code: row.get("yop_code"),
            university_name: row.get("university_name"),
            faculty: row.get("faculty"),
            class_name: row.get("class_name"),
            education_style: row.get("education_style"),
            education_duration: row.get("education_duration"),
            city: row.get("city"),
            university_style: row.get("university_style"),
            scholarship_rate: row.get("scholarship_rate"),
            student_quota_2024: row.get("student_quota_2024"),
            student_quota_2023: row.get("student_quota_2023"),
            student_quota_2022: row.get("student_quota_2022"),
            student_quota_2021: row.get("student_quota_2021"),
            fullness_status: row.get("fullness_status"),
            enrolled_2024: row.get("enrolled_2024"),
            enrolled_2023: row.get("enrolled_2023"),
            enrolled_2022: row.get("enrolled_2022"),
            enrolled_2021: row.get("enrolled_2021"),
            tbs_2024: row.get("tbs_2024"),
            tbs_2023: row.get("tbs_2023"),
            tbs_2022: row.get("tbs_2022"),
            tbs_2021: row.get("tbs_2021"),
            base_score_2024: row.get("base_score_2024"),
            base_score_2023: row.get("base_score_2023"),
            base_score_2022: row.get("base_score_2022"),
            base_score_2021: row.get("base_score_2021"),
        })
        .collect();

    Ok(warp::reply::json(&ayt))
}

pub async fn estimate_valorant_rank(rank: String) -> Result<impl Reply, Rejection> {
    let distribution_array: Vec<f64> = vec![
        35251.0, 93687.0, 206510.0, 225147.0, 300778.0, 266653.0, 318142.0, 279157.0, 283079.0,
        279748.0, 237958.0, 205472.0, 186473.0, 150596.0, 132228.0, 122509.0, 95984.0, 74587.0,
        56836.0, 36195.0, 21824.0, 13987.0, 4140.0, 3904.0, 567.0,
    ];

    let total_players: f64 = distribution_array.iter().sum();

    // https://tracker.gg/valorant/leaderboards/ranked/pc/default?page=1&region=eu&act=52ca6698-41c1-e7de-4008-8994d2221209
    let valorant_rank = ValorantRank::new(&rank.to_lowercase());

    // This ranges from 0 to 24, from bad rank to good rank
    let valorant_rank_number = valorant_rank.to_number();

    if valorant_rank_number >= distribution_array.len() {
        println!("valorant_rank_number is bigger than array length.");
        return Err(warp::reject::reject());
    }

    let players_above: f64 = distribution_array
        .iter()
        .skip(valorant_rank_number + 1)
        .sum();

    let players_at_rank = distribution_array[valorant_rank_number];

    let percentile = (players_above + (players_at_rank / 2.0)) / total_players;

    let estimated_rank = (percentile * total_players).round() as u64 + 1; // Add 1 to avoid rank 0

    Ok(warp::reply::json(&EstimateRankResponse {
        // Divide by 2 halve the rank, because I feel like it fits better that way
        estimate_rank: estimated_rank / 2,
    }))
}

pub async fn estimate_cs2_rank(elo: usize) -> Result<impl Reply, Rejection> {
    const BASE_ELO: usize = 1000;
    const DECAY_RATE: f64 = 0.0004;

    let normalized_elo = (elo - BASE_ELO).max(0);

    // Implementation of percentile below
    // https://www.desmos.com/calculator/8lpthjvbni
    let percentile = 100.0 * E.powf(-DECAY_RATE * normalized_elo as f64);

    let rank_position = ((percentile / 100.0) * TOTAL_YKS as f64).round() as u64;

    Ok(warp::reply::json(&EstimateRankResponse {
        estimate_rank: rank_position,
    }))
}

pub async fn get_osu_user(username: String, osu: Arc<Osu>) -> Result<impl Reply, Rejection> {
    let small_username: SmallString<[u8; 15]> = username.into();

    match osu.user(UserId::Name(small_username)).await {
        Ok(user) => match &user.statistics {
            Some(statistics) => match statistics.global_rank {
                Some(global_rank) => {
                    let percentage = global_rank as f32 / TOTAL_OSU as f32;
                    let new_rank = (percentage * TOTAL_YKS as f32).round() as u32;

                    Ok(warp::reply::json(&OsuResponse {
                        percentage: Some(percentage * 100.0), // multiply by 100 so you get a 100 based percentage, x.xx%
                        rank: Some(new_rank),
                        username: user.username.into_string(),
                    }))
                }
                None => Ok(warp::reply::json(&OsuResponse {
                    percentage: None,
                    rank: None,
                    username: user.username.into_string(),
                })),
            },
            None => Ok(warp::reply::json(&OsuResponse {
                percentage: None,
                rank: None,
                username: user.username.into_string(),
            })),
        },
        Err(e) => {
            eprintln!("osu! api error: {}", e);
            Err(warp::reject::not_found())
        }
    }
}

// POST endpoints

#[derive(Deserialize)]
pub struct Feedback {
    // This needs to be from 1 to 5
    stars: u8,
    game: String,
}
pub async fn feedback(feedback: Feedback, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let current_date = Utc::now().format("%Y-%m-%d").to_string();

    sqlx::query!(
        "INSERT INTO feedback (stars, game, date) VALUES (?, ?, ?)",
        feedback.stars,
        feedback.game,
        current_date,
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to insert feedback: {:?}", e);
        warp::reject::reject()
    })?;

    Ok(warp::reply::with_status(
        "Feedback submitted successfully",
        warp::http::StatusCode::CREATED,
    ))
}
