// @generated automatically by Diesel CLI.

diesel::table! {
    absences (id) {
        id -> Int4,
        employee_code -> Varchar,
        attendance_date -> Nullable<Timestamptz>,
        check_in_time -> Nullable<Timestamptz>,
        check_in_geolocation -> Nullable<Varchar>,
        check_in_mac_address -> Nullable<Varchar>,
        check_in_image -> Nullable<Varchar>,
        check_out_time -> Nullable<Timestamptz>,
        check_out_geolocation -> Nullable<Varchar>,
        check_out_mac_address -> Nullable<Varchar>,
        check_out_image -> Nullable<Varchar>,
        remark -> Nullable<Varchar>,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(absences, employees, users,);
