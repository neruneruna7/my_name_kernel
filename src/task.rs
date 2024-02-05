pub mod keyboard;
pub mod simple_executor;

use alloc::boxed::Box;
use core::task::{Context, Poll};
use core::{future::Future, pin::Pin};

pub struct Task {
    // 各async fnは異なる型を持っている
    // それに対応するため dynによる動的ディスパッチを使う
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        // 返されたTaskが任意の時間だけ生き続けることができるため，
        // futureもその時間だけ有効である必要がある
        Self {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
