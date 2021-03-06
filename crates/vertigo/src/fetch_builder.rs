use std::{
    collections::HashMap,
    rc::Rc,
};

use crate::{DomDriverTrait, FetchMethod, driver::show_log, utils::{EqBox}};


pub struct FetchBuilder {
    driver: EqBox<Rc<dyn DomDriverTrait>>,
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>
}

impl FetchBuilder {
    pub fn new(driver: EqBox<Rc<dyn DomDriverTrait>>, url: String) -> FetchBuilder {
        FetchBuilder {
            driver,
            url,
            headers: None,
            body: None,
        }
    }

    pub fn set_headres(self, headers: HashMap<String, String>) -> Self {
        let FetchBuilder { driver, url, body , .. } = self;
        FetchBuilder {
            driver,
            url,
            headers: Some(headers),
            body,
        }
    }

    pub fn set_body(self, body: String) -> Self {
        let FetchBuilder { driver, url, headers , .. } = self;
        FetchBuilder {
            driver,
            url,
            headers,
            body: Some(body),
        }
    }

    async fn run(self, method: FetchMethod) -> Result<String, String> {
        show_log(format!("fetch {:?} {}", method, &self.url));
        let fut = self.driver.fetch(method, self.url, self. headers, self.body);
        fut.await
    }

    pub async fn get(self) -> Result<String, String> {
        self.run(FetchMethod::GET).await
    }

    pub async fn post(self) -> Result<String, String> {
        self.run(FetchMethod::POST).await
    }
}
