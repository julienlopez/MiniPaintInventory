#[derive(Debug)]
pub enum Error {
    Str(String),
    DatabaseError(diesel::result::Error),
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::DatabaseError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Str(s) => write!(f, "{s}"),
            Error::DatabaseError(e) => write!(f, "Database error: {e:?}"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
