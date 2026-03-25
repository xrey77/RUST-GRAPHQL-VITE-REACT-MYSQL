use crate::graphql::models::user::User;
use crate::graphql::models::claims_struct::Claims;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum AuthenticatedUser {
    User(Claims),    
    Registered(User),
    Anonymous,
}