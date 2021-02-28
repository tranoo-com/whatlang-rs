mod confidence;
mod detect;
mod filter_list;
mod info;
mod method;
mod options;
mod query;
mod text;

pub use confidence::calculate_confidence;
pub use filter_list::FilterList;
pub use info::Info;
pub use method::Method;
pub use options::Options;
pub use query::{InternalQuery, Query};
pub use text::{LowercaseText, Text};

pub use detect::{detect, detect_lang, detect_with_options};