use diesel::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = crate::schema::tag)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}
