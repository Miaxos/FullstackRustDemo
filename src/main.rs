#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(rand)]
#![feature(test)]
#![recursion_limit="128"]
// #![feature(proc_macro)]


#![feature(use_extern_macros)]

#[macro_use]
extern crate db_proc_macros;

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate uuid;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;
extern crate simplelog;

extern crate test;

extern crate frank_jwt;

#[macro_use]
extern crate diesel;
//#[macro_use] extern crate diesel_codegen;
#[macro_use]
extern crate diesel_infer_schema;
// #[macro_use] extern crate diesel_derive_enum;
extern crate chrono;
extern crate r2d2_diesel;
extern crate r2d2;

extern crate slug;
// #[macro_use]
// extern crate lazy_static;

// extern crate bcrypt;
extern crate crypto;

extern crate rand;

use rocket::Rocket;

mod conversions;
mod routes;
use routes::*;
mod db;
mod auth;
mod error;
use auth::{Secret, BannedSet};
use db::user::User;
use db::article::Article;
use db::forum::Forum;
use db::thread::Thread;
use db::post::Post;
use db::bucket::Bucket;
use db::question::Question;
use db::answer::Answer;
use db::chat::Chat;
use db::message::Message;

extern crate requests_and_responses;


use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::fs::File;

pub use db::schema; // schema internals can be accessed via db::schema::, or via schema::


fn main() {

    const LOGFILE_NAME: &'static str = "weekend.log";
    CombinedLogger::init(vec![
        TermLogger::new(LogLevelFilter::Info, Config::default())
            .unwrap(),
        WriteLogger::new(LogLevelFilter::Trace, Config::default(), File::create(LOGFILE_NAME).unwrap()),
    ]).unwrap();

    init_rocket().launch();
}

///Initialize the webserver
pub fn init_rocket() -> Rocket {

    let secret = Secret::generate();
    let banned_set = BannedSet::new();

    rocket::ignite()
        .manage(db::init_pool())
        .manage(secret)
        .manage(banned_set)
        .mount("/", routes![static_file::files, static_file::js, static_file::app, static_file::wasm])
        .mount(&format_api(User::PATH), User::ROUTES())
        .mount(&format_api(Article::PATH), Article::ROUTES())
        .mount(&format_api(Auth::PATH), Auth::ROUTES())
        .mount(&format_api(Forum::PATH), Forum::ROUTES())
        .mount(&format_api(Thread::PATH), Thread::ROUTES())
        .mount(&format_api(Post::PATH), Post::ROUTES())
        .mount(&format_api(Bucket::PATH), Bucket::ROUTES())
        .mount(&format_api(Question::PATH), Question::ROUTES())
        .mount(&format_api(Answer::PATH), Answer::ROUTES())
        .mount(&format_api(Chat::PATH), Chat::ROUTES())
        .mount(&format_api(Message::PATH), Message::ROUTES())
}


///Path should be an &str that starts with a /
fn format_api(path: &str) -> String {
    String::from("/api") + path
}


use std::sync::{Once, ONCE_INIT};

static INIT: Once = ONCE_INIT;

/// Setup function that is only run once, even if called multiple times.
pub fn test_setup() {
    INIT.call_once(|| {

        const LOGFILE_NAME: &'static str = "weekend_test.log";
        CombinedLogger::init(vec![
            TermLogger::new(LogLevelFilter::Info, Config::default())
                .unwrap(),
            WriteLogger::new(LogLevelFilter::Trace, Config::default(), File::create(LOGFILE_NAME).unwrap()),
        ]).unwrap();
    });
}
