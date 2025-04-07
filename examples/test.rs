use mem_broker::{Broker, Subscriber};
use std::io::{self, Write};

struct EventOne {
    pub msg: String,
}

struct EventTwo {
    pub id: u32,
}

struct EventThree {
    pub x: f32,
    pub y: f32,
}

// First Subscriber Struct
struct TestSub {
    pub name: String,
}

// Second Subscriber Struct
struct AnotherSub {
    pub id: u32,
}

// Third Subscriber Struct
struct ThirdSub;

impl Subscriber<EventOne> for TestSub {
    fn subscribe_handler(&self, data: &EventOne) {
        println!("{} received EventOne: {}", self.name, data.msg);
    }
}

impl Subscriber<EventTwo> for TestSub {
    fn subscribe_handler(&self, data: &EventTwo) {
        println!("{} received EventTwo: id={}", self.name, data.id);
    }
}

impl Subscriber<EventThree> for TestSub {
    fn subscribe_handler(&self, data: &EventThree) {
        println!("{} received EventThree: x={}, y={}", self.name, data.x, data.y);
    }
}

// Another subscriber for EventOne and EventTwo
impl Subscriber<EventOne> for AnotherSub {
    fn subscribe_handler(&self, data: &EventOne) {
        println!("AnotherSub (id={}) received EventOne: {}", self.id, data.msg);
    }
}

impl Subscriber<EventTwo> for AnotherSub {
    fn subscribe_handler(&self, data: &EventTwo) {
        println!("AnotherSub (id={}) received EventTwo: id={}", self.id, data.id);
    }
}

// Another subscriber for EventThree
impl Subscriber<EventThree> for ThirdSub {
    fn subscribe_handler(&self, data: &EventThree) {
        println!("ThirdSub received EventThree: x={}, y={}", data.x, data.y);
    }
}

#[tokio::main]
async fn main() {
    let broker = Broker::default();

    // Static Subscriptions
    broker.subcribe::<EventOne, _>(TestSub { name: "SubOne".to_string() });
    broker.subcribe::<EventOne, _>(AnotherSub { id: 100 });
    
    broker.subcribe::<EventTwo, _>(TestSub { name: "SubTwo".to_string() });
    broker.subcribe::<EventTwo, _>(AnotherSub { id: 200 });

    broker.subcribe::<EventThree, _>(TestSub { name: "SubThree".to_string() });
    broker.subcribe::<EventThree, _>(ThirdSub);

    loop {
        print!("Choose event to send (1, 2, 3) or type 'exit': ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => {
                broker.publish(EventOne {
                    msg: "Hello from EventOne".to_string(),
                });
            }
            "2" => {
                broker.publish(EventTwo {
                    id: 42,
                });
            }
            "3" => {
                broker.publish(EventThree {
                    x: 3.1421,
                    y: 2.7181,
                });
            }
            "exit" => {
                println!("Exiting loop...");
                break;
            }
            _ => {
                println!("Invalid input!");
            }
        }
    }
}
