mod macros {
    #[macro_export]
    macro_rules! generate_instant_search_handlers {
        ($name:ident; [$([$($text:literal),+] -> $handler:ident),+]) => {
            #[derive(Clone, Copy)]
            pub struct $name;
            impl $name {
                async fn handle(&self, ev: $crate::bots::event::RawContext) -> bool {
                    match $crate::bots::event::get_text_from_event(&ev) {
                        $($($text)|+ => {
                            let r = $handler.handle(ev).await;
                            r
                        }),+
                        _ => false
                    }
                }
            }
        };
    }

    #[macro_export]
    macro_rules! generate_bruteforce_handlers {
        (@impl_handler_concrete; $ev:ident; $handler:ident: ()) => {
           if true {
                let lresult = $handler.handle($ev.clone()).await;
                match lresult {
                    true => return true,
                    false => return false
                };
           }
        };

        (@impl_handler_concrete; $ev:ident; $handler:ident:) => {
            generate_bruteforce_handlers!(@impl_handler_concrete; $ev; $handler: ())
        };

        (@impl_handler_concrete; $ev:ident; $handler:ident: $($filter:ident),+) => {
            if $($filter.filter(ev.clone()).await)||+ {
                let lresult = $handler.handle($ev.clone()).await;
                match lresult {
                    true => return true,
                    false => return false
                };
            }
        };

        (@impl_handler; $ev:ident; $handler:ident: $($filter:ident),*) => {
            generate_bruteforce_handlers!(@impl_handler_concrete; $ev; $handler: $($filter:ident),*)
        };

        ($name:ident; [$($handler:ident: ($($filter:ident),*)),+]) => {
            #[derive(Clone, Copy)]
            pub struct $name;

            impl $name {
                async fn handle(&self, ev: $crate::bots::event::RawContext) -> bool {
                    $(
                        generate_bruteforce_handlers!(@impl_handler; ev; $handler: $($filter),*);
                    )+

                    return false;
                }
            }
        }
    }
}
