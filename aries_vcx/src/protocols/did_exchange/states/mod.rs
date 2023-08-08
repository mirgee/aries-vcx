pub mod abandoned;
pub mod completed;
pub mod requester;
pub mod responder;
pub mod traits;

pub enum States {
    RequestSent(requester::request_sent::RequestSent),
    ResponseSent(responder::response_sent::ResponseSent),
    Abandoned(abandoned::Abandoned),
    Completed(completed::Completed),
}
