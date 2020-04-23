use super::bimap::BiMap;
use super::*;
use mecs::entity::Entity;
use std::collections::HashMap;

#[derive(Default)]
pub struct StaticWorld {
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

    pub fn with_component<C, R, F>(&mut self, entity: Entity, mut fun: F) -> Option<R>
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
    pub fn add_component<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        if let Some(components) = self.entities.get_mut(&entity) {
            components.push(Box::new(component));
        }
    }
    pub fn entities_with_component<C>(&mut self) -> Vec<Entity>
    where
        C: Component,
    {
        let me = &*self;
        me.entities
            .iter()
            .filter_map(|(&k, _)| {
                if me.has_component::<C>(k) {
                    Some(k)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn send<T,M>(&mut self, target: T, msg: M)
    where
        T: Into<MessageTarget>,
        M: Message,
    {
        self.send_annotated((target, msg).into());
    }
    
    pub fn send_annotated(&mut self, msg: AnnotatedMessage)
    {
        self.queued_messages.push(msg);
    }
}
