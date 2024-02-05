use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

pub struct BumpAllcator {
    heap_start: usize,
    heap_end: usize,
    // 次の割り当て開始アドレスを指すのがnext
    next: usize,
    allocations: usize,
}

impl BumpAllcator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// 与えられたヒープ領域でパンプアロケータを初期化する
    ///
    /// このメソッドはunsafe
    /// 呼び出し元は与えられたメモリ範囲が未使用であることを保証しなければならない
    /// また，1度しか呼ばれてはならない
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        // 最初はヒープ全体が未使用なので，nextはheap_start
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllcator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        // 割り当てがヒープメモリの領域内にあることを保証するために境界チェック（アライン）している
        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {
            // メモリ不足
            ptr::null_mut()
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
