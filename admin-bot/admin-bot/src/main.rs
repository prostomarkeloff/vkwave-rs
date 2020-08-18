use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use vkwave::api::api_context::{APIContext, APIOptions};
use vkwave::bots::event::{Event, RawContext};
use vkwave::longpoll::bot::BotLongpoll;
use vkwave::{generate_bruteforce_handlers, generate_dispatcher, generate_instant_search_handlers};
use vkwave_codegen::handler;
use vkwave_easy_contexts::EasyContext;
use vkwave_token::token::DefaultToken;

#[handler]
async fn pong(ev: EasyContext) {
    let start = Instant::now();
    ev.answer_str("ПОНГ!").await;
    let elapsed = start.elapsed();
    ev.answer_str(format!("Отправка сообщения заняла {:?}", elapsed))
        .await;
}

generate_instant_search_handlers! {
    InstantHandlers;
    [
        ["!пинг"] -> pong
    ]
}

generate_bruteforce_handlers! {
    BruteforceHandlers;
    [
        pong: ()
    ]
}

generate_dispatcher!(Dispatcher; InstantHandlers; BruteforceHandlers);

fn do_polling(mut lp: BotLongpoll<DefaultToken>, sender: UnboundedSender<Event>) {
    tokio::spawn(async move {
        loop {
            let updates = lp.get_updates().await;

            if let Ok(updates) = updates {
                for update in updates {
                    let _ = sender.send(update);
                }
            }
        }
    });
    loop {}
}

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let token = dotenv::var("GROUP_TOKEN").unwrap();
    let ctx = Arc::new(APIContext::new(APIOptions::new(DefaultToken::new(token))));
    let dp = Dispatcher::new(Arc::clone(&ctx));
    let mut lp = BotLongpoll::new(ctx, dotenv::var("GROUP_ID").unwrap());
    let (sender, receiver) = unbounded_channel();
    dp.run(receiver);
    do_polling(lp, sender);
}
