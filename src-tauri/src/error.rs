// use serde::{Serialize, Serializer};

// #[derive(Debug)]
// pub struct CommandError(    pub anyhow::Error);

// impl std::error::Error for CommandError {}
// impl std::fmt::Display for CommandError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         #[cfg(any(debug_assertions, feature = "show_errs_in_release"))]
//         {
//             write!(f, "{:#}", self.0)
//         }

//         #[cfg(not(any(debug_assertions, feature = "show_errs_in_release")))]
//         {
//             write!(f, "{}", self.0)
//         }
//     }
// }

// impl Serialize for CommandError {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(&format!("{:#}", self.0))
//     }
// }

// impl From<anyhow::Error> for CommandError {
//     fn from(error: anyhow::Error) -> Self {
//         Self(error)
//     }
// }

// pub type Result<T> = std::result::Result<T, CommandError>;
