use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

// TODO: make these configurable
const LIFETIME: Duration = Duration::from_secs(30 * 60);
const LIFETIME_LEEWAY: Duration = Duration::from_secs(60);
const ENCODING_ALGORITHM: Algorithm = Algorithm::HS256;
const SECRET: &str = "secret"; // TODO: use proper secret!

/// The payload of a JSON web token for API access
#[derive(Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub struct TokenPayload {
    exp: u64,
}

impl TokenPayload {
    /// Creates a payload object.
    #[must_use]
    pub fn new() -> Self {
        Self {
            exp: (SystemTime::now() + LIFETIME)
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("current time predates unix epoch, somehow")
                .as_secs(),
        }
    }

    /// Encodes the payload into a JSON web token.
    ///
    /// # Errors
    ///
    /// Returns an error if encoding fails.
    pub fn encode(&self) -> Result<String, Error> {
        encode(
            &Header::new(ENCODING_ALGORITHM),
            self,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )
    }

    /// Decodes a payload from a JSON web token.
    ///
    /// # Errors
    ///
    /// Returns an error if decoding fails.
    pub fn decode(token: impl AsRef<str>) -> Result<Self, Error> {
        let mut validation = Validation::new(ENCODING_ALGORITHM);
        validation.leeway = LIFETIME_LEEWAY.as_secs();

        Ok(decode::<TokenPayload>(
            token.as_ref(),
            &DecodingKey::from_secret(SECRET.as_ref()),
            &validation,
        )?
        .claims)
    }
}

impl Default for TokenPayload {
    fn default() -> Self {
        Self::new()
    }
}
