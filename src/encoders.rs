use rocket::{http::Header, Request, Response};

pub struct GzipResponder {
    pub body: Vec<u8>,
}

impl<'r> rocket::response::Responder<'r, 'static> for GzipResponder {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut response = Response::new();
        response.set_header(Header::new("Content-Encoding", "gzip"));
        response.set_sized_body(self.body.len(), std::io::Cursor::new(self.body));
        Ok(response)
    }
}
