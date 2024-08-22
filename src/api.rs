use std::fmt::Write;
use std::{collections::HashMap, sync::Arc};

use rosu_v2::prelude::*;
use rosu_v2::Osu;

use sqlx::{Row, SqlitePool};
use warp::{reject::Rejection, reply::Reply};

use crate::methods::ValorantRank;
use crate::models::{Ayt, EstimateRankResponse, Tyt};

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
        .map_err(|_| warp::reject::not_found())?;

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
        .map_err(|_| warp::reject::not_found())?;

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
    let mut cumulative_distribution: Vec<f64> = Vec::with_capacity(distribution_array.len());
    let mut sum: f64 = 0.0;
    for &value in &distribution_array {
        sum += value;
        cumulative_distribution.push(sum / total_players);
    }

    // https://tracker.gg/valorant/leaderboards/ranked/pc/default?page=1&region=eu&act=52ca6698-41c1-e7de-4008-8994d2221209
    let valorant_rank = ValorantRank::new(&rank.to_lowercase());

    // This ranges from 0 to 24, from bad rank to good rank
    let valorant_rank_number = valorant_rank.to_number();

    if valorant_rank_number >= distribution_array.len() {
        return Err(warp::reject::reject());
    }

    let reversed_rank_percentage = 1.0 - cumulative_distribution[valorant_rank_number];

    let min_threshold = 0.00005; // Adjust this value as needed
    let estimated_rank =
        (reversed_rank_percentage * total_players).max(total_players * min_threshold) as u64;

    Ok(warp::reply::json(&EstimateRankResponse {
        estimate_rank: estimated_rank,
    }))
}

pub async fn get_osu_user(username: String, osu: Arc<Osu>) -> Result<impl Reply, Rejection> {
    let small_username: SmallString<[u8; 15]> = username.into();

    match osu.user(UserId::Name(small_username)).await {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(_) => Err(warp::reject::not_found()),
    }
}
