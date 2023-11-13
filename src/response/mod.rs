mod data;
pub use data::ResponseData;
mod error;
pub use error::{FromError, ResponseError};
mod file;
pub use file::BodyFile;

pub type Response<T, E> = Result<ResponseData<T>, ResponseError<E>>;