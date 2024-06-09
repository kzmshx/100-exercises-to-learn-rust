pub mod tickets {
    use crate::{
        api::models::{CreateTicketInput, ErrorResponse, Ticket, UpdateTicketInput},
        db::Db,
        ticket::{Status, TicketId},
    };
    use std::{convert::Infallible, str::FromStr};
    use ticket_fields::{TicketDescription, TicketTitle};
    use warp::{http::StatusCode, Reply};

    pub async fn list(db: Db) -> Result<impl Reply, Infallible> {
        let db = db.read().await;
        let tickets_futures = db.into_iter().map(|ticket| async move {
            let ticket = ticket.read().await;
            Ticket::from(ticket.clone())
        });
        let tickets: Vec<Ticket> = futures::future::join_all(tickets_futures).await;

        Ok(warp::reply::json(&tickets))
    }

    pub async fn create(input: CreateTicketInput, db: Db) -> Result<impl Reply, Infallible> {
        let mut db = db.write().await;
        match input.try_into() {
            Ok(ticket_draft) => {
                let new_ticket = db.add_ticket(ticket_draft);
                let new_ticket = new_ticket.read().await;
                Ok(warp::reply::with_status(
                    warp::reply::json(&Ticket::from(new_ticket.clone())),
                    StatusCode::OK,
                ))
            }
            Err(err) => Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse {
                    message: err.to_string(),
                }),
                StatusCode::BAD_REQUEST,
            )),
        }
    }

    pub async fn get(id: u64, db: Db) -> Result<impl Reply, Infallible> {
        let db = db.read().await;
        match db.get(TicketId::new(id)) {
            Some(ticket) => {
                let ticket = ticket.read().await;
                Ok(warp::reply::with_status(
                    warp::reply::json(&Ticket::from(ticket.clone())),
                    StatusCode::OK,
                ))
            }
            None => Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse {
                    message: "Ticket not found".into(),
                }),
                StatusCode::NOT_FOUND,
            )),
        }
    }

    pub async fn update(
        id: u64,
        input: UpdateTicketInput,
        db: Db,
    ) -> Result<impl Reply, Infallible> {
        let db = db.write().await;

        match db.get(TicketId::new(id)) {
            Some(ticket) => {
                let mut ticket = ticket.write().await;
                if let Some(title) = input.title {
                    match TicketTitle::try_from(title) {
                        Ok(title) => ticket.title = title,
                        Err(e) => {
                            return Ok(warp::reply::with_status(
                                warp::reply::json(&ErrorResponse {
                                    message: e.to_string(),
                                }),
                                StatusCode::BAD_REQUEST,
                            ));
                        }
                    };
                }
                if let Some(description) = input.description {
                    match TicketDescription::try_from(description) {
                        Ok(description) => ticket.description = description,
                        Err(e) => {
                            return Ok(warp::reply::with_status(
                                warp::reply::json(&ErrorResponse {
                                    message: e.to_string(),
                                }),
                                StatusCode::BAD_REQUEST,
                            ));
                        }
                    };
                }
                if let Some(status) = input.status {
                    match Status::from_str(status.as_str()) {
                        Ok(status) => ticket.status = status,
                        Err(_) => {
                            return Ok(warp::reply::with_status(
                                warp::reply::json(&ErrorResponse {
                                    message: "Invalid status".into(),
                                }),
                                StatusCode::BAD_REQUEST,
                            ));
                        }
                    };
                }

                Ok(warp::reply::with_status(
                    warp::reply::json(&Ticket::from(ticket.clone())),
                    StatusCode::OK,
                ))
            }
            None => Ok(warp::reply::with_status(
                warp::reply::json(&ErrorResponse {
                    message: "Ticket not found".into(),
                }),
                StatusCode::NOT_FOUND,
            )),
        }
    }
}
