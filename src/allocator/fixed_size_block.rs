use super::Locked;
use alloc::alloc::GlobalAlloc;
use core::{alloc::Layout, mem, ptr::NonNull};

/// 使用するブロックサイズ
///
/// 2の累乗である必要がある
/// 2の累乗でなければならないブロックのアラインメントとしても使われるから
const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

struct ListNode {
    next: Option<&'static mut ListNode>,
}

pub struct FixedSizeBlockAllcator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    failback_allocator: linked_list_allocator::Heap,
}

impl FixedSizeBlockAllcator {
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        Self {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            failback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    /// アロケータを与えられたヒープ境界で初期化する
    ///
    /// unsafe
    /// 呼び出し元は与えるヒープ境界が有効であり，
    /// ヒープが未使用であることを保証しなければならない
    /// このメソッドは1度しか呼ばれてはならない
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.failback_allocator.init(heap_start, heap_size);
    }

    /// 代替アロケータを使って割り当てを行う
    fn failbac_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.failback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => core::ptr::null_mut(),
        }
    }
}

/// 与えられたレイアウトに対して適切なブロックsize選ぶ
fn list_index(layout: &Layout) -> Option<usize> {
    let requierd_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= requierd_block_size)
}

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllcator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        match list_index(&layout) {
            Some(index) => {
                match allocator.list_heads[index].take() {
                    Some(node) => {
                        allocator.list_heads[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    }
                    None => {
                        // リストにブロックがない -> 新しくブロックを割り当てる
                        let block_size = BLOCK_SIZES[index];
                        // すべてのブロックサイズが2の累乗のときだけ正しく動く
                        // 今はblock_sizeもalignも同じ
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align).unwrap();
                        allocator.failbac_alloc(layout)
                    }
                }
            }
            None => allocator.failbac_alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.lock();

        match list_index(&layout) {
            Some(index) => {
                let new_node = ListNode {
                    next: allocator.list_heads[index].take(),
                };
                // ブロックがノードを格納できるサイズとアラインメントをもっていることを確認
                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);
                let new_node_ptr = ptr as *mut ListNode;
                new_node_ptr.write(new_node);
                allocator.list_heads[index] = Some(&mut *new_node_ptr);
            }
            None => {
                let ptr = NonNull::new(ptr).unwrap();
                allocator.failback_allocator.deallocate(ptr, layout);
            }
        }
    }
}
