#![allow(unused_qualifications)]
#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use {
    std::sync::Arc,
    derivative::Derivative,
};

#[cfg(any(debug_assertions, feature = "always-log"))]
#[macro_export] macro_rules! lock {
    ($guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!($guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    ($guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            std::println!("[{ctx}] acquiring mutex guard");
            let mut guard_fut = std::pin::pin!($mutex.0.lock());
            let mut $guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!("[{ctx}] warning: acquiring mutex guard taking over a minute");
                    guard_fut.await
                }
            };
            std::println!("[{ctx}] acquired mutex guard");

            macro_rules! unlock {
                () => {
                    std::println!("[{ctx}] dropping mutex guard");
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!("[{ctx}] dropping mutex guard");
            drop($guard);
            value
        }
    }};
    (@blocking $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@blocking $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@blocking $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            std::println!("[{ctx}] synchronously acquiring mutex guard");
            let mut $guard = $mutex.0.blocking_lock();
            std::println!("[{ctx}] synchronously acquired mutex guard");

            macro_rules! unlock {
                () => {
                    std::println!("[{ctx}] dropping mutex guard");
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!("[{ctx}] dropping mutex guard");
            drop($guard);
            value
        }
    }};
    (@sync $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@sync $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@sync $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            std::println!("[{ctx}] acquiring parking_lot mutex guard");
            let mutex = &$mutex;
            let mut $guard = if let Some(guard) = mutex.0.try_lock_for(std::time::Duration::from_secs(60)) {
                guard
            } else {
                std::eprintln!("[{ctx}] warning: acquiring parking_lot mutex guard taking over a minute");
                mutex.0.lock()
            };
            std::println!("[{ctx}] acquired parking_lot mutex guard");

            macro_rules! unlock {
                () => {
                    std::println!("[{ctx}] dropping parking_lot mutex guard");
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!("[{ctx}] dropping parking_lot mutex guard");
            drop($guard);
            value
        }
    }};
    (@read $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@read $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@read $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            std::println!("[{ctx}] acquiring RwLock read guard");
            let mut guard_fut = std::pin::pin!($mutex.0.read());
            let mut $guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!("[{ctx}] warning: acquiring RwLock read guard taking over a minute");
                    guard_fut.await
                }
            };
            std::println!("[{ctx}] acquired RwLock read guard");

            macro_rules! unlock {
                () => {
                    std::println!("[{ctx}] dropping RwLock read guard");
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!("[{ctx}] dropping RwLock read guard");
            drop($guard);
            value
        }
    }};
    (@read blocking $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@read @blocking $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@read @blocking $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {
        $crate::lock!(@blocking @read $guard = $mutex; $ctx; $expr)
    };
    (@blocking @read $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@blocking @read $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@blocking @read $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            std::println!("[{ctx}] synchronously acquiring RwLock read guard");
            let mut $guard = $mutex.0.blocking_read();
            std::println!("[{ctx}] synchronously acquired RwLock read guard");

            macro_rules! unlock {
                () => {
                    std::println!("[{ctx}] dropping RwLock read guard");
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!("[{ctx}] dropping RwLock read guard");
            drop($guard);
            value
        }
    }};
    (@write $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@write $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@write $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            std::println!("[{ctx}] acquiring RwLock write guard");
            let mut guard_fut = std::pin::pin!($mutex.0.write());
            let mut $guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!("[{ctx}] warning: acquiring RwLock write guard taking over a minute");
                    guard_fut.await
                }
            };
            std::println!("[{ctx}] acquired RwLock write guard");

            macro_rules! unlock {
                () => {
                    std::println!("[{ctx}] dropping RwLock write guard");
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!("[{ctx}] dropping RwLock write guard");
            drop($guard);
            value
        }
    }};
    (@write @blocking $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@write @blocking $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@write @blocking $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {
        $crate::lock!(@blocking @write $guard = $mutex; $ctx; $expr)
    };
    (@blocking @write $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@blocking @write $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@blocking @write $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            std::println!("[{ctx}] synchronously acquiring RwLock write guard");
            let mut $guard = $mutex.0.blocking_write();
            std::println!("[{ctx}] synchronously acquired RwLock write guard");

            macro_rules! unlock {
                () => {
                    std::println!("[{ctx}] dropping RwLock write guard");
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!("[{ctx}] dropping RwLock write guard");
            drop($guard);
            value
        }
    }};
    (@owned @write $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@owned @write $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@owned @write $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {
        $crate::lock!(@write @owned $guard = $mutex; $ctx; $expr)
    };
    (@write @owned $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@write @owned $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@write @owned $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            std::println!("[{ctx}] acquiring owned RwLock write guard");
            let mut guard_fut = std::pin::pin!($mutex.0.write_owned());
            let mut $guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!("[{ctx}] warning: acquiring owned RwLock write guard taking over a minute");
                    guard_fut.await
                }
            };
            std::println!("[{ctx}] acquired owned RwLock write guard");

            macro_rules! unlock {
                () => {
                    std::println!("[{ctx}] dropping owned RwLock write guard");
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!("[{ctx}] dropping owned RwLock write guard");
            drop($guard);
            value
        }
    }};
}

