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
            bot.chat("Conducting the starting sequence!!");

            bot.look_at(Vec3 { x: bot.position().x, y: bot.position().y - 1, z: bot.position().z });
            let block_placing_pos = Vec3 { x: bot.position().x, y: bot.position().y - 1, z: bot.position().z };
            bot.mine(&block_placing_pos);
            bot.mine(Vec3 { x: bot.position().x, y: bot.position().y - 1, z: bot.position().z });
            bot.mine(Vec3 { x: bot.position().x, y: bot.position().y - 1, z: bot.position().z });
            bot.look_at(block_placing_pos.with_x(block_placing_pos.x + 1));
            bot.block_interact(block_placing_pos.with_x(block_placing_pos.x + 1));
        },
        Event::tick =>  {
            todo!()
        },
        _ => ()
    }

}
