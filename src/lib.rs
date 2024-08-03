#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_no_source)]
pub mod api;
pub mod core;
pub mod error;
mod utils;

#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub mod macros {
    pub use lark_bot_sdk_macros::*;
}

#[cfg(test)]
#[ctor::ctor]
fn init_test() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .is_test(true)
        .try_init()
        .unwrap();
}
