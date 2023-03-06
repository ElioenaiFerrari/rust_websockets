use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::organizations;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = organizations)]
pub struct Organization {
    pub id: Option<i32>,
    pub description: Option<String>,
    pub infra_location: String,
    pub is_active: bool,
    pub is_emailing_enabled: bool,
    pub jira_id: Option<i32>,
    pub name: String,
    pub parent_id: Option<i32>,
}
