use yew::prelude::*;
use Context;
use yew::format::{Json, Nothing};

use yew::services::fetch::Response;
use yew::services::fetch::Request;

use requests_and_responses::forum::ForumResponse;

use datatypes::forum::ForumData;

mod forum_card_component;
use self::forum_card_component::ForumCardComponent;

pub struct Forum {
    pub child: Option<ForumData>,
    pub forums: Vec<ForumData>
}


pub enum Msg {
    ContentReady(Vec<ForumData>),
    SetChild(ForumData)
}

#[derive(Clone, PartialEq)]
pub struct Props {
}

impl Default for Props {
    fn default() -> Self {
        Props {
        }
    }
}

impl Component<Context> for Forum {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        let callback = context.send_back(|response: Response<Json<Result<Vec<ForumResponse>, ()>>>| {
            let (meta, Json(data)) = response.into_parts();
            println!("META: {:?}, {:?}", meta, data);
            Msg::ContentReady(data.unwrap().into_iter().map(ForumData::from).collect())
        });
        let request = Request::get("http://localhost:8001/api/forum/forums")
            .header("Content-Type", "application/json")
            .body(Nothing)
            .unwrap();
        let task = context.networking.fetch(request, callback);

        Forum {
            child: None,
            forums: vec!()
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetChild(fd) => {
                true
            }
            Msg::ContentReady(forums) => {
                self.forums = forums;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, Forum> for Forum {

    fn view(&self) -> Html<Context, Self> {

        let forum_card = |x: &ForumData| html! {
            <ForumCardComponent: forum_data=x, callback=|fd| Msg::SetChild(fd), />
        };

        let page = || {
            if let Some(ref child) = self.child {
                html!{
                    <>
                        {"I am the forum view"}
                    <>
                }

            } else {
                html!{
                    <>
                        {for self.forums.iter().map(forum_card) }
                        {"There should be a list of forums here."}
                    </>
                }
            }
        };

        return html! {
            <>
                {page()}
            </>
        }
    }
}
