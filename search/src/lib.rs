// #[test]
// 插入排序: $O(n^2) $
pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        // 外層迴圈疊代整個序列。並取出 index i，arr[i] 是待排序的元素，
        // index 比 i 小的元素則組成已排序的部分序列
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

// 插入排序: $O(n^2) $
// 使用二分查找减少内圈比较次数
pub fn binary_insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let val = arr[i];
        let mut j = i;
        // 先限制 binary_search 範圍，取出 sorted pile arr[..i]。再對 slice 執行 binary_search
        let pos = match arr[..i].binary_search(&val) {
            // binary_search 回傳一個 Result<usize, usize> 型別，
            // 找到時回傳 Ok(index 值)，找無時回傳 Err(不影響排序穩定度的插入點)，
            // 這個 Err 的設計巧妙解決新值插入的問題
            Ok(pos) => pos, // 2
            Err(pos) => pos,
        };
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
