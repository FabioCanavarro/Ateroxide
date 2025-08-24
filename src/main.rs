use std::sync::Arc;

use azalea::{ecs::event::Event, Account, BotClientExt, Client, ClientBuilder, Event, Vec3};
use parking_lot::Mutex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let acc = Account::offline("Ateroxide");
    let server = "localhost";

    ClientBuilder::new()
        .set_handler(handle)
        .start(acc, server)
        .await.unwrap()
    ;
}

pub struct State {
    pub message_rec: Arc<Mutex<usize>>,
    pub cur_time: Arc<Mutex<u64>>

}

async fn handle(bot: Client, event: Event, state: State) {
    match event {
        Event::Spawn => {
            bot.look_at(Vec3 { x: bot.position().x, y: bot.position().y - 2, z: bot.position().z });
            bot.mine(Vec3 { x: bot.position().x, y: bot.position().y - 2, z: bot.position().z });
            bot.mine(Vec3 { x: bot.position().x, y: bot.position().y - 2, z: bot.position().z });

        },
        _ => ()
    }

}
