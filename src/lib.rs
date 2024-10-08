#![allow(unused_qualifications)]
#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use {
    std::sync::Arc,
    derivative::Derivative,
};

#[cfg(any(debug_assertions, feature = "always-log"))]
#[macro_export] macro_rules! lock {
    ($guard:ident = $mutex:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            std::println!(
                "[{} {}:{}] acquiring mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut guard_fut = std::pin::pin!($mutex.0.lock());
            let mut $guard = tokio::select! {
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
            std::println!(
                "[{} {}:{}] acquired mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );

            macro_rules! unlock {
                () => {
                    std::println!(
                        "[{} {}:{}] dropping mutex guard",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!(
                "[{} {}:{}] dropping mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            drop($guard);
            value
        }
    }};
    (@blocking $guard:ident = $mutex:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            std::println!(
                "[{} {}:{}] synchronously acquiring mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut $guard = $mutex.0.blocking_lock();
            std::println!(
                "[{} {}:{}] synchronously acquired mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );

            macro_rules! unlock {
                () => {
                    std::println!(
                        "[{} {}:{}] dropping mutex guard",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!(
                "[{} {}:{}] dropping mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            drop($guard);
            value
        }
    }};
    (@sync $guard:ident = $mutex:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            std::println!(
                "[{} {}:{}] acquiring parking_lot mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mutex = &$mutex;
            let mut $guard = if let Some(guard) = mutex.0.try_lock_for(std::time::Duration::from_secs(60)) {
                guard
            } else {
                std::eprintln!(
                    "[{} {}:{}] warning: acquiring parking_lot mutex guard taking over a minute",
                    std::file!(),
                    std::line!(),
                    std::column!(),
                );
                mutex.0.lock()
            };
            std::println!(
                "[{} {}:{}] acquired parking_lot mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );

            macro_rules! unlock {
                () => {
                    std::println!(
                        "[{} {}:{}] dropping parking_lot mutex guard",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!(
                "[{} {}:{}] dropping parking_lot mutex guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            drop($guard);
            value
        }
    }};
    (@read $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            std::println!(
                "[{} {}:{}] acquiring RwLock read guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut guard_fut = std::pin::pin!($rw_lock.0.read());
            let mut $guard = tokio::select! {
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
            std::println!(
                "[{} {}:{}] acquired RwLock read guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );

            macro_rules! unlock {
                () => {
                    std::println!(
                        "[{} {}:{}] dropping RwLock read guard",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!(
                "[{} {}:{}] dropping RwLock read guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            drop($guard);
            value
        }
    }};
    (@read @blocking $guard:ident = $rw_lock:expr; $expr:expr) => {
        $crate::lock!(@blocking @read $guard = $rw_lock; $expr)
    };
    (@blocking @read $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            std::println!(
                "[{} {}:{}] synchronously acquiring RwLock read guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut $guard = $rw_lock.0.blocking_read();
            std::println!(
                "[{} {}:{}] synchronously acquired RwLock read guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );

            macro_rules! unlock {
                () => {
                    std::println!(
                        "[{} {}:{}] dropping RwLock read guard",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!(
                "[{} {}:{}] dropping RwLock read guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            drop($guard);
            value
        }
    }};
    (@write $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            std::println!(
                "[{} {}:{}] acquiring RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut guard_fut = std::pin::pin!($rw_lock.0.write());
            let mut $guard = tokio::select! {
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
            std::println!(
                "[{} {}:{}] acquired RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );

            macro_rules! unlock {
                () => {
                    std::println!(
                        "[{} {}:{}] dropping RwLock write guard",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!(
                "[{} {}:{}] dropping RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            drop($guard);
            value
        }
    }};
    (@write @blocking $guard:ident = $rw_lock:expr; $expr:expr) => {
        $crate::lock!(@blocking @write $guard = $rw_lock; $expr)
    };
    (@blocking @write $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            std::println!(
                "[{} {}:{}] synchronously acquiring RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut $guard = $rw_lock.0.blocking_write();
            std::println!(
                "[{} {}:{}] synchronously acquired RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );

            macro_rules! unlock {
                () => {
                    std::println!(
                        "[{} {}:{}] dropping RwLock write guard",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!(
                "[{} {}:{}] dropping RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            drop($guard);
            value
        }
    }};
    (@owned @write $guard:ident = $rw_lock:expr; $expr:expr) => {
        $crate::lock!(@write @owned $guard = $rw_lock; $expr)
    };
    (@write @owned $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            std::println!(
                "[{} {}:{}] acquiring owned RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            let mut guard_fut = std::pin::pin!($rw_lock.0.write_owned());
            let mut $guard = tokio::select! {
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
            std::println!(
                "[{} {}:{}] acquired owned RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );

            macro_rules! unlock {
                () => {
                    std::println!(
                        "[{} {}:{}] dropping owned RwLock write guard",
                        std::file!(),
                        std::line!(),
                        std::column!(),
                    );
                    drop($guard);
                }
            }

            let value = $expr;
            std::println!(
                "[{} {}:{}] dropping owned RwLock write guard",
                std::file!(),
                std::line!(),
                std::column!(),
            );
            drop($guard);
            value
        }
    }};
}

#[cfg(not(any(debug_assertions, feature = "always-log")))]
#[macro_export] macro_rules! lock {
    ($guard:ident = $mutex:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let mut guard_fut = std::pin::pin!($mutex.0.lock());
            let mut $guard = tokio::select! {
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
    (@blocking $guard:ident = $mutex:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
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
    (@sync $guard:ident = $mutex:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let mutex = &$mutex;
            let mut $guard = if let Some(guard) = mutex.0.try_lock_for(std::time::Duration::from_secs(60)) {
                guard
            } else {
                std::eprintln!(
                    "[{} {}:{}] warning: acquiring parking_lot mutex guard taking over a minute",
                    std::file!(),
                    std::line!(),
                    std::column!(),
                );
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
    (@read $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let mut guard_fut = std::pin::pin!($rw_lock.0.read());
            let mut $guard = tokio::select! {
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
    (@read @blocking $guard:ident = $rw_lock:expr; $expr:expr) => {
        $crate::lock!(@blocking @read $guard = $rw_lock; $expr)
    };
    (@blocking @read $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let mut $guard = $rw_lock.0.blocking_read();

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
    (@write $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let mut guard_fut = std::pin::pin!($rw_lock.0.write());
            let mut $guard = tokio::select! {
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
    (@write @blocking $guard:ident = $rw_lock:expr; $expr:expr) => {
        $crate::lock!(@blocking @write $guard = $rw_lock; $expr)
    };
    (@blocking @write $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let mut $guard = $rw_lock.0.blocking_write();

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
    (@owned @write $guard:ident = $rw_lock:expr; $expr:expr) => {
        $crate::lock!(@write @owned $guard = $rw_lock; $expr)
    };
    (@write @owned $guard:ident = $rw_lock:expr; $expr:expr) => {{
        #[allow(unused_macros, unused_mut, unused_qualifications)] {
            let mut guard_fut = std::pin::pin!($rw_lock.0.write_owned());
            let mut $guard = tokio::select! {
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
