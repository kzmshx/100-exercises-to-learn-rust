pub mod db {
    use crate::ticket::TicketStore;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    pub type Db = Arc<RwLock<TicketStore>>;

    pub fn create_db() -> Db {
        Arc::new(RwLock::new(TicketStore::default()))
    }
}

pub mod models {
    use serde_derive::{Deserialize, Serialize};
    use ticket_fields::{TicketDescription, TicketTitle};

    use crate::ticket::TicketDraft;

    #[derive(Deserialize, Serialize)]
    pub struct Ticket {
        pub id: u64,
        pub title: String,
        pub description: String,
        pub status: String,
    }

    impl From<crate::ticket::Ticket> for Ticket {
        fn from(value: crate::ticket::Ticket) -> Self {
            Self {
                id: value.id.value(),
                title: format!("{}", value.title),
                description: format!("{}", value.description),
                status: format!("{}", value.status),
            }
        }
    }

    impl TryFrom<&[u8]> for Ticket {
        type Error = serde_json::Error;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            serde_json::from_slice::<Ticket>(value)
        }
    }

    #[derive(Deserialize, Serialize)]
    pub struct CreateTicketInput {
        pub title: String,
        pub description: String,
    }

    impl TryInto<crate::ticket::TicketDraft> for CreateTicketInput {
        type Error = anyhow::Error;

        fn try_into(self) -> Result<crate::ticket::TicketDraft, Self::Error> {
            let title = TicketTitle::try_from(self.title)?;
            let description = TicketDescription::try_from(self.description)?;
            Ok(TicketDraft { title, description })
        }
    }

    #[derive(Deserialize, Serialize)]
    pub struct UpdateTicketInput {
        pub id: u64,
        pub title: Option<String>,
        pub description: Option<String>,
        pub status: Option<String>,
    }
}

pub mod filters {
    use std::convert::Infallible;

    use crate::api::db::Db;
    use crate::api::handlers;
    use warp::Filter;

    pub fn all(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        hello().or(tickets(db))
    }

    pub fn hello() -> impl Filter<Extract = (&'static str,), Error = warp::Rejection> + Clone {
        warp::path!("hello")
            .and(warp::get())
            .map(|| "Hello, world!")
    }

    pub fn tickets(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        tickets_get(db.clone())
            .or(tickets_post(db.clone()))
            .or(tickets_id_get(db.clone()))
            .or(tickets_id_put(db.clone()))
    }

    /// GET /tickets
    pub fn tickets_get(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("tickets")
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::tickets::list)
    }

    /// POST /tickets
    pub fn tickets_post(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("tickets")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db))
            .and_then(handlers::tickets::create)
    }

    /// GET /tickets/:id
    pub fn tickets_id_get(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("tickets" / u64)
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::tickets::get)
    }

    /// PUT /tickets/:id
    pub fn tickets_id_put(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("tickets" / u64)
            .and(warp::put())
            .and(warp::body::json())
            .and(with_db(db))
            .and_then(handlers::tickets::update)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

pub mod handlers {
    pub mod tickets {
        use ticket_fields::{TicketDescription, TicketTitle};

        use crate::{
            api::{
                db::Db,
                models::{CreateTicketInput, Ticket, UpdateTicketInput},
            },
            ticket::{Status, TicketId},
        };
        use std::{convert::Infallible, str::FromStr};

        pub async fn list(db: Db) -> Result<impl warp::Reply, Infallible> {
            let db = db.read().await;
            let tickets_futures = db.into_iter().map(|ticket| async move {
                let ticket = ticket.read().await;
                Ticket::from(ticket.clone())
            });
            let tickets: Vec<Ticket> = futures::future::join_all(tickets_futures).await;

            Ok(warp::reply::json(&tickets))
        }

        pub async fn create(
            input: CreateTicketInput,
            db: Db,
        ) -> Result<impl warp::Reply, Infallible> {
            let db = db.clone();
            let mut db = db.write().await;

            let ticket_draft = input.try_into().unwrap();
            let new_ticket = db.add_ticket(ticket_draft);
            let new_ticket = new_ticket.read().await;

            Ok(warp::reply::json(&Ticket::from(new_ticket.clone())))
        }

        pub async fn get(id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
            let db = db.read().await;
            let id = TicketId::new(id);
            let ticket = db.get(id).unwrap();
            let ticket = ticket.read().await;

            Ok(warp::reply::json(&Ticket::from(ticket.clone())))
        }

        pub async fn update(
            id: u64,
            input: UpdateTicketInput,
            db: Db,
        ) -> Result<impl warp::Reply, Infallible> {
            let db = db.write().await;
            let id = TicketId::new(id);
            let ticket = db.get(id).unwrap();
            let mut ticket = ticket.write().await;

            if let Some(title) = input.title {
                ticket.title = TicketTitle::try_from(title).unwrap();
            }
            if let Some(description) = input.description {
                ticket.description = TicketDescription::try_from(description).unwrap();
            }
            if let Some(status) = input.status {
                ticket.status = Status::from_str(status.as_str()).unwrap();
            }

            Ok(warp::reply::json(&Ticket::from(ticket.clone())))
        }
    }
}
