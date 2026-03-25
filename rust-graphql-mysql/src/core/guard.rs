// use async_graphql::*;

// #[derive(Eq, PartialEq, Copy, Clone)]
// pub enum UserRole {
//     Admin,
//     User,
// }

// struct RoleGuard {
//     role: UserRole,
// }

// impl RoleGuard {
//     fn new(role: UserRole) -> Self {
//         Self { role }
//     }
// }

// impl Guard for RoleGuard {
//     async fn check(&self, ctx: &Context<'_>) -> Result<()> {
//         if ctx.data_opt::<UserRole>() == Some(&self.role) {
//             Ok(())
//         } else {
//             Err("Forbidden: Insufficient permissions".into())
//         }
//     }
// }

// pub struct QueryRoot;

// #[Object]
// impl QueryRoot {
//     // Only accessible if the user has the Admin role
//     #[graphql(guard = "RoleGuard::new(UserRole::Admin)")]
//     async fn sensitive_data(&self) -> &str {
//         "This is top secret admin stuff!"
//     }

//     // Standard query accessible to everyone (no guard)
//     async fn public_data(&self) -> &str {
//         "Hello, world!"
//     }
// }
