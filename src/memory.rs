use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

/// 有効なレベル4テーブルへの可変参照を返す
///
/// この関数はunsafeであり，また1度しか呼び出してはならない
/// 全物理メモリが渡された phisical_memory_offsetだけずらしたうえで
/// 仮想メモリでマップされていることを，呼び出しもとが保証しなければならない
/// また，&mut参照が複数の名称を持つことにつながるので，この関数は1度しか呼び出してはならない
/// (mutable aliasingというらしい 動作が未定義)
pub unsafe fn active_level4_table(phisycal_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_page_table_frame, _) = Cr3::read();

    let phys = level_4_page_table_frame.start_address();
    let virt = phisycal_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

/// 与えられた仮想アドレスを対応する物理アドレスに変換し，
/// そのアドレスがマップされていないなら None を返す
///
/// この関数はunsafe 呼び出し元は，全物理メモリが与えられた
/// physical_memory_offset だけずらしたうえでマップされていることを
/// 保証しなくてはならないから
pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    // unsafeの範囲を制限するため
    // 内部にプライベートな本当の関数を入れることで，unsafeの範囲を明確にし，コードの危険性を下げる
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    let (level_4_table_flame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_flame;

    // 複数層のページテーブルをたどる
    for &index in &table_indexes {
        // フレームをページテーブルの参照に変換する
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        // ページテーブルエントリを読んで，flameを更新する
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // ページオフセットを足すことで，目的の物理アドレスを計算する
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
