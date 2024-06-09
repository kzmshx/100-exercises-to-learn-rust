use crate::ticket::TicketDraft;
use serde_derive::{Deserialize, Serialize};
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl TryFrom<&[u8]> for ErrorResponse {
    type Error = serde_json::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice::<ErrorResponse>(value)
    }
}

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
