#![allow(unused_qualifications)]
#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use {
    std::{
        ops::{
            Deref,
            DerefMut,
        },
        sync::Arc,
    },
    derivative::Derivative,
};

#[macro_export] macro_rules! lock {
    ($mutex:expr) => {{
        #[allow(unused_qualifications)] {
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] acquiring mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut guard_fut = std::pin::pin!($mutex.0.lock());
            let guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!(
                        "[{} {}:{}] warning: acquiring mutex guard taking over a minute",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    guard_fut.await
                }
            };
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] mutex guard acquired",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            $crate::MutexGuard(guard)
        }
    }};
    (@blocking $mutex:expr) => {{
        #[allow(unused_qualifications)] {
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] synchronously acquiring mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let guard = $mutex.0.blocking_lock();
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] mutex guard acquired synchronously",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            $crate::MutexGuard(guard)
        }
    }};
    (@read $rw_lock:expr) => {{
        #[allow(unused_qualifications)] {
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] acquiring RwLock read guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut guard_fut = std::pin::pin!($rw_lock.0.read());
            let guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!(
                        "[{} {}:{}] warning: acquiring RwLock read guard taking over a minute",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    guard_fut.await
                }
            };
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] RwLock read guard acquired",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            $crate::RwLockReadGuard(guard)
        }
    }};
    (@read @blocking $rw_lock:expr) => {
        $crate::lock!(@blocking @read $rw_lock)
    };
    (@blocking @read $rw_lock:expr) => {{
        #[allow(unused_qualifications)] {
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] synchronously acquiring RwLock read guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let guard = $rw_lock.0.blocking_read();
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] RwLock read guard acquired synchronously",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            $crate::RwLockReadGuard(guard)
        }
    }};
    (@write $rw_lock:expr) => {{
        #[allow(unused_qualifications)] {
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] acquiring RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut guard_fut = std::pin::pin!($rw_lock.0.write());
            let guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!(
                        "[{} {}:{}] warning: acquiring RwLock write guard taking over a minute",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    guard_fut.await
                }
            };
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] RwLock write guard acquired",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            $crate::RwLockWriteGuard(guard)
        }
    }};
    (@write @blocking $rw_lock:expr) => {
        $crate::lock!(@blocking @write $rw_lock)
    };
    (@blocking @write $rw_lock:expr) => {{
        #[allow(unused_qualifications)] {
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] synchronously acquiring RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let guard = $rw_lock.0.blocking_write();
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] RwLock write guard acquired synchronously",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            $crate::RwLockWriteGuard(guard)
        }
    }};
    (@owned @write $rw_lock:expr) => {
        $crate::lock!(@write @owned $rw_lock)
    };
    (@write @owned $rw_lock:expr) => {{
        #[allow(unused_qualifications)] {
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] acquiring owned RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut guard_fut = std::pin::pin!($rw_lock.0.write_owned());
            let guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!(
                        "[{} {}:{}] warning: acquiring owned RwLock write guard taking over a minute",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    guard_fut.await
                }
            };
            #[cfg(debug_assertions)] std::println!(
                "[{} {}:{}] owned RwLock write guard acquired",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            $crate::OwnedRwLockWriteGuard(guard)
        }
    }};
}

#[derive(Debug, Default)]
pub struct Mutex<T: ?Sized>(pub tokio::sync::Mutex<T>);

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self(tokio::sync::Mutex::new(t))
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}

pub struct MutexGuard<'a, T: ?Sized>(pub tokio::sync::MutexGuard<'a, T>);

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T { &self.0 }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}

#[cfg(debug_assertions)] impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        println!("dropping mutex guard");
    }
}

pub struct RwLock<T: ?Sized>(pub tokio::sync::RwLock<T>);

impl<T> RwLock<T> {
    pub fn new(t: T) -> Self {
        Self(tokio::sync::RwLock::new(t))
    }
}

pub struct RwLockReadGuard<'a, T: ?Sized>(pub tokio::sync::RwLockReadGuard<'a, T>);

impl<T: ?Sized> Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T { &self.0 }
}

#[cfg(debug_assertions)] impl<T: ?Sized> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        println!("dropping RwLock read guard");
    }
}

pub struct RwLockWriteGuard<'a, T: ?Sized>(pub tokio::sync::RwLockWriteGuard<'a, T>);

impl<T: ?Sized> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T { &self.0 }
}

impl<T: ?Sized> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}

#[cfg(debug_assertions)] impl<T: ?Sized> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        println!("dropping RwLock write guard");
    }
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct ArcRwLock<T: ?Sized>(pub Arc<tokio::sync::RwLock<T>>);

impl<T> ArcRwLock<T> {
    pub fn new(t: T) -> Self {
        Self(Arc::new(tokio::sync::RwLock::new(t)))
    }
}

impl<T> From<Arc<tokio::sync::RwLock<T>>> for ArcRwLock<T> {
    fn from(value: Arc<tokio::sync::RwLock<T>>) -> Self {
        Self(value)
    }
}

pub struct OwnedRwLockWriteGuard<T: ?Sized>(pub tokio::sync::OwnedRwLockWriteGuard<T>);

impl<T: ?Sized> Deref for OwnedRwLockWriteGuard<T> {
    type Target = T;

    fn deref(&self) -> &T { &self.0 }
}

impl<T: ?Sized> DerefMut for OwnedRwLockWriteGuard<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}

#[cfg(debug_assertions)] impl<T: ?Sized> Drop for OwnedRwLockWriteGuard<T> {
    fn drop(&mut self) {
        println!("dropping owned RwLock write guard");
    }
}
