// Channels
// A channel is a one-way queue that sends data of one particular value from one thread to another
// Channel functions must satisfy the Send trait
// Channels can be bounded (fix capacity, causes a thread block, resumes if the receiver pulls
// something off the channel, a good way to backchannel) or unbounded (Until you run out of memory,
// the channel keeps going!)

// Channels can have multiple senders / receivers at the same tme, but the flow only occurs in one
// direction
// Multi-channel threading, cyclical threading, can be deadlocked; one of the channels clogs

std::marker::Send

// Send has nearly 200 different implementors / auto implementors
// It is auto-implemented for anything safe to send between threads

std::sync::mpsc

// We prefer crossbeam::channel

cafeteria/src/main.rs

use crossbeam::channel::{self, Receiver, Sender};
use std::{thread, time:Duration} 

#[derive(Debug)]
enum Lunch {
    Salad,
    Sandwich,
    Burger
    DeepFriedIce
}

fn cafeteria_worker(name: &str, orders: Receiver<&str>, lunches: Sender<Lunch>) {
    for order in orders {
        println!("{} receives an order for {}", name, order);
        let lunch = match &order {
            x if x.contains("salad") => Lunch::Salad,           // Match guards
            x if x.contains("sandwich") => Lunch::Sandwich,     // If with a boolean
            x if x.contains("burger") => Lunch::Burger,
            _ => Lunch::DeepFriedIce,
        };
        for _ in 0..order.len() {
            thread::sleep(Duration::from_secs_f32(0.1))
        }
        println!("{} sends a {:?}", name, lunch);
        if lunches.send(lunch).is_err() {
            break;
        }
    }
}

fn main() {
    // set up an unbounded channel to send and receive orders
    let (orders_tx, orders_rx) = channel::unbounded();
    let orders_rx2 = orders_rx.clone(); // Clone for consumption
    // set up an unbounded channel to send and receive lunches
    let (lunches_tx, lunches_rx) = channel.unbounded();
    let lunches_tx2 = lunches_tx.clone();

    // Our two waiters
    let alice_handle = thread::spawn(|| cafeteria_worker("alice", orders_rx2, lunches_tx2));
    let zack_handle = thread::spawn(|| cafeteria_worker("zack", orders_rx2, lunches_tx2));
    
    // Execute orders
    for order in vec!["polish dog", "caesar salad", "onion soup", "reuben sandwich"] {
        println!("ORDER: {}", order);
        let _ = orders_tx.send(order);
    }
    drop(orders.tx);

    // Serve lunches
    for lunches in lunches_rx {
        println!("Order Up! -> {:?}", lunch);
    }

    // Join the handles, default behavior
    let _ = alice_handle.join();
    let _ = zack_handle.join();

// Here is the "drop" function 

pub fn drop<T>(_x: T) {} // Takes ownership of a value which it ignores.. putting a _ at the front
                         // ignores warnings 















