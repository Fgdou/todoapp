use yew::{Reducible, UseReducerHandle};

use crate::models::users::User;

#[derive(Clone, PartialEq, Default)]
pub struct Context {
    pub user: Option<User>
}

pub enum ActionContext {
    SetUser(User),
    ClearUser,
}

impl Reducible for Context {
    type Action = ActionContext;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            ActionContext::SetUser(user) => Self {
                user: Some(user),
            }.into(),
            ActionContext::ClearUser => Self {
                user: None,
            }.into(),
        }
    }
}

pub type AppContext = UseReducerHandle<Context>;
