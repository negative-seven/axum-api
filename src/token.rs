use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

const LIFETIME: Duration = Duration::from_secs(30 * 60);
const LIFETIME_LEEWAY: Duration = Duration::from_secs(60);
const ENCODING_ALGORITHM: Algorithm = Algorithm::HS256;
const SECRET: &str = "secret"; // TODO: use proper secret!

pub fn create() -> Result<String, Error> {
    encode(
        &Header::new(ENCODING_ALGORITHM),
        &Claims::new(),
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
}

pub fn is_valid(token: impl AsRef<str>) -> bool {
    let mut validation = Validation::new(ENCODING_ALGORITHM);
    validation.leeway = LIFETIME_LEEWAY.as_secs();

    decode::<Claims>(
        token.as_ref(),
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )
    .is_ok()
}

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: u64,
}

impl Claims {
    pub fn new() -> Self {
        Self {
            exp: (SystemTime::now() + LIFETIME)
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("current time predates unix epoch, somehow")
                .as_secs(),
        }
    }
}
