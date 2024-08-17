# calissaydim-backend

Backend of the calissaydim website.

Needs a `data.db` database with five tables;

## `tytData` with schema:
        CREATE TABLE "tytData" (
    	"yop_code"	TEXT,
    	"university_name"	TEXT,
    	"faculty"	TEXT,
    	"class_name"	TEXT,
    	"education_duration"	TEXT,
    	"city"	TEXT,
    	"university_style"	TEXT,
    	"scholarship_rate"	TEXT,
    	"education_style"	TEXT,
    	"student_quota_2024"	TEXT,
    	"student_quota_2023"	TEXT,
    	"student_status_2024"	TEXT,
    	"student_status_2023"	TEXT,
    	"base_score_2024"	TEXT,
    	"base_score_2023"	TEXT,
    	"tbs_2024"	TEXT,
    	"tbs_2023"	TEXT
        );
    

## `sayData`, `sözData`, `eaData` and `dilData` with schema:
        CREATE TABLE "name" (
    	"yop_code"	TEXT,
    	"university_name"	TEXT,
    	"faculty"	TEXT,
    	"class_name"	TEXT,
    	"education_duration"	TEXT,
    	"city"	TEXT,
    	"university_style"	TEXT,
    	"scholarship_rate"	TEXT,
    	"education_style"	TEXT,
    	"student_quota_2024"	TEXT,
    	"student_quota_2023"	TEXT,
    	"student_quota_2022"	TEXT,
    	"student_quota_2021"	TEXT,
    	"fullness_status"	TEXT,
    	"enrolled_2024"	TEXT,
    	"enrolled_2023"	TEXT,
    	"enrolled_2022"	TEXT,
    	"enrolled_2021"	TEXT,
    	"tbs_2024"	TEXT,
    	"tbs_2023"	TEXT,
    	"tbs_2022"	TEXT,
    	"tbs_2021"	TEXT,
    	"base_score_2024"	TEXT,
    	"base_score_2023"	TEXT,
    	"base_score_2022"	TEXT,
    	"base_score_2021"	TEXT
        );
