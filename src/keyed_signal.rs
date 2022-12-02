use std::{fmt::Debug, future::Future, pin::Pin, rc::Rc};

use leptos_reactive::{
    create_effect, create_memo, create_rw_signal, spawn_local, Memo, ReadSignal, RwSignal, Scope,
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
    F: Fn(Key, RwSignal<Val>) -> Fu + 'static,
    Fu: Future<Output = ()> + 'static,
{
    let (value, inner) = create_key_signal_inner(cx, key, lookup);
    let act = inner.action_fn.clone();
    create_effect(cx, move |_| {
        let key = inner.key.get();
        let fut = (act)(key, inner.value);
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
    value: RwSignal<Val>,

    action_fn: Rc<dyn Fn(Key, RwSignal<Val>) -> Pin<Box<dyn Future<Output = ()>>>>,
}

fn create_key_signal_inner<Key, Val, F, Fu>(
    cx: Scope,
    key: impl Fn() -> Key + 'static,
    lookup: F,
) -> (ReadSignal<Val>, KeyedSignalInner<Key, Val>)
where
    Key: 'static + PartialEq + Clone + Debug,
    Val: 'static + Default,
    F: Fn(Key, RwSignal<Val>) -> Fu + 'static,
    Fu: Future<Output = ()> + 'static,
{
    let key = create_memo(cx, move |_| key());
    let value = create_rw_signal(cx, Val::default());

    let action_fn = Rc::new(move |input: Key, value: RwSignal<Val>| {
        let input = input.clone();
        let fut = lookup(input, value);
        Box::pin(async move { fut.await }) as Pin<Box<dyn Future<Output = ()>>>
    });

    (
        value.read_only(),
        KeyedSignalInner {
            key,
            value,
            action_fn,
        },
    )
}
