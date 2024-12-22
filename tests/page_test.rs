#[cfg(test)]
mod tests {
    use pgids::Pgids;
    use quickcheck::quickcheck;
    // use pgids::Pgids;
    use rbolt::page::*;

    #[test]
    fn test_page_typ() {
        // 测试 "branch" 类型
        let page_branch = Page {
            id: 1,
            flags: BRANCH_PAGE_FLAG,
            count: 0,
            overflow: 0,
            data: Box::new([]),
        };
        assert_eq!(page_branch.typ(), "branch");

        // 测试 "leaf" 类型
        let page_leaf = Page {
            id: 2,
            flags: LEAF_PAGE_FLAG,
            count: 0,
            overflow: 0,
            data: Box::new([]),
        };
        assert_eq!(page_leaf.typ(), "leaf");

        // 测试 "meta" 类型
        let page_meta = Page {
            id: 3,
            flags: META_PAGE_FLAG,
            count: 0,
            overflow: 0,
            data: Box::new([]),
        };
        assert_eq!(page_meta.typ(), "meta");

        // 测试 "freelist" 类型
        let page_freelist = Page {
            id: 4,
            flags: FREELIST_PAGE_FLAG,
            count: 0,
            overflow: 0,
            data: Box::new([]),
        };
        assert_eq!(page_freelist.typ(), "freelist");

        // 测试未知类型
        let page_unknown = Page {
            id: 5,
            flags: 0x4E20, // 20000 十六进制表示
            count: 0,
            overflow: 0,
            data: Box::new([]),
        };
        assert_eq!(page_unknown.typ(), "unknown<4e20>");
    }

    /// 测试 Page.hexdump 方法，确保它不会导致崩溃。
    #[test]
    fn test_page_hexdump() {
        let page = Page {
            id: 256,
            flags: 0,
            count: 0,
            overflow: 0,
            data: vec![0u8; 16].into_boxed_slice(),
        };
        // 这里我们只需要确保调用 hexdump 不会 panic
        page.hexdump(16);
    }

    /// 测试 Pgids::merge 方法，确保合并结果正确。
    #[test]
    fn test_pgids_merge() {
        let a = Pgids(vec![4, 5, 6, 10, 11, 12, 13, 27]);
        let b = Pgids(vec![1, 3, 8, 9, 25, 30]);
        let c = a.merge(&b);
        assert_eq!(
            c,
            Pgids(vec![1, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 25, 27, 30])
        );

        let a = Pgids(vec![4, 5, 6, 10, 11, 12, 13, 27, 35, 36]);
        let b = Pgids(vec![8, 9, 25, 30]);
        let c = a.merge(&b);
        assert_eq!(
            c,
            Pgids(vec![4, 5, 6, 8, 9, 10, 11, 12, 13, 25, 27, 30, 35, 36])
        );
    }

    /// 属性测试 Pgids::merge 方法，确保合并的结果与预期一致。
    #[test]
    fn test_pgids_merge_quick() {
        fn prop(a: Pgids, b: Pgids) -> bool {
            // 确保输入的 Pgids 是有序的
            let mut a_sorted = a.clone();
            a_sorted.sort();
            let mut b_sorted = b.clone();
            b_sorted.sort();

            // 合并两个 Pgids
            let got = a_sorted.merge(&b_sorted);

            // 预期结果是两个有序的 Vec 合并后再次排序
            let mut exp = a_sorted.0.clone();
            exp.extend(&b_sorted.0);
            exp.sort();
            let exp_pgids = Pgids(exp);

            // 比较结果
            exp_pgids == got
        }

        // 使用 quickcheck 进行属性测试
        quickcheck(prop as fn(Pgids, Pgids) -> bool);
    }

    /// 测试 Pgids 的 Display 实现
    #[test]
    fn test_pgids_display() {
        let pgids = Pgids(vec![1, 2, 3]);
        assert_eq!(format!("{}", pgids), "Pgids([1, 2, 3])");
    }
}
