use std::collections::HashMap;

use sqlx::{Row, SqlitePool};
use warp::{reject::Rejection, reply::Reply};

use crate::models::{Ayt, Tyt};

pub async fn get_tyt(
    query: HashMap<String, String>,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let mut sql_query = String::from("SELECT * FROM tytData");

    if !query.is_empty() {
        let mut conditions: Vec<String> = Vec::new();
        for (key, value) in query {
            conditions.push(format!("{} = '{}'", key, value))
        }
        let where_clause = conditions.join(" AND ");
        sql_query.push_str(" WHERE ");
        sql_query.push_str(&where_clause);
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

    if !query.is_empty() {
        let mut conditions: Vec<String> = Vec::new();
        for (key, value) in query {
            conditions.push(format!("{} = '{}'", key, value))
        }
        let where_clause = conditions.join(" AND ");
        sql_query.push_str(" WHERE ");
        sql_query.push_str(&where_clause);
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
