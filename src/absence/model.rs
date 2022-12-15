use crate::error_handler::CustomError;
use crate::schema::absences;
use crate::{db, TokenClaims};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "absences"]
pub struct Absence {
    pub employee_code: String,
    pub attendance_date: Option<DateTime<Utc>>,
    pub check_in_time: Option<DateTime<Utc>>,
    pub check_in_geolocation: Option<String>,
    pub check_in_mac_address: Option<String>,
    pub check_in_image: Option<String>,
    pub check_out_time: Option<DateTime<Utc>>,
    pub check_out_geolocation: Option<String>,
    pub check_out_mac_address: Option<String>,
    pub check_out_image: Option<String>,
    pub remark: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "absences"]
pub struct Absences {
    pub id: i32,
    pub employee_code: String,
    pub attendance_date: Option<DateTime<Utc>>,
    pub check_in_time: Option<DateTime<Utc>>,
    pub check_in_geolocation: Option<String>,
    pub check_in_mac_address: Option<String>,
    pub check_in_image: Option<String>,
    pub check_out_time: Option<DateTime<Utc>>,
    pub check_out_geolocation: Option<String>,
    pub check_out_mac_address: Option<String>,
    pub check_out_image: Option<String>,
    pub remark: Option<String>,
}

impl Absences {
    pub fn find_all(req_user: String) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let absences = absences::table
            .filter(absences::employee_code.eq(req_user))
            .load::<Absences>(&conn)?;
        Ok(absences)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let absence = absences::table.filter(absences::id.eq(id)).first(&conn)?;
        Ok(absence)
    }

    pub fn create(absence: Absence, req_user: String) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let mut absence = Absence::from(absence);
        absence.employee_code = req_user;
        let absence = diesel::insert_into(absences::table)
            .values(absence)
            .get_result(&conn)?;
        Ok(absence)
    }

    pub fn update(id: i32, absence: Absence) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let absence = diesel::update(absences::table)
            .filter(absences::id.eq(id))
            .set(absence)
            .get_result(&conn)?;
        Ok(absence)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(absences::table.filter(absences::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Absence {
    fn from(absence: Absence) -> Absence {
        Absence {
            employee_code: absence.employee_code,
            attendance_date: Some(Utc::now()),
            check_in_time: Some(Utc::now()),
            check_in_geolocation: absence.check_in_geolocation,
            check_in_mac_address: absence.check_in_mac_address,
            check_in_image: absence.check_in_image,
            check_out_time: absence.check_out_time,
            check_out_geolocation: absence.check_out_geolocation,
            check_out_mac_address: absence.check_out_mac_address,
            check_out_image: absence.check_out_image,
            remark: absence.remark,
        }
    }
}
