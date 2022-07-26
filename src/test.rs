use crate::{
    prelude::Component,
    query::Modified,
    world::{EntityError, World},
};

use alloc::{vec, vec::Vec};

#[derive(Debug, PartialEq, Eq)]
struct Str(&'static str);
impl Component for Str {}

#[derive(Debug, PartialEq, Eq)]
struct U32(u32);
impl Component for U32 {}

#[derive(Debug, PartialEq, Eq)]
struct Bool(bool);
impl Component for Bool {}

/// Tests that entity spawned into world has all components from bundle.
#[test]
fn world_spawn() {
    let mut world = World::new();

    let e = world.spawn((U32(42), Str("qwe")));
    assert_eq!(world.has_component::<U32>(&e), Ok(true));
    assert_eq!(world.has_component::<&Str>(&e), Ok(true));
    assert_eq!(
        world.query_one_mut::<(&U32, &Str)>(&e),
        Ok((&U32(42), &Str("qwe")))
    );
}

/// Tests that entity does not have a component that wasn't in spawn bundle
/// but has it after component is inserted
#[test]
fn world_insert() {
    let mut world = World::new();

    let e = world.spawn((U32(42),));
    assert_eq!(world.has_component::<U32>(&e), Ok(true));
    assert_eq!(world.has_component::<Str>(&e), Ok(false));
    assert_eq!(
        world.query_one_mut::<(&U32, &Str)>(&e),
        Err(EntityError::MissingComponents)
    );

    assert_eq!(world.try_insert(&e, Str("qwe")), Ok(()));
    assert_eq!(world.has_component::<Str>(&e), Ok(true));
    assert_eq!(
        world.query_one_mut::<(&U32, &Str)>(&e),
        Ok((&U32(42), &Str("qwe")))
    );
}

/// Tests that entity does not have a component that was removed.
#[test]
fn world_remove() {
    let mut world = World::new();

    let e = world.spawn((U32(42), Str("qwe")));
    assert_eq!(world.has_component::<U32>(&e), Ok(true));
    assert_eq!(world.has_component::<Str>(&e), Ok(true));
    assert_eq!(
        world.query_one_mut::<(&U32, &Str)>(&e),
        Ok((&U32(42), &Str("qwe")))
    );

    assert_eq!(world.remove::<Str>(&e), Ok(Str("qwe")));
    assert_eq!(world.has_component::<Str>(&e), Ok(false));
    assert_eq!(
        world.query_one_mut::<(&U32, &Str)>(&e),
        Err(EntityError::MissingComponents)
    );
}

/// Insertion test. Bundle version
#[test]
fn world_insert_bundle() {
    let mut world = World::new();

    let e = world.spawn((U32(42),));
    assert_eq!(world.has_component::<U32>(&e), Ok(true));
    assert_eq!(world.has_component::<Str>(&e), Ok(false));
    assert_eq!(
        world.query_one_mut::<(&U32, &Str)>(&e),
        Err(EntityError::MissingComponents)
    );

    assert_eq!(
        world.try_insert_bundle(&e, (Str("qwe"), Bool(true))),
        Ok(())
    );
    assert_eq!(world.has_component::<Str>(&e), Ok(true));
    assert_eq!(world.has_component::<Bool>(&e), Ok(true));
    assert_eq!(
        world.query_one_mut::<(&U32, &Str, &Bool)>(&e),
        Ok((&U32(42), &Str("qwe"), &Bool(true)))
    );
}

/// Removing test. Bundle version.
#[test]
fn world_remove_bundle() {
    let mut world = World::new();

    let e = world.spawn((U32(42), Str("qwe")));
    assert_eq!(world.has_component::<U32>(&e), Ok(true));
    assert_eq!(world.has_component::<Str>(&e), Ok(true));
    assert_eq!(
        world.query_one_mut::<(&U32, &Str)>(&e),
        Ok((&U32(42), &Str("qwe")))
    );

    // When removing a bundle, any missing component is simply ignored.
    assert_eq!(world.drop_bundle::<(Str, Bool)>(&e), Ok(()));
    assert_eq!(world.has_component::<Str>(&e), Ok(false));
    assert_eq!(
        world.query_one_mut::<(&U32, &Str)>(&e),
        Err(EntityError::MissingComponents)
    );
}

#[test]
fn version_test() {
    let mut world = World::new();

    let mut tracks = world.tracks();
    let e = world.spawn((U32(42), Str("qwe")));

    assert_eq!(
        world
            .query::<Modified<&U32>>()
            .tracked_iter(&mut tracks)
            .collect::<Vec<_>>(),
        vec![(e, &U32(42))]
    );

    assert_eq!(
        world
            .query::<Modified<&U32>>()
            .tracked_iter(&mut tracks)
            .collect::<Vec<_>>(),
        vec![]
    );

    *world.query_one_mut::<&mut U32>(&e).unwrap() = U32(42);

    assert_eq!(
        world
            .query::<Modified<&U32>>()
            .tracked_iter(&mut tracks)
            .collect::<Vec<_>>(),
        vec![(e, &U32(42))]
    );
}

#[test]
fn version_despawn_test() {
    let mut world = World::new();

    let mut tracks = world.tracks();
    let e1 = world.spawn((U32(42), Str("qwe")));
    let e2 = world.spawn((U32(23), Str("rty")));

    assert_eq!(
        world
            .query::<Modified<&U32>>()
            .tracked_iter(&mut tracks)
            .collect::<Vec<_>>(),
        vec![(e1, &U32(42)), (e2, &U32(23))]
    );

    assert_eq!(
        world
            .query::<Modified<&U32>>()
            .tracked_iter(&mut tracks)
            .collect::<Vec<_>>(),
        vec![]
    );

    *world.query_one_mut::<&mut U32>(&e2).unwrap() = U32(50);
    assert_eq!(world.despawn(&e1), Ok(()));

    assert_eq!(
        world
            .query::<Modified<&U32>>()
            .tracked_iter(&mut tracks)
            .collect::<Vec<_>>(),
        vec![(e2, &U32(50))]
    );
}

#[test]
fn version_insert_test() {
    let mut world = World::new();

    let mut tracks = world.tracks();
    let e1 = world.spawn((U32(42), Str("qwe")));
    let e2 = world.spawn((U32(23), Str("rty")));

    assert_eq!(
        world
            .query::<Modified<&U32>>()
            .tracked_iter(&mut tracks)
            .collect::<Vec<_>>(),
        vec![(e1, &U32(42)), (e2, &U32(23))]
    );

    assert_eq!(
        world
            .query::<Modified<&U32>>()
            .tracked_iter(&mut tracks)
            .collect::<Vec<_>>(),
        vec![]
    );

    *world.query_one_mut::<&mut U32>(&e1).unwrap() = U32(50);
    *world.query_one_mut::<&mut U32>(&e2).unwrap() = U32(100);

    assert_eq!(world.try_insert(&e1, Bool(true)), Ok(()));

    assert_eq!(
        world
            .query::<Modified<&U32>>()
            .tracked_iter(&mut tracks)
            .collect::<Vec<_>>(),
        vec![(e2, &U32(100)), (e1, &U32(50))]
    );
}