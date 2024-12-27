use rouille::Response;
use rusqlite::{Connection, Error::QueryReturnedNoRows};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

pub fn start_server(db_ctx: Arc<Mutex<Connection>>) -> JoinHandle<()> {
    thread::spawn(move || {
        rouille::start_server("0.0.0.0:7584", move |request| {
            println!("Received request {:#?}", request);
            match request.url().as_str() {
                "/" => Response::text("Stop screwing around with our server"),
                "/passport" => {
                    if let Some(token) = request.get_param("token") {
                        let ctx = db_ctx.lock().unwrap();
                        let mut statement = ctx
                            .prepare("SELECT data FROM passports WHERE token=(?1)")
                            .unwrap();
                        match statement
                            .query_row([token.clone()], |r| Ok(r.get::<usize, String>(0)?))
                        {
                            Ok(data) => {
                                Response::text(format!("Here's your response BUDDY: {}", data))
                            }
                            Err(QueryReturnedNoRows) => {
                                Response::text(format!("No data for token {}", token))
                                    .with_status_code(404)
                            }
                            Err(e) => {
                                Response::text(format!("Server Error: {}", e)).with_status_code(500)
                            }
                        }
                    } else {
                        Response::text("No token provided").with_status_code(400)
                    }
                }
                "/coffee" => Response::text("I am NOT a coffee maker!!11").with_status_code(418),
                _ => Response::empty_404(),
            }
        })
    })
}
