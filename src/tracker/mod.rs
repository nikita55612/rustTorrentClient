// use crate::{error::Result, peers::PeerId, torrent::InfoHash};
mod event;
mod request;
mod response;

pub use event::Event;
pub use request::TrackerRequest;
pub use response::TrackerResponse;
// // трекеры
// // Запросы GET трекера имеют следующие ключи:

// // инфо_хэш
// // 20-байтовый sha1-хеш закодированной формы значения info из файла metainfo. Это значение почти наверняка придется экранировать.

// // Обратите внимание, что это подстрока файла metainfo. Информационный хэш должен быть хешем закодированной формы, как найдено в файле .torrent, что идентично bdecoding файла metainfo, извлечению словаря info и его кодированию, если и только если bdecoder полностью проверил входные данные (например, порядок ключей, отсутствие начальных нулей). И наоборот, это означает, что клиенты должны либо отклонить недействительные файлы metainfo, либо извлечь подстроку напрямую. Они не должны выполнять цикл декодирования-кодирования для недействительных данных.

// // peer_id
// // Строка длиной 20, которую этот загрузчик использует в качестве своего идентификатора. Каждый загрузчик генерирует свой собственный идентификатор случайным образом в начале новой загрузки. Это значение также почти наверняка придется экранировать.
// // ip
// // Необязательный параметр, дающий IP (или имя DNS), на котором находится этот пир. Обычно используется для источника, если он находится на той же машине, что и трекер.
// // порт
// // Номер порта, который прослушивает этот пир. Обычное поведение для загрузчика — попытаться прослушивать порт 6881, и если этот порт занят, попробовать 6882, затем 6883 и т. д. и сдаться после 6889.
// // загружено
// // Общая сумма загруженных на данный момент данных, закодированная в десятичном формате ASCII.
// // скачал
// // Общий объем загруженных на данный момент данных, закодированный в десятичном формате ASCII.
// // левый
// // Количество байт, которые этот пир еще должен загрузить, закодированное в десятичном ascii. Обратите внимание, что это не может быть вычислено из загруженного и длины файла, поскольку это может быть резюме, и есть вероятность, что некоторые из загруженных данных не прошли проверку целостности и их пришлось загрузить повторно.
// // событие
// // Это необязательный ключ, который сопоставляется с started , complete или stopped (или empty , что то же самое, что и отсутствовать). Если отсутствует, это одно из объявлений, сделанных через регулярные интервалы. Объявление с использованием started отправляется, когда загрузка начинается впервые, а объявление с использованием complete отправляется, когда загрузка завершается. No done не отправляется, если файл был завершен при запуске. Загрузчики отправляют объявление с использованием stopped, когда они прекращают загрузку.

// // Ответы трекера представляют собой закодированные словари. Если ответ трекера содержит ключ failure reason , то он сопоставляется с понятной человеку строкой, которая объясняет, почему запрос не удался, и никаких других ключей не требуется. В противном случае он должен иметь два ключа: interval , который сопоставляется с количеством секунд, которое загрузчик должен ждать между обычными повторными запросами, и peers . peers сопоставляется со списком словарей, соответствующих peers , каждый из которых содержит ключи peer id , ip и port , которые сопоставляются с самостоятельно выбранным идентификатором, IP-адресом или именем DNS однорангового узла в виде строки и номером порта соответственно. Обратите внимание, что загрузчики могут делать повторные запросы в незапланированное время, если происходит событие или им нужно больше одноранговых узлов.

// const DEFAULT_PORT: u16 = 6881;
// const DEFAULT_LEFT: u64 = 0;
// const DEFAULT_UPLOADED: u64 = 0;
// const DEFAULT_DOWNLOADED: u64 = 0;
// const DEFAULT_EVENT: Event = Event::Started;
// const USER_AGENT: &'static str = "uTorrent/2210(25110)";

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Event {
//     Started,
//     Completed,
//     Stopped,
// }

// impl Event {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Self::Started => "started",
//             Self::Completed => "completed",
//             Self::Stopped => "stopped",
//         }
//     }
// }

// #[derive(Debug)]
// pub struct TrackerRequest {
//     pub info_hash: InfoHash,
//     pub peer_id: PeerId,
//     pub port: u16,
//     pub left: u64,
//     pub uploaded: u64,
//     pub downloaded: u64,
//     pub event: Event,
// }

// impl Default for TrackerRequest {
//     fn default() -> Self {
//         Self {
//             info_hash: InfoHash::new(),
//             peer_id: PeerId::new(),
//             port: DEFAULT_PORT,
//             left: DEFAULT_LEFT,
//             uploaded: DEFAULT_UPLOADED,
//             downloaded: DEFAULT_DOWNLOADED,
//             event: DEFAULT_EVENT,
//         }
//     }
// }

// impl TrackerRequest {
//     pub fn with_info_hash(mut self, info_hash: &InfoHash) -> Self {
//         self.info_hash = info_hash.clone();
//         self
//     }

//     pub fn with_peer_id(mut self, peer_id: &PeerId) -> Self {
//         self.peer_id = peer_id.clone();
//         self
//     }

//     pub fn with_port(mut self, port: u16) -> Self {
//         self.port = port;
//         self
//     }

//     pub fn with_left(mut self, left: u64) -> Self {
//         self.left = left;
//         self
//     }

//     pub fn with_uploaded(mut self, uploaded: u64) -> Self {
//         self.uploaded = uploaded;
//         self
//     }

//     pub fn with_downloaded(mut self, downloaded: u64) -> Self {
//         self.downloaded = downloaded;
//         self
//     }

//     pub fn with_event(mut self, event: Event) -> Self {
//         self.event = event;
//         self
//     }

//     pub async fn fetch_with_retries(&self, announce: &str, retries: u32) -> Result<Vec<u8>> {
//         let client = reqwest::Client::default();
//         let url = format!(
//             "{}?{}&{}&{}&{}&{}&{}&{}",
//             announce,
//             format!("info_hash={}", self.info_hash.percent_encoding()),
//             format!(
//                 "peer_id={}",
//                 String::from_utf8_lossy(self.peer_id.as_slice()),
//             ),
//             format!("port={}", self.port),
//             format!("uploaded={}", self.uploaded),
//             format!("downloaded={}", self.downloaded),
//             format!("left={}", self.left),
//             format!("event={}", self.event.as_str()),
//         );

//         let bytes = {
//             let mut last_err = None;
//             let mut result = None;

//             for _ in 0..std::cmp::max(1, retries) {
//                 let request = client
//                     .get(&url)
//                     .timeout(std::time::Duration::from_secs(5))
//                     .header("User-Agent", USER_AGENT);

//                 match request.send().await {
//                     Ok(res) => {
//                         result = Some(res);
//                         break;
//                     }
//                     Err(e) => {
//                         last_err = Some(e);
//                     }
//                 }
//             }

//             result.ok_or_else(|| last_err.unwrap())?.bytes().await?
//         };

//         Ok(bytes.into())
//     }
// }