#[cfg(not(any(debug_assertions, feature = "always-log")))]
#[macro_export] macro_rules! lock {
    ($guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!($guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    ($guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            let mut guard_fut = std::pin::pin!($mutex.0.lock());
            let mut $guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!("[{ctx}] warning: acquiring mutex guard taking over a minute");
                    guard_fut.await
                }
            };

            macro_rules! unlock {
                () => {
                    drop($guard);
                }
            }

            let value = $expr;
            drop($guard);
            value
        }
    }};
    (@blocking $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@blocking $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@blocking $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let _ = $ctx;
            let mut $guard = $mutex.0.blocking_lock();

            macro_rules! unlock {
                () => {
                    drop($guard);
                }
            }

            let value = $expr;
            drop($guard);
            value
        }
    }};
    (@sync $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@sync $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@sync $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            let mutex = &$mutex;
            let mut $guard = if let Some(guard) = mutex.0.try_lock_for(std::time::Duration::from_secs(60)) {
                guard
            } else {
                std::eprintln!("[{ctx}] warning: acquiring parking_lot mutex guard taking over a minute");
                mutex.0.lock()
            };

            macro_rules! unlock {
                () => {
                    drop($guard);
                }
            }

            let value = $expr;
            drop($guard);
            value
        }
    }};
    (@read $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@read $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@read $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            let mut guard_fut = std::pin::pin!($mutex.0.read());
            let mut $guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!("[{ctx}] warning: acquiring RwLock read guard taking over a minute");
                    guard_fut.await
                }
            };

            macro_rules! unlock {
                () => {
                    drop($guard);
                }
            }

            let value = $expr;
            drop($guard);
            value
        }
    }};
    (@read @blocking $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@read @blocking $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@read @blocking $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {
        $crate::lock!(@blocking @read $guard = $mutex; $ctx; $expr)
    };
    (@blocking @read $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@blocking @read $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@blocking @read $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let _ = $ctx;
            let mut $guard = $mutex.0.blocking_read();

            macro_rules! unlock {
                () => {
                    drop($guard);
                }
            }

            let value = $expr;
            drop($guard);
            value
        }
    }};
    (@write $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@write $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@write $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            let mut guard_fut = std::pin::pin!($mutex.0.write());
            let mut $guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!("[{ctx}] warning: acquiring RwLock write guard taking over a minute");
                    guard_fut.await
                }
            };

            macro_rules! unlock {
                () => {
                    drop($guard);
                }
            }

            let value = $expr;
            drop($guard);
            value
        }
    }};
    (@write @blocking $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@write @blocking $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@write @blocking $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {
        $crate::lock!(@blocking @write $guard = $mutex; $ctx; $expr)
    };
    (@blocking @write $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@blocking @write $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@blocking @write $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let _ = $ctx;
            let mut $guard = $mutex.0.blocking_write();

            macro_rules! unlock {
                () => {
                    drop($guard);
                }
            }

            let value = $expr;
            drop($guard);
            value
        }
    }};
    (@owned @write $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@owned @write $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@owned @write $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {
        $crate::lock!(@write @owned $guard = $mutex; $ctx; $expr)
    };
    (@write @owned $guard:ident = $mutex:expr; $expr:expr) => {
        $crate::lock!(@write @owned $guard = $mutex; format!("{} {}:{}", std::file!(), std::line!(), std::column!()); $expr)
    };
    (@write @owned $guard:ident = $mutex:expr; $ctx:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let ctx = $ctx;
            let mut guard_fut = std::pin::pin!($mutex.0.write_owned());
            let mut $guard = tokio::select! {
                guard = &mut guard_fut => guard,
                () = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                    std::eprintln!("[{ctx}] warning: acquiring owned RwLock write guard taking over a minute");
                    guard_fut.await
                }
            };

            macro_rules! unlock {
                () => {
                    drop($guard);
                }
            }

            let value = $expr;
            drop($guard);
            value
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

#[derive(Debug, Default)]
pub struct ParkingLotMutex<T: ?Sized>(pub parking_lot::Mutex<T>);

impl<T> ParkingLotMutex<T> {
    pub fn new(t: T) -> Self {
        Self(parking_lot::Mutex::new(t))
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}

#[derive(Debug, Default)]
pub struct RwLock<T: ?Sized>(pub tokio::sync::RwLock<T>);

impl<T> RwLock<T> {
    pub fn new(t: T) -> Self {
        Self(tokio::sync::RwLock::new(t))
    }
}

#[derive(Derivative, Debug, Default)]
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

#[derive(Derivative, Debug)]
#[derivative(Clone(bound = ""))]
pub struct RwFuture<T: Send + Sync + 'static>(pub RwFutureInner<T>);

impl<T: Send + Sync + 'static> RwFuture<T> {
    /// Creates a new `RwFuture` which will hold the output of the given future.
    pub fn new<F: Future<Output = T> + Send + 'static>(fut: F) -> Self {
        Self(RwFutureInner::new(fut))
    }
}

