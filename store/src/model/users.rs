use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;
use argon2::password_hash::SaltString;

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
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| diesel::result::Error::RollbackTransaction)?
            .to_string();

        let user = User {
            id: Uuid::new_v4().to_string(),
            username,
            password: password_hash,
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

        let parsed_hash = PasswordHash::new(&user_result.password)
            .map_err(|_| diesel::result::Error::NotFound)?;
        let argon2 = Argon2::default();
        argon2
            .verify_password(input_password.as_bytes(), &parsed_hash)
            .map_err(|_| diesel::result::Error::NotFound)?;

        Ok(user_result.id)
    }
}
