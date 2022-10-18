use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::tasks;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct NewTask {
    pub title: String,
}
