use core::{fmt, marker::PhantomData, ops::Deref};

use crate::{bundle::Bundle, world::World};

use super::{strong::StrongEntity, weak::WeakEntity};

/// Strong reference to an entity.
/// This value can be used to access an entity and keeps the entity alive.
/// On access to a component, if entity doesn't have accessed component,
/// an error is returned.
///
/// This type is generic over list of pinned components.
/// Pinned components cannot be removed and thus they can be accessed with guarantee.
#[derive(Clone, PartialEq, Eq)]
pub struct Entity<T = ()> {
    pub(super) strong: StrongEntity,
    pub(super) marker: PhantomData<fn() -> T>,
}

impl Entity {
    pub(crate) fn with_bundle<B>(self) -> Entity<B>
    where
        B: Bundle,
    {
        Entity {
            strong: self.strong,
            marker: PhantomData,
        }
    }
}

impl<T> Entity<T> {
    /// Returns [`WeakEntity`] value that references same entity.
    pub fn downgrade(&self) -> WeakEntity {
        **self
    }
}

impl<T> fmt::Debug for Entity<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity")
            .field("gen", &self.gen.get())
            .field("id", &self.id)
            .finish()
    }
}

impl<T> fmt::Display for Entity<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.strong.weak, f)
    }
}

impl<T> Deref for Entity<T> {
    type Target = WeakEntity;

    fn deref(&self) -> &WeakEntity {
        &self.strong.weak
    }
}

macro_rules! for_tuple {
    () => {
        for_tuple!(for A B C D E F);
    };

    (for) => {
        for_tuple!(impl);
    };

    (for $head:ident $($tail:ident)*) => {
        for_tuple!(for $($tail)*);
        for_tuple!(impl $head $($tail)*);
    };

    (impl) => {
        impl Entity<()> {
            for_tuple!(fn );
        }
    };

    (impl $($a:ident)+) => {
        impl<$($a),+> Entity<($($a,)+)> {
            for_tuple!(fn $($a)+);
        }
    };

    (fn $($a:ident)*) => {
            /// Pins another component to the Entity.
            /// Pinned components of the entity can be fetched from `World` without failure cases.
            /// `World::get` and `World::get_mut` functions return references to pinned components as-is,
            /// while other must be wrapped in `Option`.
            ///
            /// This function recreates `Entity` object with different type pameter.
            ///
            /// At the time of writing, this function does not actually make it impossible to remove pinned component.
            /// If pinned components are removed, `World::get` and `World::get_mut` would panic
            /// unless removed components are skipped.
            ///
            /// # Panics
            ///
            /// This function panics if entity does not have pinned component.
            ///
            /// # Example
            ///
            /// ```
            /// # use edict::prelude::World;
            /// # let mut world = World::new();
            /// let entity = world.spawn((0u32,));
            /// let entity = entity.pin::<u32>(&mut world);
            /// ```
            ///
            /// # Example
            ///
            /// ```should_panic
            /// # use edict::prelude::World;
            /// # let mut world = World::new();
            /// let entity = world.spawn((0u32,));
            /// let entity = entity.pin::<u8>(&mut world);
            /// ```
            pub fn pin<T: 'static>(self, world: &mut World) -> Entity<($($a,)* T,)> {
                assert!(world.has_component::<T, _>(&self));

                drop(world);
                Entity {
                    strong: self.strong,
                    marker: PhantomData,
                }
            }
    };
}

for_tuple!();
