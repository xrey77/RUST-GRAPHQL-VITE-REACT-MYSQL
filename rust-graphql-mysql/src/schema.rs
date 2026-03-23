use async_graphql::{MergedObject, Schema, EmptySubscription};
use crate::graphql::queries::userid_query::UserByIdQuery;
use crate::graphql::queries::users_query::UsersQuery;
use crate::graphql::mutations::creates::newuser::FormRegistration;
use crate::graphql::mutations::login::signin::UserSignin;
use crate::graphql::mutations::profile::update::ProfileMutation;
use crate::graphql::mutations::password::change::ChangePassword;
use crate::graphql::mutations::activate::mfa_mutation::ActivateMfa;
use crate::graphql::mutations::verify::otp_verification::OtpVerification;
use sqlx::MySqlPool;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserByIdQuery, UsersQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(FormRegistration, UserSignin, ProfileMutation, ChangePassword, ActivateMfa, OtpVerification);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(pool: MySqlPool) -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(pool)
        .finish()
}
