use base64::Engine;
use renet2::{
    transport::{CongestionControl, NetcodeClientTransport, ServerCertHash, WebTransportClient, WebTransportClientConfig},
    ConnectionConfig, DefaultChannel, RenetClient,
};
use renetcode2::ClientAuthentication;
use std::time::Duration;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_timer::{SystemTime, UNIX_EPOCH};

#[wasm_bindgen]
pub struct ChatApplication {
    renet_client: RenetClient,
    transport_client: NetcodeClientTransport,
    time_last_update: Duration,
    messages: Vec<String>,
}

#[wasm_bindgen]
impl ChatApplication {
    // Todo: this doesn't need to be async or return a result.
    pub async fn new() -> Result<ChatApplication, JsValue> {
        console_error_panic_hook::set_once();

        let server_addr = "127.0.0.1:4434".parse().unwrap();
        // >>>>>>>>>>> INSERT SERVER CERT HASH HERE <<<<<<<<<<<
        let server_cert_hash_b64 = "sQMkRdBGFypjga8araFrt3D7dTThtFppmHRxILyuUYc=";
        let hash = base64::engine::general_purpose::STANDARD.decode(server_cert_hash_b64).unwrap();

        let connection_config = ConnectionConfig::default();
        let client = RenetClient::new(connection_config);
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let client_auth = ClientAuthentication::Unsecure {
            client_id: current_time.as_millis() as u64,
            protocol_id: 0,
            socket_id: 1, //Webtransport socket id is 1 in this example
            server_addr,
            user_data: None,
        };
        let socket_config = WebTransportClientConfig {
            server_addr,
            congestion_control: CongestionControl::default(),
            server_cert_hashes: Vec::from([ServerCertHash::try_from(hash).unwrap()]),
        };
        let socket = WebTransportClient::new(socket_config);

        let transport: NetcodeClientTransport = NetcodeClientTransport::new(current_time, client_auth, socket).unwrap();

        Ok(Self {
            renet_client: client,
            transport_client: transport,
            time_last_update: current_time,
            messages: Vec::with_capacity(20),
        })
    }

    pub fn update(&mut self) {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let delta = current_time.saturating_sub(self.time_last_update);
        self.time_last_update = current_time;
        let _ = self.transport_client.update(delta, &mut self.renet_client);

        if let Some(message) = self.renet_client.receive_message(DefaultChannel::Unreliable) {
            let message = String::from_utf8(message.into()).unwrap();
            self.messages.push(message);
        };
    }

    pub fn is_disconnected(&self) -> bool {
        if self.transport_client.is_disconnected() {
            panic!("{:?}", self.transport_client.disconnect_reason());
        }
        false
    }

    pub fn send_packets(&mut self) {
        let _ = self.transport_client.send_packets(&mut self.renet_client);
    }

    pub fn send_message(&mut self, message: &str) {
        self.renet_client
            .send_message(DefaultChannel::Unreliable, message.as_bytes().to_vec());
    }

    pub fn disconnect(mut self) {
        self.transport_client.disconnect();
    }

    pub fn get_messages(&self) -> String {
        self.messages.join("\n")
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }
}
