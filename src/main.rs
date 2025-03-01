use request_hoarder_backend::establish_connection;
use tokio;
use sea_orm::{Set, ActiveModelTrait};
use entity::project;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = establish_connection().await?;
    let project = project::ActiveModel {
        name: Set("Test".to_owned()),
        description: Set(Some("Test project".to_owned())),
        ..Default::default()
    };

    let p: project::Model = project.insert(&db).await?;
    println!("{:?}", p);
    Ok(())
}