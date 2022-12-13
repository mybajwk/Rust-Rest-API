-- Your SQL goes here
CREATE TABLE "absences"
(
    id SERIAL PRIMARY KEY,
    employee_code VARCHAR NOT NULL,
    attendance_date TIMESTAMPTZ default DATE(CURRENT_TIMESTAMP),
    check_in_time TIMESTAMPTZ,
    check_in_geolocation VARCHAR,
    check_in_mac_address VARCHAR,
    check_in_image VARCHAR,
    check_out_time TIMESTAMPTZ,
    check_out_geoLocation VARCHAR,
    check_out_mac_address VARCHAR,
    check_out_image VARCHAR,
    remark VARCHAR
)