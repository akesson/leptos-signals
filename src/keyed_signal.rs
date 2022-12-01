use std::{fmt::Debug, future::Future, pin::Pin, rc::Rc};

use leptos::{
    create_effect, create_memo, create_signal, spawn_local, Memo, ReadSignal, Scope, WriteSignal,
};

/// A KeyedSignal associates a key with a value.
/// This is typically used for caching, where a key is needed
/// for searching in the cache.
///
/// Only when the key changes, when it's not equal to previous value,
/// the lookup function is called.
///
/// The value needs to implement Default, if it happens to be something
/// that doesn't have a Default implementation, then wrap it in an Option
/// since it implements a Default value of None.
pub struct KeyedSignal<Key, Val>
where
    Key: 'static + PartialEq + Clone + Debug,
    Val: 'static + Default,
{
    inner: KeyedSignalInner<Key, Val>,
    value: ReadSignal<Val>,
}

impl<Key, Val> KeyedSignal<Key, Val>
where
    Key: 'static + PartialEq + Clone + Debug,
    Val: 'static + Clone + Default,
{
    pub fn get(&self) -> Val {
        self.value.get()
    }
    pub fn get_key(&self) -> Key {
        self.inner.key.get()
    }

    pub fn key(&self) -> Memo<Key> {
        self.inner.key
    }
}

pub fn create_keyed_signal<Key, Val, F, Fu>(
    cx: Scope,
    key: impl Fn() -> Key + 'static,
    lookup: F,
) -> KeyedSignal<Key, Val>
where
    Key: 'static + PartialEq + Clone + Debug,
    Val: 'static + Default,
    F: Fn(Key, WriteSignal<Val>) -> Fu + 'static,
    Fu: Future<Output = ()> + 'static,
{
    let (value, inner) = create_key_signal_inner(cx, key, lookup);
    let act = inner.action_fn.clone();
    create_effect(cx, move |_| {
        let key = inner.key.get();
        let fut = (act)(key, inner.set_value);
        spawn_local(async move { fut.await })
    });
    KeyedSignal { inner, value }
}

struct KeyedSignalInner<Key, Val>
where
    Key: 'static + PartialEq + Clone + Debug,
    Val: 'static + Default,
{
    key: Memo<Key>,
    set_value: WriteSignal<Val>,

    action_fn: Rc<dyn Fn(Key, WriteSignal<Val>) -> Pin<Box<dyn Future<Output = ()>>>>,
}

fn create_key_signal_inner<Key, Val, F, Fu>(
    cx: Scope,
    key: impl Fn() -> Key + 'static,
    lookup: F,
) -> (ReadSignal<Val>, KeyedSignalInner<Key, Val>)
where
    Key: 'static + PartialEq + Clone + Debug,
    Val: 'static + Default,
    F: Fn(Key, WriteSignal<Val>) -> Fu + 'static,
    Fu: Future<Output = ()> + 'static,
{
    let key = create_memo(cx, move |_| key());
    let (value, set_value) = create_signal(cx, Val::default());

    let action_fn = Rc::new(move |input: Key, value: WriteSignal<Val>| {
        let input = input.clone();
        let fut = lookup(input, value);
        Box::pin(async move { fut.await }) as Pin<Box<dyn Future<Output = ()>>>
    });

    (
        value,
        KeyedSignalInner {
            key,
            set_value,
            action_fn,
        },
    )
}
