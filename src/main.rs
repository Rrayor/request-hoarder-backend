use request_hoarder_backend::establish_connection;
use sea_orm::prelude::DateTime;
use chrono::{NaiveDate, NaiveTime};
use sea_orm::DatabaseConnection;
use tokio;
use sea_orm::{Set, ActiveModelTrait};
use entity::project;
use entity::group;
use entity::endpoint;
use entity::request;
use entity::request_header;
use entity::response_header;
use entity::response;
use entity::session;
use entity::sessions_to_endpoints;
use sea_orm::{EntityTrait, QueryOrder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = establish_connection().await?;
    insert_entities(&db).await?;
    request_entities(&db).await?;
    Ok(())
}

async fn insert_entities(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let project = project::ActiveModel {
        name: Set("Test".to_owned()),
        description: Set(Some("Test project".to_owned())),
        ..Default::default()
    };

    let p: project::Model = project.insert(db).await?;

    println!("project inserted");

    let group = group::ActiveModel {
        name: Set("Test".to_owned()),
        description: Set(Some("Test group".to_owned())),
        project_id: Set(p.id),
        ..Default::default()
    };

    let g: group::Model = group.insert(db).await?;

    println!("group inserted");


    let endpoint = endpoint::ActiveModel {
        name: Set("Test".to_owned()),
        url: Set("http://localhost:8080".to_owned()),
        method: Set("GET".to_owned()),
        group_id: Set(g.id),
        ..Default::default()
    };

    let e: endpoint::Model = endpoint.insert(db).await?;

    println!("endpoint inserted");

    let request = request::ActiveModel {
        name: Set("Test".to_owned()),
        sent_status: Set(200),
        sent_at: Set(DateTime::new(NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(), NaiveTime::from_hms_opt(10, 10, 10).unwrap())),
        endpoint_id: Set(e.id),
        body: Set("{request: \"Some request\"}".to_owned()),
        ..Default::default()
    };
    
    let r: request::Model = request.insert(db).await?;

    println!("request inserted");

    let request_header = request_header::ActiveModel {
        key: Set("Content-Type".to_owned()),
        value: Set("application/json".to_owned()),
        request_id: Set(r.id),
        ..Default::default()
    };

    let request_header: request_header::Model = request_header.insert(db).await?;

    println!("request header inserted");
    
    let response = response::ActiveModel {
        request_id: Set(r.id),
        status: Set(200),
        received_at: Set(DateTime::new(NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(), NaiveTime::from_hms_opt(10, 11, 10).unwrap())),
        body: Set("{response: \"Some response\"}".to_owned()),
        ..Default::default()
    };
    
    let s: response::Model = response.insert(db).await?;

    println!("response inserted");
    
    let response_header = response_header::ActiveModel {
        key: Set("Content-Type".to_owned()),
        value: Set("application/json".to_owned()),
        response_id: Set(s.id),
        ..Default::default()
    };

    let rh: response_header::Model = response_header.insert(db).await?;

    println!("response header inserted");

    let session = session::ActiveModel {
        ..Default::default()
    };
    let s: session::Model = session.insert(db).await?;

    println!("session inserted");

    let session_to_endpoint = sessions_to_endpoints::ActiveModel {
        session_id: Set(s.id),
        endpoint_id: Set(e.id),
        ..Default::default()
    };

    let se: sessions_to_endpoints::Model = session_to_endpoint.insert(db).await?;

    println!("session to endpoint inserted");

    println!("All entities inserted");

    println!("{:?}", p);
    println!("{:?}", g);
    println!("{:?}", e);
    println!("{:?}", r);
    println!("{:?}", request_header);
    println!("{:?}", rh);
    println!("{:?}", s);
    println!("{:?}", se);

    Ok(())
}
async fn request_entities(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Fetching all entities ===\n");

    // Fetch projects
    let projects = entity::project::Entity::find()
        .order_by_asc(entity::project::Column::Id)
        .all(db)
        .await?;
    println!("Projects:");
    for p in projects {
        println!("{:?}", p);
    }

    // Fetch groups
    let groups = entity::group::Entity::find()
        .order_by_asc(entity::group::Column::Id)
        .all(db)
        .await?;
    println!("\nGroups:");
    for g in groups {
        println!("{:?}", g);
    }

    // Fetch endpoints
    let endpoints = entity::endpoint::Entity::find()
        .order_by_asc(entity::endpoint::Column::Id)
        .all(db)
        .await?;
    println!("\nEndpoints:");
    for e in endpoints {
        println!("{:?}", e);
    }

    // Fetch requests
    let requests = entity::request::Entity::find()
        .order_by_asc(entity::request::Column::Id)
        .all(db)
        .await?;
    println!("\nRequests:");
    for r in requests {
        println!("{:?}", r);
    }

    // Fetch request headers
    let request_headers = entity::request_header::Entity::find()
        .find_also_related(request::Entity)
        .order_by_asc(entity::request_header::Column::Id)
        .all(db)
        .await?;
    println!("\nRequest Headers:");
    for rh in request_headers {
        println!("{:?}", rh);
        println!("{}", rh.1.unwrap().name)
    }

    // Fetch responses
    let responses = entity::response::Entity::find()
        .order_by_asc(entity::response::Column::Id)
        .all(db)
        .await?;
    println!("\nResponses:");
    for resp in responses {
        println!("{:?}", resp);
    }

    // Fetch response headers
    let response_headers = entity::response_header::Entity::find()
        .order_by_asc(entity::response_header::Column::Id)
        .all(db)
        .await?;
    println!("\nResponse Headers:");
    for rh in response_headers {
        println!("{:?}", rh);
    }

    // Fetch sessions
    let sessions = entity::session::Entity::find()
        .order_by_asc(entity::session::Column::Id)
        .all(db)
        .await?;
    println!("\nSessions:");
    for s in sessions {
        println!("{:?}", s);
    }

    // Fetch sessions to endpoints
    let sessions_endpoints = entity::sessions_to_endpoints::Entity::find()
        .order_by_asc(entity::sessions_to_endpoints::Column::SessionId)
        .all(db)
        .await?;
    println!("\nSessions to Endpoints:");
    for se in sessions_endpoints {
        println!("{:?}", se);
    }

    println!("\n=== All entities fetched ===");

    Ok(())
}