use crate::errors::AppError;

pub type ErrorHandler = fn(AppError);
