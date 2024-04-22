use std::{collections::VecDeque, task::Poll};

use futures::Stream;
use salvo::{http::HeaderMap, prelude::*};

use crate::HttpResult;

#[allow(dead_code)]
pub(crate) fn common_assets(pub_dir: String, listing: bool) -> Router {
    Router::with_path("public/<**path>").get(
        StaticDir::new([pub_dir])
            .defaults("index.html")
            .auto_list(listing),
    )
}

#[handler]
pub(crate) async fn favicon_ico(res: &mut Response) -> HttpResult<()> {
    res.send_file("./favicon.ico", &HeaderMap::new()).await;
    Ok(())
}

/// Response to the clients with the memory data by streaming
pub struct MemoryStream(VecDeque<Vec<u8>>);

impl MemoryStream {
    #[allow(dead_code)]
    pub fn new(data: Vec<u8>, chunk_size: usize) -> Self {
        Self(data.chunks(chunk_size).map(|v| v.to_vec()).collect())
    }
}

impl Stream for MemoryStream {
    type Item = Result<Vec<u8>, std::io::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if let Some(frame) = self.0.pop_front() {
            Poll::Ready(Some(Ok(frame)))
        } else {
            Poll::Ready(None)
        }
    }
}
