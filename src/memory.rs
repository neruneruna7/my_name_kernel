use core::future::IntoFuture;

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

/// 与えられたページを フレーム 0xb8000 に試しにマップする（VGAバッファ）
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: テストのためにのみ行う
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map to failed").flush();
}

/// つねにNoneを返す
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}

/// ブートローダのメモリマップから使用可能なフレームを返す
/// FrameAllocator
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// 渡されたメモリマップからFrameAllocatorを作る
    ///
    /// unsafe
    /// 呼び出し元は渡されたメモリマップが有効なことを保証しなくてはならない
    /// 特に， USABLEなフレームは実際に未使用でなくてはならない（使用可能という意味）
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        // カーネルが動いている間生きていないといけないデータだからstaticがつく
        // 他のinit関数も同様
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// メモリマップによって指定されたusableなフレームのイテレータを返す
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // メモリマップからusableな領域を得る
        let regions = self.memory_map.iter();
        // このカーネルのコード，データ，スタック領域等はInUseとしてマークされている
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        // それぞれの領域をアドレス範囲にmaで変換する
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());

        // フレームの開始アドレスのイテレータへと変換する
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by((4096)));

        // 開始アドレスから PhysFrame 型をつくる
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/// 新しいOffsetPageTableを初期化する
///
/// この関数はunsafeであり，また1度しか呼び出してはならない
/// 全物理メモリが渡された phisical_memory_offsetだけずらしたうえで
/// 仮想メモリでマップされていることを，呼び出しもとが保証しなければならない
/// また，&mut参照が複数の名称を持つことにつながるので，この関数は1度しか呼び出してはならない
/// (mutable aliasingというらしい 動作が未定義)
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// 有効なレベル4テーブルへの可変参照を返す
///
/// この関数はunsafeであり，また1度しか呼び出してはならない
/// 全物理メモリが渡された phisical_memory_offsetだけずらしたうえで
/// 仮想メモリでマップされていることを，呼び出しもとが保証しなければならない
/// また，&mut参照が複数の名称を持つことにつながるので，この関数は1度しか呼び出してはならない
/// (mutable aliasingというらしい 動作が未定義)
unsafe fn active_level4_table(phisycal_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_page_table_frame, _) = Cr3::read();

    let phys = level_4_page_table_frame.start_address();
    let virt = phisycal_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

// ライブラリの実装を使っているため，もう以下の実装は不要
// /// 与えられた仮想アドレスを対応する物理アドレスに変換し，
// /// そのアドレスがマップされていないなら None を返す
// ///
// /// この関数はunsafe 呼び出し元は，全物理メモリが与えられた
// /// physical_memory_offset だけずらしたうえでマップされていることを
// /// 保証しなくてはならないから
// pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     // unsafeの範囲を制限するため
//     // 内部にプライベートな本当の関数を入れることで，unsafeの範囲を明確にし，コードの危険性を下げる
//     translate_addr_inner(addr, physical_memory_offset)
// }

// fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     use x86_64::registers::control::Cr3;
//     use x86_64::structures::paging::page_table::FrameError;

//     let (level_4_table_flame, _) = Cr3::read();

//     let table_indexes = [
//         addr.p4_index(),
//         addr.p3_index(),
//         addr.p2_index(),
//         addr.p1_index(),
//     ];
//     let mut frame = level_4_table_flame;

//     // 複数層のページテーブルをたどる
//     for &index in &table_indexes {
//         // フレームをページテーブルの参照に変換する
//         let virt = physical_memory_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable = virt.as_ptr();
//         let table = unsafe { &*table_ptr };

//         // ページテーブルエントリを読んで，flameを更新する
//         let entry = &table[index];
//         frame = match entry.frame() {
//             Ok(frame) => frame,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
//         };
//     }

//     // ページオフセットを足すことで，目的の物理アドレスを計算する
//     Some(frame.start_address() + u64::from(addr.page_offset()))
// }
