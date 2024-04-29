/*!

# System Independent Timer

Async timer that works on wasm (with gloo) and linux/macos/windows (with tokio)

```rust
use std::time::Duration;

async {
    sint::Timeout::new(Duration::from_millis(500)).await;

    //Or equivalent
    sint::sleep(Duration::from_millis(500)).await;
};
```

*/

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

#[repr(transparent)]
#[pin_project::pin_project]
pub struct Timeout {
    #[cfg(not(target_family = "wasm"))]
    #[pin]
    backend: tokio::time::Sleep,

    #[cfg(target_family = "wasm")]
    #[pin]
    backend: gloo_timers::future::TimeoutFuture,
}
impl Timeout {
    #[inline(always)]
    pub fn new(timeout: Duration) -> Self {
        Timeout {
            #[cfg(not(target_family = "wasm"))]
            backend: tokio::time::sleep(timeout),

            #[cfg(target_family = "wasm")]
            backend: gloo_timers::future::TimeoutFuture::new(timeout.as_millis() as u32),
        }
    }

    #[inline(always)]
    pub fn until(instant: Instant) -> Self {
        Timeout {
            #[cfg(not(target_family = "wasm"))]
            backend: tokio::time::sleep_until(instant.into()),

            #[cfg(target_family = "wasm")]
            backend: gloo_timers::future::TimeoutFuture::new(
                instant.duration_since(Instant::now()).as_millis() as u32,
            ),
        }
    }
}

impl Future for Timeout {
    type Output = ();

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().backend.poll(cx)
    }
}

#[inline(always)]
pub fn sleep(timeout: Duration) -> Timeout {
    Timeout::new(timeout)
}

#[inline(always)]
pub fn sleep_until(instant: Instant) -> Timeout {
    Timeout::until(instant)
}
