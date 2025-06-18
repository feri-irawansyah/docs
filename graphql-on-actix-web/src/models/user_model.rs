use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::schema::datetime::MyDateTime;

#[derive(FromRow, Deserialize, Debug, Serialize)]
pub struct UserDB {
    pub user_id: i32,
    pub email: String,
    pub handphone: String,
    pub register_date: chrono::DateTime<chrono::Utc>,
    pub web_cif_id: i32,
    pub disable_login: bool,
    pub activate_code: Option<String>,
    pub activate_time: Option<chrono::DateTime<chrono::Utc>>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub client_category: i32,
    pub last_resend_otp: Option<chrono::DateTime<chrono::Utc>>,
    pub otp_generated_link: Option<String>,
    pub reset_password_key: Option<String>,
    pub reset_password_flag: Option<bool>,
    pub reset_password_date: Option<chrono::DateTime<chrono::Utc>>,
    pub count_resend_activation: Option<i32>,
    pub picture: Option<String>,
    pub google_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub handphone: String,
    pub register_date: MyDateTime,
    pub web_cif_id: i32,
    pub disable_login: bool,
    pub activate_code: Option<String>,
    pub activate_time: Option<MyDateTime>,
    pub last_login: Option<MyDateTime>,
    pub client_category: Option<i32>,
    pub last_resend_otp: Option<MyDateTime>,
    pub otp_generated_link: Option<String>,
    pub reset_password_key: Option<String>,
    pub reset_password_flag: Option<bool>,
    pub reset_password_date: Option<MyDateTime>,
    pub count_resend_activation: Option<i32>,
    pub picture: Option<String>,
    pub google_id: Option<String>,
}

impl From<UserDB> for User {
    fn from(user: UserDB) -> Self {
        User {
            user_id: user.user_id,
            email: user.email,
            handphone: user.handphone,
            register_date: MyDateTime(user.register_date),
            web_cif_id: user.web_cif_id,
            disable_login: user.disable_login,
            activate_code: Some(user.activate_code.unwrap_or_default()),
            activate_time: Some(MyDateTime(user.activate_time.unwrap_or_default())),
            last_login: Some(MyDateTime(user.last_login.unwrap_or_default())),
            client_category: Some(user.client_category),
            last_resend_otp: Some(MyDateTime(user.last_resend_otp.unwrap_or_default())),
            otp_generated_link: Some(user.otp_generated_link.unwrap_or_default()),
            reset_password_key: Some(user.reset_password_key.unwrap_or_default()),
            reset_password_flag: Some(user.reset_password_flag.unwrap_or_default()),
            reset_password_date: Some(MyDateTime(user.reset_password_date.unwrap_or_default())),
            count_resend_activation: Some(user.count_resend_activation.unwrap_or_default()),
            picture: user.picture,
            google_id: user.google_id,
        }
    }
}
