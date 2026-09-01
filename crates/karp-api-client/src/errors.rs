#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    SerdeError {
        error: format_serde_error::SerdeError,
        status: reqwest::StatusCode,
    },
    Server {
        body: String,
        status: reqwest::StatusCode,
    },
}

impl Error {
    pub fn from_serde_error(
        e: format_serde_error::SerdeError,
        status: reqwest::StatusCode,
    ) -> Self {
        Self::SerdeError { error: e, status }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err)
    }
}

// impl From<serde_json::Error> for Error {
//     fn from(err: serde_json::Error) -> Self {

//     }
// }
