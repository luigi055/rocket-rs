mod create;
mod delete;
mod read;
mod read_one;
mod update;

pub use create::create_rustacean;
pub use delete::delete_rustaceans;
pub use read::get_rustaceans;
pub use read_one::view_rustacean;
pub use update::update_rustacean;
