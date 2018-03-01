use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::thread::{Thread, NewThread};
use db::post::NewPost;
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::thread::{NewThreadRequest, ThreadResponse};
use requests_and_responses::thread::MinimalThreadResponse;
use chrono::Utc;
use auth::user_authorization::NormalUser;
use auth::user_authorization::ModeratorUser;

use routes::post::PostData;


impl From<NewThreadRequest> for NewThread {
    fn from(request: NewThreadRequest) -> NewThread {
        NewThread {
            forum_id: request.forum_id,
            author_id: request.author_id,
            created_date: Utc::now().naive_utc(),
            locked: false,
            archived: false,
            title: request.title,
        }
    }
}

impl From<NewThreadRequest> for NewPost {
    fn from(request: NewThreadRequest) -> NewPost {
        // Just grab the post field from the thread request.
        NewPost::from(request.post)
    }
}

pub struct ThreadData {
    pub thread: Thread,
    pub post: PostData,
    pub user: User,
}

impl From<ThreadData> for ThreadResponse {
    fn from(data: ThreadData) -> ThreadResponse {
        ThreadResponse {
            id: data.thread.id,
            title: data.thread.title,
            author: data.user.into(),
            posts: data.post.into(),
            created_date: data.thread.created_date,
            locked: data.thread.locked,
        }
    }
}

pub struct MinimalThreadData {
    pub thread: Thread,
    pub user: User,
}

impl From<MinimalThreadData> for MinimalThreadResponse {
    fn from(data: MinimalThreadData) -> MinimalThreadResponse {
        MinimalThreadResponse {
            id: data.thread.id,
            title: data.thread.title,
            author: data.user.into(),
            created_date: data.thread.created_date,
            locked: data.thread.locked,
        }
    }
}


// impl Thread {
//     /// The response requires both a post and a user to be attached.
//     fn into_one_post_thread_response(self, post: Post, user: User) -> ThreadResponse {
//         ThreadResponse {
//             id: self.id,
//             title: self.title,
//             author: user.clone().into(),
//             posts: post.into_childless_response(user),
//             created_date: self.created_date,
//             locked: self.locked,
//         }
//     }

//     fn into_full_thread_response(self, conn: &Conn) -> Result<ThreadResponse, WeekendAtJoesError> {
//         let post: Post = Post::get_root_post(self.id, conn)?;
//         let post_response: PostResponse = post.into_post_response(conn)?;
//         Ok(ThreadResponse {
//             id: self.id,
//             title: self.title,
//             author: post_response.author.clone(),
//             posts: post_response,
//             created_date: self.created_date,
//             locked: self.locked,
//         })
//     }

//     fn into_minimal_thread_response(self, user: User) -> MinimalThreadResponse {
//         MinimalThreadResponse {
//             id: self.id,
//             title: self.title,
//             author: user.clone().into(),
//             created_date: self.created_date,
//             locked: self.locked,
//         }
//     }
// }


#[post("/create", data = "<new_thread_request>")]
fn create_thread(new_thread_request: Json<NewThreadRequest>, _normal_user: NormalUser, conn: Conn) -> Result<Json<ThreadResponse>, WeekendAtJoesError> {
    let new_thread_request = new_thread_request.into_inner();

    let new_thread: NewThread = new_thread_request.clone().into();
    let new_original_post: NewPost = new_thread_request.into();

    Thread::create_thread_with_initial_post(new_thread, new_original_post, &conn)
        .map(ThreadResponse::from)
        .map(Json)
}

#[put("/lock/<thread_id>")]
fn lock_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<MinimalThreadResponse>, WeekendAtJoesError> {
    Thread::lock_thread(thread_id, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

#[put("/unlock/<thread_id>")]
fn unlock_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<MinimalThreadResponse>, WeekendAtJoesError> {
    Thread::unlock_thread(thread_id, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

#[delete("/archive/<thread_id>")]
fn archive_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<MinimalThreadResponse>, WeekendAtJoesError> {
    Thread::archive_thread(thread_id, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

#[get("/get/<forum_id>")]
fn get_threads_by_forum_id(forum_id: i32, conn: Conn) -> Result<Json<Vec<MinimalThreadResponse>>, WeekendAtJoesError> {
    // TODO move the 25 into a parameter
    // TODO make this more efficient by doing a join in the database method
    Thread::get_threads_in_forum(forum_id, 25, &conn)
        .map(|threads| {
            threads
                .into_iter()
                .map(MinimalThreadResponse::from)
                .collect()
        })
        .map(Json)
}


impl Routable for Thread {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            create_thread,
            lock_thread,
            unlock_thread,
            archive_thread,
            get_threads_by_forum_id,
        ]
    };
    const PATH: &'static str = "/thread/";
}
