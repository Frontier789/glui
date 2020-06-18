use mecs::{System, SystemId};
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct SystemSet {
    systems_of_type: HashMap<TypeId, HashMap<SystemId, Box<dyn System>>>,
    system_number: usize,
}

impl SystemSet {
    pub fn new() -> SystemSet {
        Default::default()
    }
    pub fn system<S>(&self, id: SystemId) -> Option<&S>
    where
        S: System + 'static,
    {
        self.system_boxed(id)
            .map(|sys| sys.downcast_ref::<S>())
            .flatten()
    }
    pub fn system_mut<S>(&mut self, id: SystemId) -> Option<&mut S>
    where
        S: System + 'static,
    {
        self.system_boxed_mut(id)
            .map(|sys| sys.downcast_mut::<S>())
            .flatten()
    }
    pub fn system_boxed(&self, id: SystemId) -> Option<&Box<dyn System>> {
        self.systems_of_type
            .get(&id.type_id)
            .map(|systems| systems.get(&id))
            .flatten()
    }
    pub fn system_boxed_mut(&mut self, id: SystemId) -> Option<&mut Box<dyn System>> {
        self.systems_of_type
            .get_mut(&id.type_id)
            .map(|systems| systems.get_mut(&id))
            .flatten()
    }
    pub fn first_of_type_mut(&mut self, sys_type: TypeId) -> Option<&mut Box<dyn System>> {
        let syses = self.systems_of_type.get_mut(&sys_type);
        syses
            .map(|s| s.iter_mut().next().map(|(_key, sys)| sys))
            .flatten()
    }
    fn systems_of_type_mut<S>(&mut self) -> &mut HashMap<SystemId, Box<dyn System>>
    where
        S: System,
    {
        let type_id = TypeId::of::<S>();
        if let None = self.systems_of_type.get_mut(&type_id) {
            self.systems_of_type.insert(type_id, HashMap::new());
        }

        self.systems_of_type.get_mut(&type_id).unwrap()
    }

    pub fn add_system<S>(&mut self, system: S) -> SystemId
    where
        S: System + 'static,
    {
        let id = SystemId::from_number::<S>(self.next_system_number());
        self.systems_of_type_mut::<S>().insert(id, Box::new(system));
        id
    }
    fn next_system_number(&mut self) -> usize {
        let id = self.system_number;
        self.system_number += 1;
        id
    }
    pub fn del_system(&mut self, id: SystemId) {
        if let Some(systems) = self.systems_of_type.get_mut(&id.type_id) {
            systems.remove(&id);
        }
    }
    pub fn all_systems_mut<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (&SystemId, &'a mut Box<dyn System>)> {
        self.systems_of_type
            .iter_mut()
            .map(|(_, systems)| systems.iter_mut())
            .flatten()
    }
}
