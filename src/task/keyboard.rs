use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use crate::println;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

/// キーボード割り込みハンドラから呼び出される
///
/// 処理をブロックしたり，アロケートしてはいけない
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            //  スキャンコードキューがいっぱいでキーボード入力を取りこぼしている
            println!("WARNING: scancode queue full; dropping keyboard input");
        }
    } else {
        //  スキャンコードキューが初期化されていない
        println!("WARNING: scancode queue uninitialized");
    }
}
