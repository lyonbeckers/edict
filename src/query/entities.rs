use core::{any::TypeId, marker::PhantomData};

use crate::{archetype::Archetype, entity::EntityId};

use super::{Access, Fetch, ImmutablePhantomQuery, PhantomQuery};

/// [`Fetch`] type for the [`Entities`] query.
pub struct EntitiesFetch<'a> {
    entities: &'a [EntityId],
}

unsafe impl<'a> Fetch<'a> for EntitiesFetch<'a> {
    type Item = EntityId;

    #[inline]
    fn dangling() -> Self {
        EntitiesFetch { entities: &[] }
    }

    #[inline]
    unsafe fn get_item(&mut self, idx: usize) -> EntityId {
        *self.entities.get_unchecked(idx)
    }
}

/// Queries entity ids.
#[derive(Clone, Copy, Debug, Default)]
pub struct Entities;

/// Query type for the [`Entities`] phantom query.
pub type EntitiesQuery = PhantomData<fn() -> Entities>;

impl Entities {
    /// Creates a new [`Entities`] query.
    pub fn query() -> PhantomData<fn() -> Self> {
        PhantomQuery::query()
    }
}

unsafe impl PhantomQuery for Entities {
    type Fetch<'a> = EntitiesFetch<'a>;
    type Item<'a> = EntityId;

    #[inline]
    fn access(_ty: TypeId) -> Option<Access> {
        None
    }

    #[inline]
    fn visit_archetype(_archetype: &Archetype) -> bool {
        true
    }

    #[inline]
    unsafe fn access_archetype(_archetype: &Archetype, _f: &dyn Fn(TypeId, Access)) {}

    #[inline]
    unsafe fn fetch<'a>(
        archetype: &'a Archetype,
        _epoch: crate::epoch::EpochId,
    ) -> EntitiesFetch<'a> {
        EntitiesFetch {
            entities: archetype.entities(),
        }
    }

    #[inline]
    fn reserved_entity_item<'a>(id: EntityId) -> Option<EntityId>
    where
        EntityId: 'a,
    {
        Some(id)
    }
}

unsafe impl ImmutablePhantomQuery for Entities {}
