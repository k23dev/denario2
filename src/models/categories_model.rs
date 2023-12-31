use diesel::prelude::*;
use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::core::db_manager_core::postgres_establish_connection;
use crate::schema::categories::dsl::*;

#[derive(Insertable,Queryable, Selectable,Serialize,AsChangeset,Deserialize,Debug,Clone)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category{
    pub id:i32,
    pub name:String,
    pub created_at:NaiveDateTime,
    pub updated_at:NaiveDateTime,
    pub is_deleted:bool,
}

impl Category{

    pub fn new(category_name:String) -> Self{
        Self {
            id          :   0,
            name        :   category_name,
            created_at  :   Utc::now().naive_utc(),
            updated_at  :   Utc::now().naive_utc(),
            is_deleted  :   false
        }
    }

    pub fn find_all() -> Vec<Category> {
        let mut conn=postgres_establish_connection().clone();
        
        let result= categories.load::<Category>(&mut conn).expect("Fail to read table");

        return results;
    }

    pub fn find_one(conn: &mut PgConnection, id: i32) -> Category {
        use crate::schema::categories::dsl::*;
        let result = categories
            .filter(id.eq(id))
            .filter(is_deleted.eq(false))
            .first::<Category>(conn)
            .expect("Error loading categories");
        return result;
    }

    pub fn create(conn: &mut PgConnection, category: Category) -> Category {
        use crate::schema::categories::dsl::*;
        let result = diesel::insert_into(categories)
            .values(category)
            .get_result(conn)
            .expect("Error saving new category");
        return result;
    }

    pub fn update(conn: &mut PgConnection, id: i32, category: Category) -> Category {
        use crate::schema::categories::dsl::*;
        let result = diesel::update(categories)
            .filter(id.eq(id))
            .filter(is_deleted.eq(false))
            .set(category)
            .get_result(conn)
            .expect("Error saving new category");
        return result;
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> Category {
        use crate::schema::categories::dsl::*;
        let result = diesel::update(categories)
            .filter(id.eq(id))
            .filter(is_deleted.eq(false))
            .set(is_deleted.eq(true))
            .get_result(conn)
            .expect("Error saving new category");
        return result;
    }

}