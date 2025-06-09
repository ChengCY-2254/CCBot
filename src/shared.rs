#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
/// 线程安全的[Mutex]封装
pub struct DataBox<T>(Arc<Mutex<T>>);

impl<T> DataBox<T> {
    /// 创建一个新的[DataBox]
    pub fn new(data: T) -> Self {
        DataBox(Arc::new(Mutex::new(data)))
    }
}
unsafe impl<T> Send for DataBox<T> {}
unsafe impl<T> Sync for DataBox<T> {}

impl<T> Deref for DataBox<T> {
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// UpSafeCell 是一个线程安全的 RefCell 封装
#[derive(Debug, Default, Deserialize, Serialize)]
#[repr(transparent)]
pub struct UpSafeCell<T>(RefCell<T>);

unsafe impl<T> Sync for UpSafeCell<T> {}
unsafe impl<T> Send for UpSafeCell<T> {}

impl<T> UpSafeCell<T>
where
    T: Send + Sync,
{
    /// 创建一个新的 UpSafeCell
    /// ## Safety
    /// 自行保证安全使用，与[RefCell]一致，这只是一个警告。
    pub unsafe fn new(data: T) -> Self {
        Self(RefCell::new(data))
    }
    /// 获取一个可变引用
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
    /// 获取一个不可变引用
    pub fn access(&self) -> std::cell::Ref<'_, T> {
        self.0.borrow()
    }
}
