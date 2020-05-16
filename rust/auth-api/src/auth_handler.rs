use actix_identity::Identity;
use actix_web::{dev::Payload, error::BlockingError, web, Error, FromRequest, HttpRequest, HttpResponse};
use argonautica::Verifier;
use diesel::prelude::*;
use diesel::PgConnection;
use futures::Future;

use crate::errors::ServiceError;
use crate::models::{Pool, SlimUser, User};

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Result<LoggedUser, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Some(identity) = Identity::from_request(req, payload)?.identity() {
            let user: LoggedUser = serde_json::from_str(&identity)?;
            return Ok(user);
        }

        Err(ServiceError::Unauthorized.into())
    }
}

pub fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || query(auth_data.into_inner(), pool))
        .then(move |res: Result<SlimUser, BlockingError<ServiceError>>| {
            match res {
                Ok(user) => {
                    let user_string = serde_json::to_string(&user).unwrap();
                    id.remember(user_string);
                    Ok(HttpResponse::Ok().finish())
                },
                Err(err) => match err {
                    BlockingError::Error(service_error) => Err(service_error),
                    BlockingError::Canceled => Err(ServiceError::InternalServerError),
                },
            }
        }
    )
}

fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    let conn: &PgConnection = &pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.encrypted_password, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }

    Err(ServiceError::Unauthorized)
}

pub fn verify(encrypted_password: &str, password: &str) -> Result<bool, ServiceError> {
    Verifier::default()
        .with_hash(encrypted_password)
        .with_password(password)
        .with_secret_key("1234".repeat(8))
        .verify()
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}
