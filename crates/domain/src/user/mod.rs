mod entity;
mod inputs;

pub use entity::User;
pub use inputs::{
    CreateUserInput, DeleteUserInput, GetUserByEmailInput, GetUserByUsernameInput, GetUserInput,
    ListUsersInput, UpdateUserInput,
};
