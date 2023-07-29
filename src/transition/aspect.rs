use bevy::{prelude::Component, utils::tracing::Value};

pub trait Aspect<C : Component, V: Value>{
    fn get_value(component: &C)-> V;

    fn set_value(component: &mut C, value: V);
}

//TODO implement aspect for tuples of aspects