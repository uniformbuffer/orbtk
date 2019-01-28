use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
    properties::{Constraint, GridColumn, HorizontalAlignment, Margin, VerticalAlignment},
    systems::LayoutResult,
};

pub use self::grid::GridLayout;
pub use self::fixed_size::FixedSizeLayout;


mod grid;
mod fixed_size;

pub trait Layout {
    fn measure(&self, entity: Entity, ecm: &mut EntityComponentManager, tree: &Tree, layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>) -> (f64, f64);
    fn arrange(&self, parent_size: (f64, f64), entity: Entity, ecm: &mut EntityComponentManager, tree: &Tree, layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>) -> (f64, f64);
}

// --- helpers ---

pub fn get_vertical_alignment(entity: Entity, ecm: &EntityComponentManager) -> VerticalAlignment {
    if let Ok(vertical_alignment) = ecm.borrow_component::<VerticalAlignment>(entity) {
        return *vertical_alignment;
    }

    VerticalAlignment::default()
}

pub fn get_horizontal_alignment(
    entity: Entity,
    ecm: &EntityComponentManager,
) -> HorizontalAlignment {
    if let Ok(horizontal_alignment) = ecm.borrow_component::<HorizontalAlignment>(entity) {
        return *horizontal_alignment;
    }

    HorizontalAlignment::default()
}

pub fn get_margin(entity: Entity, ecm: &EntityComponentManager) -> Margin {
    if let Ok(margin) = ecm.borrow_component::<Margin>(entity) {
        return *margin;
    }

    Margin::default()
}

pub fn get_constraint(entity: Entity, ecm: &EntityComponentManager) -> Constraint {
    if let Ok(constraint) = ecm.borrow_component::<Constraint>(entity) {
        return *constraint;
    }

    Constraint::default()
}

// todo provide helpers for basic properties get_.. borrow_.. borrow_mut..

// --- helpers ---