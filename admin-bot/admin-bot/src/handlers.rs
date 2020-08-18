use std::convert::TryFrom;
use vkwave::framework::extractors::arg::{self, Arg};
use vkwave::framework::raw_context::RawContext;
use vkwave_codegen::handler;

async fn filter_1(ctx: RawContext) -> bool {
    true
}

#[handler(filter_1)]
#[inline]
pub async fn echo(args: Arg<arg::AllArgs<arg::AllArgsAtLeast::One>>) -> String {
    args.value().join(" ")
}
