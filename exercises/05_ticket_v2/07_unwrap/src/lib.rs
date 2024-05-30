// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    return Ticket {
        title: TicketTitle::new(title).unwrap(),
        description: TicketDescription::new(description)
            .unwrap_or(TicketDescription("Description not provided".to_string())),
        status,
    };
}

#[derive(Clone, Debug, PartialEq)]
struct TicketTitle(String);

impl TicketTitle {
    fn new(title: String) -> Result<Self, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".to_string());
        }
        Ok(TicketTitle(title))
    }
}

impl PartialEq<&str> for TicketTitle {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TicketDescription(String);

impl TicketDescription {
    fn new(description: String) -> Result<Self, String> {
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".to_string());
        }
        Ok(TicketDescription(description))
    }
}

impl PartialEq<&str> for TicketDescription {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, String> {
        if let Err(e) = TicketTitle::new(title.clone()) {
            return Err(e);
        }
        if let Err(e) = TicketDescription::new(description.clone()) {
            return Err(e);
        }
        Ok(Ticket {
            title: TicketTitle(title),
            description: TicketDescription(description),
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
