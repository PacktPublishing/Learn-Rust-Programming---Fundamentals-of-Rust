pub struct Person {
    name: String,
    age: u32,
}

pub struct Ticket {
    ticket_num: u32,
    bought_by: Person,
}

pub struct Event {
    name: String,
    location: String,
    tickets: Vec<Ticket>,
}

impl Event {
    pub fn new(name: String, location: String) -> Self {
        Self { name, location, tickets: vec![] }
    }

    pub fn buy_ticket(&mut self, buyer: Person) -> Result<String, String> {
        let buyer_name = buyer.name.clone();

        if buyer.age >= 18 {
            let next_ticket_number: usize = self.tickets.len();
            let ticket = Ticket { ticket_num: next_ticket_number as u32, bought_by: buyer };

            self.tickets.push(ticket);

            Ok(buyer_name)
        } else {
            Err(format!("Buyer {} is under 18 years old.", buyer_name))
        }
    }
}

fn main() {
    let mut event = Event::new("90's party".into(), "Central Station, Amsterdam, The Netherlands".to_string());

    let alice = Person { name: String::from("Alice Peppermint"), age: 17 };
    let bob = Person { name: String::from("Bob Doe"), age: 18 };

    let buyers: Vec<Person> = vec![alice, bob];

    for buyer in buyers {
        match event.buy_ticket(buyer) {
            Ok(buyer_name) => println!("Ticket bought successfully for {}!", buyer_name),
            Err(error) => println!("Error occurred buying the ticket: {}", error),
        }
    }
}