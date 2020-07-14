use super::bimap::BiMap;
use super::component::*;
use super::message::*;
use mecs::entity::Entity;
use mecs::{GlutinElementState, GlutinKey, System};
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct StaticWorld {
    pub(super) key_states: HashMap<GlutinKey, GlutinElementState>,
    entities: HashMap<Entity, Vec<Box<dyn Component>>>,
    entity_names: BiMap<String, Entity>,
    next_entity_id: usize,
    pub(super) queued_messages: Vec<AnnotatedMessage>,
}

impl StaticWorld {
    pub fn entity(&mut self) -> Entity {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        let entity = Entity::from_id(id);
        self.entities.insert(entity, vec![]);
        entity
    }
    pub fn named_entity(&mut self, name: &str) -> Entity {
        match self.entity_names.get_by_left(&name.to_owned()) {
            Some(&e) => e,
            None => {
                let e = self.entity();
                self.entity_names.insert(name.into(), e);
                e
            }
        }
    }
    pub fn delete_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
        self.entity_names.remove_by_right(&entity);
    }
    pub fn entities(&self) -> Vec<Entity> {
        self.entities.keys().into_iter().map(|e| *e).collect()
    }

    pub fn with_component<C, R, F>(&self, entity: Entity, mut fun: F) -> Option<R>
    where
        C: Component,
        F: FnMut(&C) -> R,
    {
        if let Some(components) = self.entities.get(&entity) {
            for c in components {
                if c.is::<C>() {
                    return Some(fun(c.downcast_ref::<C>().unwrap()));
                }
            }
        }
        None
    }
    pub fn with_component_mut<C, R, F>(&mut self, entity: Entity, mut fun: F) -> Option<R>
    where
        C: Component,
        F: FnMut(&mut C) -> R,
    {
        if let Some(components) = self.entities.get_mut(&entity) {
            for c in components {
                if c.is::<C>() {
                    return Some(fun(c.downcast_mut::<C>().unwrap()));
                }
            }
        }
        None
    }
    pub fn has_component<C>(&self, entity: Entity) -> bool
    where
        C: Component,
    {
        if let Some(components) = self.entities.get(&entity) {
            for c in components {
                if c.is::<C>() {
                    return true;
                }
            }
        }
        false
    }
    pub fn component<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        if let Some(components) = self.entities.get(&entity) {
            for c in components {
                if let Some(comp) = c.downcast_ref() {
                    return Some(comp);
                }
            }
        }
        None
    }
    pub fn component_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        if let Some(components) = self.entities.get_mut(&entity) {
            for c in components {
                if let Some(comp) = c.downcast_mut() {
                    return Some(comp);
                }
            }
        }
        None
    }
    pub fn add_component<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        if let Some(components) = self.entities.get_mut(&entity) {
            components.push(Box::new(component));
        }
    }
    pub fn new_entity_with_component<C>(&mut self, component: C) -> Entity
    where
        C: Component,
    {
        let entity = self.entity();
        if let Some(components) = self.entities.get_mut(&entity) {
            components.push(Box::new(component));
        }
        entity
    }
    pub fn entities_with_component<C>(&self) -> Vec<(Entity, &C)>
    where
        C: Component,
    {
        let me = &*self;
        me.entities
            .iter()
            .filter_map(|(&entity, _)| me.component(entity).map(|comp| (entity, comp)))
            .collect()
    }
    pub fn entities_with_component_mut<C>(&mut self) -> Vec<(Entity, &mut C)>
    where
        C: Component,
    {
        self.entities
            .iter_mut()
            .filter_map(|(entity, components)| {
                for c in components {
                    if let Some(comp) = c.downcast_mut() {
                        return Some((*entity, comp));
                    }
                }
                None
            })
            .collect()
    }

    pub fn send<T, M>(&mut self, target: T, msg: M)
    where
        T: Into<MessageTarget>,
        M: Message,
    {
        self.send_annotated((target, msg).into());
    }

    pub fn send_root<M>(&mut self, msg: M)
    where
        M: Message,
    {
        self.send_annotated((MessageTarget::Root, msg).into());
    }

    pub fn send_by_type<S, M>(&mut self, msg: M)
    where
        S: System,
        M: Message,
    {
        self.send_annotated((MessageTarget::SystemOfType(TypeId::of::<S>()), msg).into());
    }

    pub fn send_annotated(&mut self, msg: AnnotatedMessage) {
        self.queued_messages.push(msg);
    }

    pub fn broadcast<M>(&mut self, msg: M)
    where
        M: Message,
    {
        self.send_annotated((MessageTarget::Broadcast, msg).into());
    }

    pub fn is_key_pressed(&self, key: GlutinKey) -> bool {
        if let Some(GlutinElementState::Pressed) = self.key_states.get(&key) {
            true
        } else {
            false
        }
    }
}
