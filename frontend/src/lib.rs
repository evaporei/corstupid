use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::format::{Text, Json};
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
};
use serde_json::json;

struct Model {
    link: ComponentLink<Self>,
    fetch: FetchService,
    fetch_task: Option<FetchTask>,
    fetching: bool,
    response: Option<String>,
}

pub enum Msg {
    FetchData(Option<Text>),
}

impl Model {
    pub fn fetch_data(&mut self) {
        let body = json!({
            "username": "asdf",
            "email": "asdf",
            "password": "asdf",
            "confirm_password": "asdf"
        });

        let request = Request::builder()
            .uri("http://localhost:8000/user/info")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Json(&body))
            .expect("Failed to build request.");
  
          let callback = self.link.callback(
              move |response: Response<Text>| {
                  let (meta, data) = response.into_parts();
                  if meta.status.is_success() {
                      Msg::FetchData(Some(data))
                  } else {
                      Msg::FetchData(None)
                  }
              },
          );

          let task = self.fetch.fetch(request, callback).unwrap();
          self.fetch_task = Some(task);
          self.fetching = false;
      }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch: FetchService::new(),
            fetch_task: None,
            fetching: false,
            response: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.fetching = true;
            self.fetch_data(); 
        }
     }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData(data) => {
                self.response = match data {
                    Some(Ok(val)) => {
                        self.fetching = false;
                        Some(val)
                    },
                    _ => {
                        self.fetching = false;
                        None
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.fetching {
            html! {
                <div class="loading">
                    {"Loading..."}
                </div>
            } 
        } else {
            html! {
                <div>
                    <p>{ 
                        if let Some(data) = &self.response {
                            data
                        } else {
                            "Failed to fetch"
                        }
                    }</p>
                </div>
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
