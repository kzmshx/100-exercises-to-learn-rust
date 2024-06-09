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

use api::db::create_db;

pub mod api;
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
    use api::{
        db::Db,
        models::{CreateTicketInput, Ticket, UpdateTicketInput},
    };
    use ticket::{Status, TicketDraft};
    use ticket_fields::{TicketDescription, TicketTitle};

    #[tokio::test]
    async fn hello_ok() {
        // Arrange
        let db = setup_db().await;
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
        let db = setup_db().await;
        // Act
        let req = warp::test::request().method("GET").path("/tickets");
        let res = req.reply(&api::filters::all(db)).await;
        // Assert
        assert_eq!(200, res.status());
    }

    #[tokio::test]
    async fn tickets_create_ok() {
        // Arrange
        let db = setup_db().await;
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
    async fn tickets_get_ok() {
        // Arrange
        let db = setup_db().await;
        let req = warp::test::request().method("GET").path("/tickets");
        let res = req.reply(&api::filters::all(db.clone())).await;
        let body = res.body().to_vec();
        let body = body.as_slice();
        let tickets = serde_json::from_slice::<Vec<Ticket>>(body).unwrap();
        let ticket = tickets.first().unwrap();

        // Act
        let req = warp::test::request()
            .method("GET")
            .path(&format!("/tickets/{}", ticket.id));
        let res = req.reply(&api::filters::all(db.clone())).await;

        // Assert
        let ticket = Ticket::try_from(res.body().as_ref()).unwrap();
        assert_eq!(200, res.status());
        assert_eq!("Test title", ticket.title);
        assert_eq!("Test description", ticket.description);
    }

    #[tokio::test]
    async fn tickets_update_ok() {
        // Arrange
        let db = setup_db().await;
        let req = warp::test::request()
            .method("POST")
            .path("/tickets")
            .json(&CreateTicketInput {
                title: "New task title".into(),
                description: "New task description".into(),
            });
        let res = req.reply(&api::filters::all(db.clone())).await;
        let ticket = Ticket::try_from(res.body().as_ref()).unwrap();
        assert_eq!(200, res.status());

        // Act
        let req = warp::test::request()
            .method("PUT")
            .path(&format!("/tickets/{}", ticket.id))
            .json(&UpdateTicketInput {
                id: ticket.id,
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

    async fn setup_db() -> Db {
        let db = create_db();

        let mut db_write = db.write().await;
        db_write.add_ticket(TicketDraft {
            title: TicketTitle::try_from("Test title").unwrap(),
            description: TicketDescription::try_from("Test description").unwrap(),
        });
        db_write.add_ticket(TicketDraft {
            title: TicketTitle::try_from("Test title 2").unwrap(),
            description: TicketDescription::try_from("Test description 2").unwrap(),
        });

        db.clone()
    }
}
