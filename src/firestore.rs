use std::cell::RefCell;
use std::borrow::BorrowMut;
use crate::cmn::Hub;
use crate::ProjectMethods;
use std::mem;

pub struct Firestore<C, A> {
    pub(crate) client: RefCell<C>,
    pub(crate) auth: RefCell<A>,
    pub(crate) _user_agent: String,
    pub(crate) _base_url: String,
    pub(crate) _root_url: String,
}

impl<'a, C, A> Hub for Firestore<C, A> {}

impl<'a, C, A> Firestore<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Firestore<C, A> {
        Firestore {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.11".to_string(),
            _base_url: "https://firestore.googleapis.com/".to_string(),
            _root_url: "https://firestore.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.11`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://firestore.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://firestore.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
