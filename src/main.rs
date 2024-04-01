use actix::{System, Actor, Context};

struct actor;

impl Actor for actor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx : &mut Self::Context) {
        println!("I am alive!");
        // System::current().stop();
    }

    fn stopped(&mut self, _ctx : &mut Self::Context) {
        println!("Stopping the server!");
    }
}

fn main() {
    let system = System::new();

    let _addr = system.block_on(async { actor.start() });
    system.run().unwrap();
}
