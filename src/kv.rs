use std::fmt::Debug;
use std::ptr::null_mut;

use crate::{Node, NodePtr};

pub trait Key: Ord + Clone + Debug {}
impl<T: Ord + Clone + Debug> Key for T {}

pub trait Value: Clone {}
impl<T: Clone> Value for T {}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Color {
    BLACK = 0,
    RED = 1
}

pub struct KeyValue<K: Key, V: Value> {
    left: KeyValuePtr<K, V>,
    right: KeyValuePtr<K, V>,
    color: Color,
    key: K,
    value: V,
}

impl<K: Key, V: Value> KeyValue<K, V> {
    pub fn new(key: K, value: V) -> KeyValue<K, V> {
        KeyValue {
            left: KeyValuePtr::<K, V>::NIL,
            right: KeyValuePtr::<K, V>::NIL,
            color: Color::RED,
            key: key,
            value: value
        }
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

impl<K: Key, V: Value> Node for KeyValue<K, V> {
    type Key = K;
    type Ptr = KeyValuePtr<K, V>;

    fn new(data: &Self) -> Self::Ptr {
        use std::alloc::{alloc, Layout};

        unsafe {
            let layout = Layout::new::<Self>();
            let ptr = KeyValuePtr(alloc(layout) as *mut Self);
            let node = &mut (*ptr.0);
            node.left = KeyValuePtr::NIL;
            node.right = KeyValuePtr::NIL;
            node.color = Color::RED;
            node.key.clone_from(data.key());
            node.value.clone_from(data.value());
            return ptr;
        }
    }

    fn key(&self) -> &Self::Key {
        &self.key
    }

    fn left(&self) -> &Self::Ptr {
        &self.left
    }

    fn left_mut(&mut self) -> &mut Self::Ptr {
        &mut self.left
    }

    fn right(&self) -> &Self::Ptr {
        &self.right
    }

    fn right_mut(&mut self) -> &mut Self::Ptr {
        &mut self.right
    }

    fn update(&mut self, data: &Self) {
        self.value.clone_from(&data.value);
    }

    fn is_black(&self) -> bool {
        self.color == Color::BLACK
    }

    fn set_black(&mut self) {
        self.color = Color::BLACK
    }

    fn set_red(&mut self) {
        self.color = Color::RED
    }
}

pub struct KeyValuePtr<K: Key, V: Value>(*mut KeyValue<K, V>);

impl<K: Key, V: Value> Clone for KeyValuePtr<K, V> {
    fn clone(&self) -> Self {
        KeyValuePtr(self.0)
    }
}

impl<K: Key, V: Value> Copy for KeyValuePtr<K, V> {}

impl<K: Key, V: Value> NodePtr<KeyValue<K, V>> for KeyValuePtr<K, V> {
    const NIL: Self = KeyValuePtr(null_mut());

    fn is_nil(&self) -> bool {
        self.0.is_null()
    }

    fn node<'a>(&self) -> &'a KeyValue<K, V> {
        unsafe { &*self.0 }
    }

    fn node_mut<'a>(&self) -> &'a mut KeyValue<K, V> {
        unsafe { &mut *self.0 }
    }
}
