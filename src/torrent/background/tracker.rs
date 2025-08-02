// pub async fn send_announce(url: &str, params: &AnnounceRequestParams) -> Result<AnnounceResponse> {
//     let query = params.to_query_string();
//     let full_url = format!("{url}?{query}");

//     let response = self
//         .client
//         .get(&full_url)
//         .headers(self.headers.clone())
//         .timeout(self.timeout)
//         .send()
//         .await?;

//     let bytes = response.bytes().await?;
//     AnnounceResponse::from_bytes(&bytes)?.check_failure_reason()
// }

// impl TrackerTransportLayer for HttpTrackerTransport {
//     fn announce<'a>(
//         &'a self,
//         url: &'a str,
//         params: &'a AnnounceRequestParams,
//     ) -> Pin<Box<dyn Future<Output = Result<HttpAnnounceResponse>> + Send + 'a>> {
//         Box::pin(async move {
//             let query = params.to_query_string();
//             let full_url = format!("{url}?{query}");

//             let response = self
//                 .client
//                 .get(&full_url)
//                 .headers(self.headers.clone())
//                 .timeout(self.timeout)
//                 .send()
//                 .await?;

//             let bytes = response.bytes().await?;
//             HttpAnnounceResponse::from_bytes(&bytes)?.check_failure_reason()
//         })
//     }
// }
