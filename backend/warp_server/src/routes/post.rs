use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use crate::error::Error;
use crate::db_integration::db_filter;
use db::Conn;
use uuid::Uuid;
use crate::convert_and_json;
use crate::convert_vector_and_json;
use crate::json_body_filter;
use identifiers::user::UserUuid;
use crate::jwt::normal_user_filter;
use wire::post::NewPostRequest;
use db::Post;
use wire::post::PostResponse;
use db::post::NewPost;
use db::post::ChildlessPostData;
use wire::post::EditPostRequest;
use identifiers::thread::ThreadUuid;
use db::RetrievableUuid;
use db::post::EditPostChangeset;
use crate::jwt::moderator_user_filter;
use identifiers::post::PostUuid;
use crate::uuid_integration::uuid_filter;


pub fn post_api() -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Post API");
    let api = create_post()
        .or(edit_post())
        .or(censor_post())
        .or(get_posts_by_user())
        ;

    warp::path("post")
        .and(api)
        .with(warp::log("post"))
        .boxed()
}


pub fn create_post() -> BoxedFilter<(impl Reply,)> {
    warp::post2()
        .and(json_body_filter(12))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: NewPostRequest, user_uuid: UserUuid, conn: Conn| {
            // check if token user id matches the request user id.
            // This prevents users from creating posts under other user's names.
            let new_post: NewPost = request.into();
            if new_post.author_uuid != user_uuid.0 {
                return Error::BadRequest.reject()
            }
            Post::create_and_get_user(new_post, &conn)
                .map(convert_and_json::<ChildlessPostData, PostResponse>)
                .map_err(Error::convert_and_reject)

        })
        .boxed()
}

pub fn edit_post() -> BoxedFilter<(impl Reply,)> {
    warp::put2()
        .and(json_body_filter(12))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: EditPostRequest, user_uuid: UserUuid, conn: Conn|{
             // Prevent editing other users posts
             let existing_post = Post::get_by_uuid(request.uuid.0, &conn).map_err(Error::convert_and_reject)?;
             if user_uuid.0 != existing_post.author_uuid {
                 return Error::BadRequest.reject()
             }

             let edit_post_request: EditPostRequest = request;
             let edit_post_changeset: EditPostChangeset = edit_post_request.clone().into();
             let thread_id: ThreadUuid = edit_post_request.thread_uuid;
             Post::modify_post(edit_post_changeset, thread_id, user_uuid, &conn)
                .map(convert_and_json::<ChildlessPostData, PostResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

pub fn censor_post() -> BoxedFilter<(impl Reply,)> {
    warp::put2()
        .and(warp::path("censor"))
        .and(warp::path::param::<Uuid>())
        .and(moderator_user_filter())
        .and(db_filter())
        .and_then(|post_uuid: Uuid, _user: UserUuid, conn: Conn| {
            let post_uuid = PostUuid(post_uuid);
            Post::censor_post(post_uuid, &conn)
                .map(convert_and_json::<ChildlessPostData, PostResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

pub fn get_posts_by_user() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path("users_posts"))
        .and(uuid_filter())
        .and(db_filter())
        .and_then(|user_uuid: Uuid, conn:Conn| {
            let user_uuid = UserUuid(user_uuid);
            Post::get_posts_by_user(user_uuid, &conn)
                .map(convert_vector_and_json::<ChildlessPostData, PostResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}