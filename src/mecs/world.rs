use super::bimap::BiMap;
use super::Component;
use mecs::entity::Entity;
use std::collections::HashMap;

pub struct World {
    entities: HashMap<Entity, Vec<Box<dyn Component>>>,
    names: BiMap<String, Entity>,
    next_entity_id: usize,
}

impl World {
    pub fn new() -> World {
        World {
            entities: Default::default(),
            names: Default::default(),
            next_entity_id: Default::default(),
        }
    }
    
    pub fn entity(&mut self) -> Entity {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        
        let entity = Entity::from_id(id);
        self.entities.insert(entity,vec![]);
        
        entity
    }
    pub fn named_entity(&mut self, name: &str) -> Entity {
        match self.names.get_by_left(&name.to_owned()) {
            Some(&e) => e,
            None => {
                let e = self.entity();
                self.names.insert(name.into(), e);
                e
            }
        }
    }
    pub fn delete_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
        self.names.remove_by_right(&entity);
    }

    pub fn with_component<C,R,F>(&mut self, entity: Entity, mut fun: F) -> Option<R>
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
        me.entities.iter().filter_map(|(&k,_)|{
            if me.has_component::<C>(k) {
                Some(k)
            } else {
                None
            }
        }).collect()
    }
}
