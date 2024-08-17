# WhatIfUni Backend

Backend of the WhatIfUni website.

Needs a `data.db` database with five tables;

## `tytData` with schema:
    ```sql
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
    	"base_score_2024"	REAL,
    	"base_score_2023"	REAL,
    	"tbs_2024"	INTEGER,
    	"tbs_2023"	INTEGER
        );
    ```

## `sayData`, `sözData`, `eaData` and `dilData` with schema:
    ```sql
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
    	"enrolled_2024"	INTEGER,
    	"enrolled_2023"	INTEGER,
    	"enrolled_2022"	INTEGER,
    	"enrolled_2021"	INTEGER,
    	"tbs_2024"	INTEGER,
    	"tbs_2023"	INTEGER,
    	"tbs_2022"	INTEGER,
    	"tbs_2021"	INTEGER,
    	"base_score_2024"	REAL,
    	"base_score_2023"	REAL,
    	"base_score_2022"	REAL,
    	"base_score_2021"	REAL
        );
    ```
