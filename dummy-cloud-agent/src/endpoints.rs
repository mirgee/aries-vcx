use actix::prelude::*;
use actix_web::*;//{AsyncResponder, FutureResponse, HttpMessage, HttpRequest, HttpResponse, State, Error};
use actors::forward_agent::{ForwardAgent, GetForwardDetail, ForwardMessage};
use futures::*;

pub struct AppState {
    pub forward_agent: Addr<ForwardAgent>,
}

pub fn get(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.forward_agent
        .send(GetForwardDetail {})
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

pub fn post_msg((state, req): (State<AppState>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    req
        .body()
        .from_err()
        .and_then(move |body| {
            state.forward_agent
                .send(ForwardMessage(body.to_vec()))
                .from_err()
                .and_then(|res| match res {
                    Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                    Err(_) => Ok(HttpResponse::InternalServerError().into()),
                })
        })
        .responder()
}
