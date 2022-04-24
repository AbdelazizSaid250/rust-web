use diesel::{Insertable, Queryable};
use juniper::GraphQLInputObject;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::user;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Validate, Clone, GraphQLObject)]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    #[validate(email(code = "email-format-error"))]
    pub email: String,
    pub name: String,
}

#[derive(Default, Debug, Serialize, Deserialize, GraphQLInputObject, Validate)]
pub struct NewUser {
    #[validate(email(code = "email-format-error"))]
    pub email: String,
    pub name: String,
    pub password: String,
}