/// Helper type to make [`RwFuture`] work with the [`lock`] macro. Typically doesn't need to be interacted with directly.
#[derive(Derivative, Debug)]
#[derivative(Clone(bound = ""))]
pub struct RwFutureInner<T: Send + Sync + 'static>(Arc<tokio::sync::RwLock<RwFutureData<T>>>);

#[derive(Debug)]
enum RwFutureData<T: Send + Sync> {
    Pending(tokio::sync::broadcast::Sender<()>),
    Ready(T),
}

impl<T: Send + Sync> RwFutureData<T> {
    fn unwrap(&self) -> &T {
        match self {
            RwFutureData::Pending(_) => panic!("not ready"),
            RwFutureData::Ready(value) => value,
        }
    }

    fn unwrap_mut(&mut self) -> &mut T {
        match self {
            RwFutureData::Pending(_) => panic!("not ready"),
            RwFutureData::Ready(value) => value,
        }
    }
}

impl<T: Send + Sync + 'static> RwFutureInner<T> {
    fn new<F: Future<Output = T> + Send + 'static>(fut: F) -> Self {
        let (tx, _) = tokio::sync::broadcast::channel(1);
        let data = Arc::new(tokio::sync::RwLock::new(RwFutureData::Pending(tx.clone())));
        let data_clone = data.clone();
        tokio::spawn(async move {
            let value = fut.await;
            *data_clone.write().await = RwFutureData::Ready(value);
            let _ = tx.send(()); // an error just means no one's listening, which is fine
        });
        Self(data)
    }

    /// Waits until the value is available, then locks this `RwFuture` for read access.
    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, T> {
        let mut rx = {
            let data = self.0.read().await;
            match *data {
                RwFutureData::Pending(ref tx) => tx.subscribe(),
                RwFutureData::Ready(_) => return tokio::sync::RwLockReadGuard::map(data, RwFutureData::unwrap),
            }
        };
        let () = rx.recv().await.expect("RwFuture notifier dropped");
        let data = self.0.read().await;
        match *data {
            RwFutureData::Pending(_) => unreachable!("RwFuture should be ready after notification"),
            RwFutureData::Ready(_) => tokio::sync::RwLockReadGuard::map(data, RwFutureData::unwrap),
        }
    }

    /// Waits until the value is available, then locks this `RwFuture` for write access.
    pub async fn write(&self) -> tokio::sync::RwLockMappedWriteGuard<'_, T> {
        let mut rx = {
            let data = self.0.write().await;
            match *data {
                RwFutureData::Pending(ref tx) => tx.subscribe(),
                RwFutureData::Ready(_) => return tokio::sync::RwLockWriteGuard::map(data, RwFutureData::unwrap_mut),
            }
        };
        let () = rx.recv().await.expect("RwFuture notifier dropped");
        let data = self.0.write().await;
        match *data {
            RwFutureData::Pending(_) => unreachable!("RwFuture should be ready after notification"),
            RwFutureData::Ready(_) => tokio::sync::RwLockWriteGuard::map(data, RwFutureData::unwrap_mut),
        }
    }
}

impl<T: Send + Sync + Default> Default for RwFuture<T> {
    fn default() -> Self {
        Self(RwFutureInner(Arc::new(tokio::sync::RwLock::new(RwFutureData::Ready(T::default())))))
    }
}
