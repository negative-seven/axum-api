use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use crate::database::User;

/// Configurable manager for JSON web tokens for API access.
#[allow(clippy::module_name_repetitions)]
pub struct TokenManager {
    lifetime: Duration,
    lifetime_leeway: Duration,
    encoding_algorithm: Algorithm,
    secret: Arc<String>,
}

impl TokenManager {
    /// Creates a new token manager.
    #[must_use]
    pub fn new(
        lifetime: Duration,
        lifetime_leeway: Duration,
        encoding_algorithm: Algorithm,
        secret: String,
    ) -> Self {
        Self {
            lifetime,
            lifetime_leeway,
            encoding_algorithm,
            secret: Arc::new(secret),
        }
    }

    #[must_use]
    pub fn lifetime(&self) -> Duration {
        self.lifetime
    }

    #[must_use]
    pub fn lifetime_mut(&mut self) -> &mut Duration {
        &mut self.lifetime
    }

    #[must_use]
    pub fn lifetime_leeway(&self) -> Duration {
        self.lifetime_leeway
    }

    #[must_use]
    pub fn lifetime_leeway_mut(&mut self) -> &mut Duration {
        &mut self.lifetime_leeway
    }

    /// Creates a new token according to the `TokenManager` configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if token encoding fails.
    pub fn new_token(&self, user: &User) -> Result<String, Error> {
        let payload = TokenPayload::new(user, self.lifetime);
        payload.encode(self.encoding_algorithm, &self.secret)
    }

    /// Decodes a token into a payload according to the `TokenManager`
    /// configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if decoding fails or if the token is invalid.
    pub fn decode_and_validate_token(&self, token: String) -> Result<TokenPayload, Error> {
        TokenPayload::decode(
            token,
            self.encoding_algorithm,
            &self.secret,
            self.lifetime_leeway,
        )
    }
}

/// The payload of a JSON web token for API access
#[derive(Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub struct TokenPayload {
    pub exp: u64,
    pub user_email: String,
}

impl TokenPayload {
    /// Creates a payload object.
    #[must_use]
    fn new(user: &User, lifetime: Duration) -> Self {
        Self {
            exp: (SystemTime::now() + lifetime)
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("token expiry time predates unix epoch, somehow")
                .as_secs(),
            user_email: user.email.clone(),
        }
    }

    /// Encodes the payload into a JSON web token.
    ///
    /// # Errors
    ///
    /// Returns an error if encoding fails.
    fn encode(&self, encoding_algorithm: Algorithm, secret: &str) -> Result<String, Error> {
        encode(
            &Header::new(encoding_algorithm),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    /// Decodes a payload from a JSON web token.
    ///
    /// # Errors
    ///
    /// Returns an error if decoding fails.
    fn decode(
        token: impl AsRef<str>,
        decoding_algorithm: Algorithm,
        secret: &str,
        lifetime_leeway: Duration,
    ) -> Result<Self, Error> {
        let mut validation = Validation::new(decoding_algorithm);
        validation.leeway = lifetime_leeway.as_secs();

        Ok(decode::<TokenPayload>(
            token.as_ref(),
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )?
        .claims)
    }
}
