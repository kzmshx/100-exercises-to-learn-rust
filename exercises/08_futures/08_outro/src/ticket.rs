use std::{
    collections::{btree_map::Values, BTreeMap},
    sync::Arc,
};
use ticket_fields::{TicketDescription, TicketTitle};
use tokio::sync::RwLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

impl TicketId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, strum_macros::EnumString, strum_macros::Display)]
pub enum Status {
    #[strum(serialize = "To-Do")]
    ToDo,
    #[strum(serialize = "In-Progress")]
    InProgress,
    #[strum(serialize = "Done")]
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> Arc<RwLock<Ticket>> {
        let id = TicketId(self.counter);
        self.counter += 1;
        self.tickets.insert(
            id,
            Arc::new(RwLock::new(Ticket {
                id,
                title: ticket.title,
                description: ticket.description,
                status: Status::ToDo,
            })),
        );
        self.get(id).unwrap()
    }

    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }
}

impl Default for TicketStore {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Arc<RwLock<Ticket>>;
    type IntoIter = Values<'a, TicketId, Arc<RwLock<Ticket>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.values()
    }
}
