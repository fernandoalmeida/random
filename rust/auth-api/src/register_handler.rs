use actix_web::{error::BlockingError, web, HttpResponse};
use argonautica::Hasher;
use diesel::{prelude::*, PgConnection};
use futures::Future;

use crate::errors::ServiceError;
use crate::models::{Invitation, Pool, SlimUser, User};

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

pub fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || {
        query(
            invitation_id.into_inner(),
            user_data.into_inner().password,
            pool,
        )
    }).then(|res| {
        match res {
            Ok(user) => Ok(HttpResponse::Ok().json(&user)),
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            }
        }
    })
}

fn query(
    invitation_id: String,
    password: String,
    pool: web::Data<Pool>,
) -> Result<SlimUser, ServiceError> {
    use crate::schema::invitations::dsl::{id, invitations};
    use crate::schema::users::dsl::users;

    let invitation_id = uuid::Uuid::parse_str(&invitation_id)?;
    let conn:&PgConnection = &pool.get().unwrap();

    invitations
        .filter(id.eq(invitation_id))
        .load::<Invitation>(conn)
        .map_err(|_db_error| ServiceError::BadRequest("invalid invitation".into()))
        .and_then(|mut result| {
            if let Some(invitation) = result.pop() {
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    let encrypted_password: String = encrypt_password(&password)?;
                    let user = User::from_details(invitation.email, encrypted_password);

                    let inserted_user: User = diesel::insert_into(users)
                        .values(&user)
                        .get_result(conn)?;
                    dbg!(&inserted_user);

                    return Ok(inserted_user.into());
                }
            }

            Err(ServiceError::BadRequest("invalid invitation".into()))
        })
}

pub fn encrypt_password(password: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key("1234".repeat(8))
        .hash()
        .map_err(|err| {
            dbg!(err);
            ServiceError::InternalServerError
        })
}
