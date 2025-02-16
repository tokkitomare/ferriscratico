use poise::serenity_prelude::Error as SerenityError;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ErrorOrigin {
    User,
    Internal,
    External,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    AlreadyExists,
    BlockedAction,
    CanceledAction,
    CorruptedFile,
    EmptyValue,
    GhostEditing,
    InvalidValue,
    LogicError,
    MissingParam,
    MissingVariable,
    NotEnoughArgs,
    NotFound,
    NotSent,
    NullishValue,
    SyntaxError,
    Other,
    TimeOut,
    TypeError,
}

#[derive(Debug, Clone)]
pub struct RsError {
    pub message: String,
    #[allow(warnings)]
    pub origin: ErrorOrigin,
    pub kind: ErrorKind,
}

pub trait Error: std::error::Error + Display {
    fn kind(&self) -> ErrorKind;
    fn new(message: String, kind: ErrorKind, origin: ErrorOrigin) -> BotErr
    where
        Self: Sized;
}

impl Display for RsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RsError {}

impl Error for RsError {
    fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }
    fn new(message: String, kind: ErrorKind, origin: ErrorOrigin) -> BotErr {
        Box::new(RsError {
            message,
            origin,
            kind,
        })
    }
}

impl From<SerenityError> for BotErr {
    fn from(value: SerenityError) -> Self {
        Box::new(RsError {
            message: value.to_string(),
            origin: ErrorOrigin::Unknown,
            kind: ErrorKind::Other,
        })
    }
}

pub type BotErr = Box<dyn Error + Send + Sync>;