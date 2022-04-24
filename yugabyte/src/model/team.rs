use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::team;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone, GraphQLObject)]
#[table_name = "team"]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NewTeam {
    pub name: String,
    pub description: String,
}
