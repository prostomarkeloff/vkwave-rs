use crate::api::api_context::APIContext;
use vkwave_token::token::DefaultToken;

// for more performance we generate dispatcher with macro

mod macros {
    #[macro_export]
    macro_rules! generate_dispatcher {
        ($name:ident; $instant_search_handler:ident; $bruteforce_handler:ident) => {
            use futures::stream::StreamExt;
            use tokio::sync::mpsc::UnboundedReceiver as __Receiver;
            use vkwave::bots::event::Event as __Event;
            type __API_CTX = std::sync::Arc<
                $crate::api::api_context::APIContext<vkwave_token::token::DefaultToken>,
            >;
            pub struct $name {
                api_ctx: __API_CTX,
            }

            impl $name {
                pub fn new(api_ctx: __API_CTX) -> Self {
                    Self { api_ctx }
                }

                pub fn run(self, receiver: __Receiver<__Event>) {
                    tokio::spawn(async move {
                        receiver
                            .for_each_concurrent(None, |ev| {
                                let api_ctx = std::sync::Arc::clone(&self.api_ctx);
                                async move {
                                    let arced_event = std::sync::Arc::new(ev);
                                    let context =
                                        $crate::bots::event::RawContext(arced_event, api_ctx);

                                    let result =
                                        $instant_search_handler.handle(context.clone()).await;
                                    if result {
                                        return;
                                    }
                                    let result = $bruteforce_handler.handle(context).await;
                                    if result {
                                        return;
                                    }
                                }
                            })
                            .await;
                    });
                }
            }
        };
    }
}
