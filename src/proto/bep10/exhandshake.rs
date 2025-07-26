use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Структура extended handshake-сообщения (обогащённая)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtendedHandshake {
    /// "m": карта поддерживаемых расширений
    pub m: Option<HashMap<String, u8>>,

    /// "v": строка версии клиента
    pub v: Option<String>,

    /// "metadata_size": если используется ut_metadata
    pub metadata_size: Option<u32>,

    /// "reqq": максимальное количество параллельных metadata-запросов
    pub reqq: Option<u32>,

    /// "yourip": IP-адрес отправителя (массив байтов)
    #[serde(default, with = "serde_bytes")]
    pub yourip: Option<Vec<u8>>,

    /// "ipv4": публичный IPv4 (опционально, по BEP 7)
    #[serde(default, with = "serde_bytes")]
    pub ipv4: Option<Vec<u8>>,

    /// "ipv6": публичный IPv6 (опционально, по BEP 7)
    #[serde(default, with = "serde_bytes")]
    pub ipv6: Option<Vec<u8>>,

    /// "p": порт, на котором слушает пир (по BEP 5/7)
    pub p: Option<u16>,
    // Дополнительные произвольные поля (всё, что не указано явно)
    // #[serde(flatten)]
    // pub extra: HashMap<String, Value>,
}
