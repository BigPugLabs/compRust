use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        println!("{} is trying to eat", &self.name);
        let mut locked = false;

        while !locked {
            let left_lock = self.left_fork.try_lock();
            let right_lock = self.right_fork.try_lock();
            match (left_lock, right_lock) {
                (Ok(_), Ok(_)) => {
                    println!("{} is eating...", &self.name);
                    locked = true;
                }
                (_, _) => thread::sleep(Duration::from_millis(1)),
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks
    let forks = [
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
    ];

    // Create philosophers
    let (tx, rx) = mpsc::channel();
    let philosophers = PHILOSOPHERS
        .iter()
        .enumerate()
        .map(|(index, name)| Philosopher {
            name: name.to_string(),
            left_fork: forks[index % forks.len()].clone(),
            right_fork: forks[(index + 1) % forks.len()].clone(),
            thoughts: tx.clone(),
        });

    // Make them think and eat
    for phil in philosophers {
        thread::spawn(move || {
            for _ in 0..100 {
                phil.eat();
                phil.think();
            }
        });
    }

    // Output their thoughts
    drop(tx);
    for thought in rx.iter() {
        println!("{thought}");
    }
}

