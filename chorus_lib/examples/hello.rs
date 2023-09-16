extern crate chorus_lib;

use std::thread;

use chorus_lib::core::{ChoreoOp, Choreography, ChoreographyLocation, Projector};
use chorus_lib::transport::local::LocalTransport;
use chorus_lib::{hlist, projector};

// --- Define two locations (Alice and Bob) ---

#[derive(ChoreographyLocation)]
struct Alice;

#[derive(ChoreographyLocation)]
struct Bob;

// --- Define a choreography ---
struct HelloWorldChoreography;

// Implement the `Choreography` trait for `HelloWorldChoreography`
impl Choreography for HelloWorldChoreography {
    type L = hlist!(Alice, Bob);
    fn run(self, op: &impl ChoreoOp<Self::L>) {
        // Create a located value at Alice
        let msg_at_alice = op.locally(Alice, |_| {
            println!("Hello from Alice!");
            "Hello from Alice!".to_string()
        });
        // Send the located value to Bob
        let msg_at_bob = op.comm(Alice, Bob, &msg_at_alice);
        // Print the received message at Bob
        op.locally(Bob, |un| {
            let msg = un.unwrap(&msg_at_bob);
            println!("Bob received a message: {}", msg);
            msg
        });
    }
}

fn main() {
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    // Create a local transport
    let transport = LocalTransport::from(&[Alice::name(), Bob::name()]);
    // Crate available locations for Projector
    type AL = hlist!(Alice, Bob);

    // Run the choreography in two threads
    {
        let transport = transport.clone();
        handles.push(thread::spawn(move || {
            let p = projector!(AL, Alice, transport);
            p.epp_and_run(HelloWorldChoreography);
        }));
    }
    {
        let transport = transport.clone();
        handles.push(thread::spawn(move || {
            let p = projector!(AL, Bob, transport);
            p.epp_and_run(HelloWorldChoreography);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
