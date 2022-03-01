use diesel::{Insertable, Queryable};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::user;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Validate, Clone, Apiv2Schema)]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    #[validate(email(code = "email-format-error"))]
    pub email: String,
    pub name: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct NewUser {
    #[validate(email(code = "email-format-error"))]
    pub email: String,
    pub name: String,
}