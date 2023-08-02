/// ReciteError enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug)]
pub enum SignupError {
    #[error("The username `{0}` already exists")]
    UsernameExists(String),
    #[error("Invalid username")]
    InvalidUsername,
    #[error("Passwords do not match")]
    PasswordsDoNotMatch,
    #[error("Missing details")]
    MissingDetails,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Internal errors")]
    InternalError,
}

/// ReciteError enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Missing details")]
    MissingDetails,
    #[error("The username `{0}` does not exits")]
    UserDoesNotExist(String),
    #[error("Wrong password")]
    WrongPassword,
}
