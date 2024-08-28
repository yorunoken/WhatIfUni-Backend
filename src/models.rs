use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tyt {
    pub yop_code: Option<String>,
    pub university_name: Option<String>,
    pub faculty: Option<String>,
    pub class_name: Option<String>,
    pub education_duration: Option<String>,
    pub city: Option<String>,
    pub university_style: Option<String>,
    pub scholarship_rate: Option<String>,
    pub education_style: Option<String>,
    pub student_quota_2024: Option<String>,
    pub student_quota_2023: Option<String>,
    pub student_status_2024: Option<String>,
    pub student_status_2023: Option<String>,
    pub base_score_2024: Option<String>,
    pub base_score_2023: Option<String>,
    pub tbs_2024: Option<String>,
    pub tbs_2023: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ayt {
    pub yop_code: Option<String>,
    pub university_name: Option<String>,
    pub faculty: Option<String>,
    pub class_name: Option<String>,
    pub education_style: Option<String>,
    pub education_duration: Option<String>,
    pub city: Option<String>,
    pub university_style: Option<String>,
    pub scholarship_rate: Option<String>,
    pub student_quota_2024: Option<String>,
    pub student_quota_2023: Option<String>,
    pub student_quota_2022: Option<String>,
    pub student_quota_2021: Option<String>,
    pub fullness_status: Option<String>,
    pub enrolled_2024: Option<String>,
    pub enrolled_2023: Option<String>,
    pub enrolled_2022: Option<String>,
    pub enrolled_2021: Option<String>,
    pub tbs_2024: Option<String>,
    pub tbs_2023: Option<String>,
    pub tbs_2022: Option<String>,
    pub tbs_2021: Option<String>,
    pub base_score_2024: Option<String>,
    pub base_score_2023: Option<String>,
    pub base_score_2022: Option<String>,
    pub base_score_2021: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateRankResponse {
    pub estimate_rank: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuResponse {
    pub rank: Option<u32>,
    pub username: String,
    pub percentage: Option<f32>,
}
