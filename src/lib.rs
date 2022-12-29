//!
//! ## Edict
//!
//! Edict is a fast and powerful ECS crate that expands traditional ECS feature set.
//! Written in Rust by your fellow 🦀
//!
//! ### Features
//!
//! * General purpose archetype based ECS with fast iteration.
//!
//! * Relations can be added to pair of entities, binding them together.
//!   When either of the two entities is despawned, relation is dropped.
//!   [`Relation`] type may further configure behavior of the bonds.
//!
//! * Change tracking.
//!   Each component instance is equipped with epoch counter that tracks last potential mutation of the component.
//!   Special query type uses epoch counter to skip entities where component wasn't changed since specified epoch.
//!   Last epoch can be obtained with [`World::epoch`].
//!
//! * Built-in type-map for singleton values called "resources".
//!   Resources can be inserted into/fetched from [`World`].
//!   Resources live separately from entities and their components.
//!
//! * Runtime checks for query validity and mutable aliasing avoidance.
//!   This requires atomic operations at the beginning iteration on next archetype.
//!
//! * Support for [`!Send`] and [`!Sync`] components.
//!   [`!Send`] components cannot be fetched mutably from outside "main" thread.
//!   [`!Sync`] components cannot be fetched immutably from outside "main" thread.
//!   [`World`] has to be [`!Send`] but implements [`Sync`].
//!
//! * [`ActionEncoder`] allows recording actions and later run them on [`World`].
//!   Actions get mutable access to [`World`].
//!
//! * Component replace/drop hooks.
//!   Components can define hooks that will be executed on value drop and replace.
//!   Hooks can read old and new values, [`EntityId`] and can record actions into [`ActionEncoder`].
//!
//! * Component type may define a set of types that can be borrowed from it.
//!   Borrowed type may be not sized, allowing slices, dyn traits and any other [`!Sized`] types.
//!   There's macro to define dyn trait borrows.
//!   Special kind of queries look into possible borrows to fetch.
//!
//! * [`WorldBuilder`] can be used to manually register component types and override default behavior.
//!
//! * Optional [`Component`] trait to allow implicit component type registration by insertion methods.
//!   Implicit registration uses behavior defined by [`Component`] implementation as-is.
//!   Separate insertions methods with [`Component`] trait bound lifted can be used where trait is not implemented or implementation is not visible for generic type.
//!   Those methods require pre-registration of the component type. If type was not registered - method panics.
//!   Both explicit registration with [`WorldBuilder`] and implicit registration via insertion method with [`Component`] type bound is enough.
//!
//! * [`System`] trait and [`IntoSystem`] implemented for functions if argument types implement [`FnArg`].
//!   This way practically any system can be defined as a function.
//!
//! * [`Scheduler`] that can run [`System`]s in parallel using provided executor.
//!
//! [`Send`]: core::marker::Send
//! [`!Send`]: core::marker::Send
//! [`Sync`]: core::marker::Sync
//! [`!Sync`]: core::marker::Sync
//! [`World`]: edict::world::World
//! [`WorldBuilder`]: edict::world::WorldBuilder
//! [`ActionEncoder`]: edict::action::ActionEncoder
//! [`EntityId`]: edict::entity::EntityId
//! [`!Sized`]: core::marker::Sized
//! [`Component`]: edict::component::Component
//! [`World::epoch`]: edict::world::World::epoch
//! [`Relation`]: edict::relation::Relation
//! [`System`]: edict::system::System
//! [`IntoSystem`]: edict::system::IntoSystem
//! [`FnArg`]: edict::system::FnArg
//! [`Scheduler`]: edict::scheduler::Scheduler

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

extern crate alloc;
extern crate self as edict;

pub use atomicell;

macro_rules! impl_copy {
    ($type:ident < $( $a:ident ),+ >) => {
        impl< $( $a ),+ > Copy for $type < $( $a ),+ > {}
        impl< $( $a ),+ > Clone for $type < $( $a ),+ > {
            fn clone(&self) -> Self {
                *self
            }
        }
    };
}

macro_rules! impl_debug {
    ($type:ident < $( $a:ident ),+ > { $($fname:ident)* }) => {
        impl< $( $a ),+ > core::fmt::Debug for $type < $( $a ),+ > {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!($type))
                    $(.field(stringify!($fname), &self.$fname))*
                    .finish()
            }
        }
    };
}

macro_rules! phantom_newtype {
    (
        $(#[$meta:meta])*
        $vis:vis struct $type:ident < $a:ident >
    ) => {
        $(#[$meta])*
        $vis struct $type < $a > {
            marker: core::marker::PhantomData< $a >,
        }

        impl< $a > $type < $a > {
            /// Constructs new phantom wrapper instance.
            /// This function is noop as it returns ZST and have no side-effects.
            pub const fn new() -> Self {
                $type {
                    marker: core::marker::PhantomData,
                }
            }
        }

        impl_copy!($type < $a >);

        impl< $a > core::fmt::Debug for $type < $a >
        where
            $a : 'static
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, core::concat!(core::stringify!($type), "<{}>"), core::any::type_name::<$a>())
            }
        }

        impl< $a > Default for $type < $a > {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

pub mod action;
pub mod archetype;
pub mod bundle;
pub mod component;
pub mod entity;
pub mod epoch;
pub mod executor;
pub mod prelude;
pub mod query;
pub mod relation;
pub mod scheduler;
pub mod system;
pub mod world;

mod hash;
mod idx;
mod res;

#[cfg(test)]
mod test;

#[doc(hidden)]
pub mod private {
    pub use alloc::vec::Vec;
}

#[doc(hidden)]
pub struct ExampleComponent;

impl component::Component for ExampleComponent {}

#[doc(inline)]
pub use self::prelude::*;
