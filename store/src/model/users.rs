use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    password: String,
}

impl Store {
    pub fn sign_up_user(
        &mut self,
        username: String,
        password: String,
    ) -> Result<String, diesel::result::Error> {
        let user = User {
            id: Uuid::new_v4().to_string(),
            username,
            password,
        };

        diesel::insert_into(crate::schema::user::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;

        Ok(user.id)
    }

    pub fn sign_in_user(
        &mut self,
        input_username: String,
        input_password: String,
    ) -> Result<String, diesel::result::Error> {
        use crate::schema::user::dsl::*;

        let user_result = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        if user_result.password != input_password {
            return Err(diesel::result::Error::NotFound);
        }

        Ok(user_result.id)
    }
}
