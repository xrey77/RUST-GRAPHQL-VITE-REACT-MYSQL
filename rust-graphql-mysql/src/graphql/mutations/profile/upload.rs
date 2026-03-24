use async_graphql::*;
use tokio::fs;
use std::path::Path;
use std::ffi::OsStr;
// use tokio::io::AsyncWriteExt;
// use tokio::io::AsyncWriteExt as _;

use async_graphql::{Object, Context, Result, InputObject, SimpleObject};
use sqlx::MySqlPool;

#[derive(InputObject)]
pub struct UploadInput {
    pub id: i64,
    pub file: Upload
}

#[derive(SimpleObject)]
pub struct UploadResponse {
    pub userpic: String,
    pub message: String,
}

#[derive(Default)]
pub struct UploadPicture; 

#[Object]
impl UploadPicture {
    async fn upload_picture(
        &self, 
        ctx: &Context<'_>, 

        input: UploadInput
    ) -> Result<UploadResponse> {
        let pool = ctx.data::<MySqlPool>()?;

        let upload = input.file.value(ctx)?;

        let extension = Path::new(&upload.filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        let new_filename = format!("00{}{}", input.id, extension);        
        let file_path = format!("assets/users/{}", new_filename);


        let existing = sqlx::query!(
            "SELECT email, username FROM users WHERE id = ?",
            input.id
        )
        .fetch_optional(pool)
        .await?;

        if existing.is_none () {
            return Err(async_graphql::Error::new("User not found."));
        }

        fs::create_dir_all("assets/users").await?;
        let mut dest_file = fs::File::create(&file_path).await?;

        // let mut content = upload.content;
        let mut content = tokio::fs::File::from_std(upload.content);
        tokio::io::copy(&mut content, &mut dest_file).await?;

        sqlx::query!(
            "UPDATE users SET userpic=? WHERE id=?",
            new_filename,
            input.id
        )
        .execute(pool)
        .await?;

        Ok(UploadResponse {
            userpic: new_filename,
            message: "You have changed you profile picture successfully.".to_string(),
        })
    }
}
