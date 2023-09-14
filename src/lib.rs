use std::pin::Pin;
use std::sync::mpsc::sync_channel;
use std::{future::Future, time::Duration};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    fn await_for() -> JsValue;
}

#[wasm_bindgen]
pub fn test() {
    console_error_panic_hook::set_once();
    console_log::init().unwrap();
    let (tx, rx) = sync_channel(1);
    log::info!("start spawn local");
    wasm_bindgen_futures::spawn_local(async move {
        log::info!("inside spawn local");
        let promise = js_sys::Promise::from(await_for());
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        log::info!("awaiting future");
        let _ = future.await.unwrap();
        log::info!("future is done");
        tx.send(1).unwrap();
    });
    log::info!("end spawn local");
    log::info!("create duration");
    let dur = Duration::from_secs(10);
    log::info!("wait");
    rx.recv_timeout(dur).unwrap();
    log::info!("function returns");
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    use wasm_bindgen_test::*;
    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1, 1);
    }
}
