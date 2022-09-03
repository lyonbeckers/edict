use core::{any::TypeId, marker::PhantomData, ptr::NonNull};

use atomicell::borrow::AtomicBorrow;

use crate::{
    archetype::Archetype,
    entity::EntityId,
    epoch::EpochId,
    query::{Access, Fetch, ImmutableQuery, IntoQuery, Query, QueryFetch},
    relation::{OriginComponent, Relation},
};

/// Fetch for the [`FilterNotRelatesTo<R>`] query.
pub struct FetchFilterNotRelatesTo<'a, R: Relation> {
    kind: FetchKind<'a, R>,
}

enum FetchKind<'a, R: Relation> {
    /// Variant for entities without relation
    NotRelates,

    /// Variant for entities with relation
    Relates {
        target: EntityId,
        ptr: NonNull<OriginComponent<R>>,
        _borrow: AtomicBorrow<'a>,
        marker: PhantomData<&'a OriginComponent<R>>,
    },
}

use FetchKind::{NotRelates, Relates};

unsafe impl<'a, R> Fetch<'a> for FetchFilterNotRelatesTo<'a, R>
where
    R: Relation,
{
    type Item = ();

    #[inline]
    fn dangling() -> Self {
        FetchFilterNotRelatesTo { kind: NotRelates }
    }

    #[inline]
    unsafe fn skip_chunk(&mut self, _: usize) -> bool {
        false
    }

    #[inline]
    unsafe fn visit_chunk(&mut self, _: usize) {}

    #[inline]
    unsafe fn skip_item(&mut self, idx: usize) -> bool {
        match self.kind {
            NotRelates => false,
            Relates { ptr, target, .. } => {
                let origin_component = &*ptr.as_ptr().add(idx);
                origin_component
                    .origins()
                    .iter()
                    .all(|origin| origin.target != target)
            }
        }
    }

    #[inline]
    unsafe fn get_item(&mut self, _: usize) -> () {}
}

/// Filters out relation origin with specified targets.
/// Yields entities that are not relation origins and origins of other targets.
pub struct FilterNotRelatesTo<R> {
    target: EntityId,
    phantom: PhantomData<R>,
}

phantom_debug!(FilterNotRelatesTo<R> { target });

impl<R> FilterNotRelatesTo<R> {
    /// Returns relation filter bound to one specific target entity.
    pub const fn new(target: EntityId) -> Self {
        FilterNotRelatesTo {
            target,
            phantom: PhantomData,
        }
    }
}

impl<'a, R> QueryFetch<'a> for FilterNotRelatesTo<R>
where
    R: Relation,
{
    type Item = ();
    type Fetch = FetchFilterNotRelatesTo<'a, R>;
}

impl<R> IntoQuery for FilterNotRelatesTo<R>
where
    R: Relation,
{
    type Query = Self;
}

impl<R> Query for FilterNotRelatesTo<R>
where
    R: Relation,
{
    #[inline]
    fn access(&self, ty: TypeId) -> Option<Access> {
        if ty == TypeId::of::<OriginComponent<R>>() {
            Some(Access::Read)
        } else {
            None
        }
    }

    fn skip_archetype(&self, archetype: &Archetype) -> bool {
        !archetype.contains_id(TypeId::of::<OriginComponent<R>>())
    }

    #[inline]
    unsafe fn fetch<'a>(
        &mut self,
        archetype: &'a Archetype,
        _epoch: EpochId,
    ) -> FetchFilterNotRelatesTo<'a, R> {
        match archetype.id_index(TypeId::of::<OriginComponent<R>>()) {
            None => FetchFilterNotRelatesTo { kind: NotRelates },
            Some(idx) => {
                let component = archetype.component(idx);
                debug_assert_eq!(component.id(), TypeId::of::<OriginComponent<R>>());

                let (data, borrow) = atomicell::Ref::into_split(component.data.borrow());

                FetchFilterNotRelatesTo {
                    kind: Relates {
                        target: self.target,
                        ptr: data.ptr.cast(),
                        _borrow: borrow,
                        marker: PhantomData,
                    },
                }
            }
        }
    }
}

unsafe impl<R> ImmutableQuery for FilterNotRelatesTo<R> where R: Relation {}

/// Returns a filter to filter out origins of relation with specified target.
pub fn not_relates_to<R: Relation>(target: EntityId) -> FilterNotRelatesTo<R> {
    FilterNotRelatesTo {
        target,
        phantom: PhantomData,
    }
}