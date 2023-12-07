#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]
// had to modify leptos-use/use_websocket, so that it works with rykv

use crate::common::{
    input::{ActionInput, GameInput},
    server_message::ServerMessage,
};
use cfg_if::cfg_if;
use leptos::*;
use rkyv::{from_bytes, to_bytes};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{MessageEvent, WebSocket};

#[derive(Clone)]
pub struct WsInner {
    inner: Option<WebSocket>,
}

pub type Ws = StoredValue<WsInner>;

impl WsInner {
    #[cfg(not(feature = "ssr"))]
    pub fn new(url: &str) -> Self {
        let url = normalize_url(url);
        let ws = if let Ok(ws) = WebSocket::new(&url) {
            ws
        } else {
            return WsInner { inner: None };
        };

        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        WsInner { inner: Some(ws) }
    }

    #[cfg(feature = "ssr")]
    pub fn new(url: &str) -> Self {
        WsInner { inner: None }
    }

    pub fn ready(&self) -> bool {
        self.inner
            .as_ref()
            .is_some_and(|ws| ws.ready_state() == WebSocket::OPEN)
    }

    fn send(&self, data: &[u8]) {
        if let Some(ws) = self.inner.as_ref() {
            if ws.ready_state() == WebSocket::OPEN {
                log::debug!("sending: {:?}", data);
                ws.send_with_u8_array(data);
            }
        }
    }

    pub fn send_game_input(&self, input: ActionInput) {
        if self.ready() {
            let input = GameInput::Action(input);
            let data = to_bytes::<_, 4>(&input).unwrap();
            self.send(data.as_slice());
        }
    }

    pub fn set_onmessage(&self, callback: impl Fn(ServerMessage) + 'static) {
        if let Some(ws) = self.inner.as_ref() {
            let wrap = Closure::wrap(Box::new(move |event: MessageEvent| {
                event.data().dyn_into::<js_sys::ArrayBuffer>().map_or_else(
                    |_| {
                        panic!("Received string instead of bytes.");
                    },
                    |array_buffer| {
                        let array = js_sys::Uint8Array::new(&array_buffer);
                        let array = array.to_vec();
                        let data = from_bytes::<ServerMessage>(&array).unwrap();
                        callback(data);
                    },
                );
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(wrap.as_ref().unchecked_ref()));
        }
    }

    pub fn close(self) {
        if let Some(ws) = self.inner {
            ws.close();
        }
    }
}

// credits: leptos-use/src/use_websocket.rs
// source: https://github.com/Synphonyte/leptos-use/blob/main/src/use_websocket.rs
fn normalize_url(url: &str) -> String {
    cfg_if! { if #[cfg(feature = "ssr")] {
        url.to_string()
    } else {
        if url.starts_with("ws://") || url.starts_with("wss://") {
            url.to_string()
        } else if url.starts_with("//") {
            format!("{}{}", detect_protocol(), url)
        } else if url.starts_with('/') {
            format!(
                "{}//{}{}",
                detect_protocol(),
                window().location().host().expect("Host not found"),
                url
            )
        } else {
            let mut path = window().location().pathname().expect("Pathname not found");
            if !path.ends_with('/') {
                path.push('/')
            }
            format!(
                "{}//{}{}{}",
                detect_protocol(),
                window().location().host().expect("Host not found"),
                path,
                url
            )
        }
    }}
}

fn detect_protocol() -> String {
    cfg_if! { if #[cfg(feature = "ssr")] {
        "ws".to_string()
    } else {
        window().location().protocol().expect("Protocol not found").replace("http", "ws")
    }}
}

