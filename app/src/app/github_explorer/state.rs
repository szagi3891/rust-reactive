use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};

use vertigo::{
    DomDriver,
    FetchMethod,
    computed::{
        Dependencies,
        Value,
        AutoMap,
        Computed,
    }
};

#[derive(PartialEq)]
pub enum Resource<T: PartialEq> {
    Loading,
    Ready(T),
    Failed(String),
}

#[derive(PartialEq)]
pub struct State {
    pub repo_input: Value<String>,
    pub repo_shown: Value<String>,
    pub data: AutoMap<String, Resource<Branch>>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let root_inner = root.clone();
        let driver_inner = driver.clone();

        root.new_computed_from(State {
            repo_input: root.new_value("".into()),
            repo_shown: root.new_value("".into()),
            data: AutoMap::new(move |repo_name: &String| -> Computed<Resource<Branch>> {
                log::info!("Creating for {}", repo_name);
                let new_value = root_inner.new_value(Resource::Loading);
                let new_computed = new_value.to_computed();
    
                fetch_repo(repo_name.as_str(), new_value, &driver_inner);
    
                new_computed
            }),
        })
    }
}


fn fetch_repo(repo: &str, value: Value<Resource<Branch>>, driver: &DomDriver) {
    let driver_span = driver.clone();
    let url = format!("https://api.github.com/repos/{}/branches/master", repo);
    log::info!("Fetching1 {}", url);

    driver.spawn_local(async move {
        log::info!("Fetching2 {}", url);
        let response = driver_span.fetch(FetchMethod::GET, url, None, None).await;

        match response {
            Ok(response) => {
                match serde_json::from_str::<Branch>(response.as_str()) {
                    Ok(branch) => {
                        log::info!("odpowiedź z serwera {:?}", branch);
                        value.set_value(Resource::Ready(branch));
                    },
                    Err(err) => {
                        log::error!("Error parsing response: {}", err);
                        value.set_value(Resource::Failed(err.to_string()));
                    }
                }
            },
            Err(_) => {
                log::error!("Error fetch branch")
            }
        }
    });
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}
