use crate::domain::rbac::Action;

pub trait Can<T> {
    fn can(&self, action: Action, resource: &T) -> bool;
    fn cannot(&self, action: Action, resource: &T) -> bool {
        !self.can(action, resource)
    }
}
