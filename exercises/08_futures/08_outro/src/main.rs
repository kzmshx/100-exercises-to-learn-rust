// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use db::create_db;

pub mod api;
pub mod db;
pub mod ticket;

#[tokio::main]
async fn main() {
    let db = create_db();

    warp::serve(api::filters::all(db))
        .run(([127, 0, 0, 1], 3000))
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::models::{CreateTicketInput, ErrorResponse, Ticket, UpdateTicketInput};
    use db::Db;
    use std::sync::Arc;
    use ticket::{Status, TicketDraft};
    use ticket_fields::{TicketDescription, TicketTitle};
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn hello_ok() {
        // Arrange
        let (db, _) = setup_db().await;
        // Act
        let req = warp::test::request().method("GET").path("/hello");
        let res = req.reply(&api::filters::all(db)).await;
        // Assert
        assert_eq!(200, res.status());
        assert_eq!("Hello, world!".as_bytes(), res.body());
    }

    #[tokio::test]
    async fn tickets_list_ok() {
        // Arrange
        let (db, _) = setup_db().await;
        // Act
        let req = warp::test::request().method("GET").path("/tickets");
        let res = req.reply(&api::filters::all(db)).await;
        // Assert
        assert_eq!(200, res.status());
    }

    #[tokio::test]
    async fn tickets_create_ok() {
        // Arrange
        let (db, _) = setup_db().await;
        // Act
        let req = warp::test::request()
            .method("POST")
            .path("/tickets")
            .json(&CreateTicketInput {
                title: "New task title".into(),
                description: "New task description".into(),
            });
        let res = req.reply(&api::filters::all(db.clone())).await;
        // Assert
        let ticket = Ticket::try_from(res.body().as_ref()).unwrap();
        assert_eq!(200, res.status());
        assert_eq!("New task title", ticket.title);
        assert_eq!("New task description", ticket.description);
    }

    #[tokio::test]
    async fn tickets_create_bad_request_empty_title() {
        // Arrange
        let (db, _) = setup_db().await;
        // Act
        let req = warp::test::request()
            .method("POST")
            .path("/tickets")
            .json(&CreateTicketInput {
                title: "".into(),
                description: "New task description".into(),
            });
        let res = req.reply(&api::filters::all(db.clone())).await;
        // Assert
        let error_response = ErrorResponse::try_from(res.body().as_ref()).unwrap();
        assert_eq!(400, res.status());
        assert_eq!("The title cannot be empty", error_response.message);
    }

    #[tokio::test]
    async fn tickets_get_ok() {
        // Arrange
        let (db, tickets) = setup_db().await;
        let ticket = tickets.first().unwrap().read().await.clone();
        // Act
        let req = warp::test::request()
            .method("GET")
            .path(&format!("/tickets/{}", ticket.id.value()));
        let res = req.reply(&api::filters::all(db.clone())).await;
        // Assert
        let ticket = Ticket::try_from(res.body().as_ref()).unwrap();
        assert_eq!(200, res.status());
        assert_eq!("Test title", ticket.title);
        assert_eq!("Test description", ticket.description);
    }

    #[tokio::test]
    async fn tickets_get_not_found() {
        // Arrange
        let (db, _) = setup_db().await;
        // Act
        let req = warp::test::request().method("GET").path("/tickets/12345");
        let res = req.reply(&api::filters::all(db.clone())).await;
        // Assert
        assert_eq!(404, res.status());
    }

    #[tokio::test]
    async fn tickets_get_not_found_invalid_id() {
        // Arrange
        let (db, _) = setup_db().await;
        // Act
        let req = warp::test::request().method("GET").path("/tickets/invalid");
        let res = req.reply(&api::filters::all(db.clone())).await;
        // Assert
        assert_eq!(404, res.status());
    }

    #[tokio::test]
    async fn tickets_update_ok() {
        // Arrange
        let (db, tickets) = setup_db().await;
        let ticket = tickets.first().unwrap().read().await.clone();
        // Act
        let req = warp::test::request()
            .method("PUT")
            .path(&format!("/tickets/{}", ticket.id.value()))
            .json(&UpdateTicketInput {
                id: ticket.id.value(),
                title: Some("Updated task title".into()),
                description: Some("Updated task description".into()),
                status: Some(format!("{}", Status::InProgress)),
            });
        let res = req.reply(&api::filters::all(db.clone())).await;
        let ticket = Ticket::try_from(res.body().as_ref()).unwrap();
        // Assert
        assert_eq!(200, res.status());
        assert_eq!("Updated task title", ticket.title);
        assert_eq!("Updated task description", ticket.description);
        assert_eq!(format!("{}", Status::InProgress), ticket.status)
    }

    #[tokio::test]
    async fn tickets_update_bad_request_invalid_status() {
        // Arrange
        let (db, tickets) = setup_db().await;
        let ticket = tickets.first().unwrap().read().await.clone();
        // Act
        let req = warp::test::request()
            .method("PUT")
            .path(&format!("/tickets/{}", ticket.id.value()))
            .json(&UpdateTicketInput {
                id: ticket.id.value(),
                title: None,
                description: None,
                status: Some("Invalid-Status".to_string()),
            });
        let res = req.reply(&api::filters::all(db.clone())).await;
        // Assert
        let error_response = ErrorResponse::try_from(res.body().as_ref()).unwrap();
        assert_eq!(400, res.status());
        assert_eq!("Invalid status", error_response.message);
    }

    async fn setup_db() -> (Db, Vec<Arc<RwLock<ticket::Ticket>>>) {
        let db = create_db();

        let mut db_write = db.write().await;
        let mut tickets: Vec<Arc<RwLock<ticket::Ticket>>> = Vec::new();
        let ticket = db_write.add_ticket(TicketDraft {
            title: TicketTitle::try_from("Test title").unwrap(),
            description: TicketDescription::try_from("Test description").unwrap(),
        });
        tickets.push(ticket);
        let ticket = db_write.add_ticket(TicketDraft {
            title: TicketTitle::try_from("Test title 2").unwrap(),
            description: TicketDescription::try_from("Test description 2").unwrap(),
        });
        tickets.push(ticket);

        (db.clone(), tickets)
    }
}
