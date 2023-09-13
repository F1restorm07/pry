use std::{sync::OnceLock, collections::HashMap};

use downcast_rs::{Downcast, impl_downcast};

pub mod user;
pub mod index;

// inspired by tracing-subscriber
// currently the only implementer is Index
// will need to figure out how to expand in the future
// may also have an event trait as well
pub trait EventSubscriber: std::fmt::Debug + Send + Sync 
where
    Self: 'static
{
    // receive an event from a dispatcher and decide what to do with it
    fn receive_event(&self, event: &dyn Event);
}

pub trait Event: std::fmt::Debug + Downcast {
    fn dispatch(&self, route_name: &str) // is there a way to rid the Sized aspect while keeping functionality
    where
        Self: Sized
    {
        unsafe { CURRENT_DISPATCHER.get().unwrap().send_event(route_name, self); }
    }
}

// for downcasting into a concrete object
impl_downcast!(Event);

static mut CURRENT_DISPATCHER: OnceLock<EventDispatcher> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct EventDispatcher {
    routes: HashMap<String, &'static dyn EventSubscriber>
}

impl EventDispatcher {
    pub fn new() -> Self {
        // println!("[EventDispatcher] new dispatcher");
        unsafe { CURRENT_DISPATCHER.get_or_init(|| {
            Self { routes: HashMap::new() }
        }).clone() }
    }
    pub fn add_route(&self, name: &str, subscriber: &'static impl EventSubscriber) {
        unsafe { assert!(!CURRENT_DISPATCHER.get().unwrap().routes.contains_key(name)) }

        unsafe { CURRENT_DISPATCHER.get_mut().unwrap().routes.insert(name.to_string(), subscriber); }
    }
    pub fn send_event(&self, route_name: &str, event: &dyn Event) { self.routes.get(route_name).unwrap().receive_event(event); }
}
