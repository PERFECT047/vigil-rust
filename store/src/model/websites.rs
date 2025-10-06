use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub time_added: chrono::NaiveDateTime,
    user_id: String,
}

impl Store {
    pub fn create_website(
        &mut self,
        user_id: String,
        url: String,
    ) -> Result<Website, diesel::result::Error> {
        let website = Website {
            id: Uuid::new_v4().to_string(),
            url,
            time_added: chrono::Utc::now().naive_utc(),
            user_id: user_id,
        };

        let website_result = diesel::insert_into(crate::schema::website::table)
            .values(&website)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;

        Ok(website_result)
    }

    pub fn get_website(&mut self, input_website_id: String) -> Result<Website, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let website_result = website
            .filter(id.eq(input_website_id))
            .select(Website::as_returning())
            .first(&mut self.conn)?;

        Ok(website_result)
    }
}
