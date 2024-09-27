pub struct SegTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    n: i32,
    default: T,
    cell: Vec<T>,
    lazy: Vec<T>,
    op: F,
}

impl<T, F> SegTree<T, F>
where
    T: Clone + Copy + std::cmp::PartialEq,
    F: Fn(T, T) -> T,
{
    /// Constructs a new Segment Tree with default values. The tree can store a range of [1, n].
    ///
    /// ### Usage
    /// ```rust
    /// use cp_lib::ds::range::seg_tree::SegTree;
    ///
    /// let mut tree = SegTree::new(10, i32::MIN, std::cmp::max);
    /// tree.insert(1, 10);
    /// tree.insert(2, 20);
    /// assert_eq!(tree.query(1, 10), 20);
    ///
    /// tree.insert_range(3, 6, 10);
    /// assert_eq!(tree.query(5, 10), 10);
    /// ```
    pub fn new(n: usize, default: T, op: F) -> Self {
        Self {
            n: n as i32,
            default,
            cell: vec![default; 4 * n],
            lazy: vec![default; 4 * n],
            op,
        }
    }

    fn _push(&mut self, n: usize) {
        self.cell[n] = (self.op)(self.cell[n], self.lazy[n]);
        if (n << 1) < self.lazy.len() {
            self.lazy[n << 1] = (self.op)(self.lazy[n << 1], self.lazy[n]);
            self.lazy[n << 1 | 1] = (self.op)(self.lazy[n << 1 | 1], self.lazy[n]);
        }
        self.lazy[n] = self.default;
    }

    fn _insert(&mut self, n: usize, start: i32, end: i32, l: i32, r: i32, v: T) {
        if start > end || r < start || l > end {
            return;
        } else if start >= l && end <= r {
            self.lazy[n] = (self.op)(self.lazy[n], v);
            self._push(n);
            return;
        }
        if self.lazy[n] != self.default {
            self._push(n);
        }
        let mid = (start + end) >> 1;
        self._insert(n << 1, start, mid, l, r, v);
        self._insert(n << 1 | 1, mid + 1, end, l, r, v);
        self.cell[n] = (self.op)(self.cell[n << 1], self.cell[n << 1 | 1]);
    }

    fn _query(&mut self, n: usize, start: i32, end: i32, l: i32, r: i32) -> T {
        if start > end || start > r || end < l {
            return self.default;
        }
        if self.lazy[n] != self.default {
            self._push(n);
        }
        if start >= l && end <= r {
            return self.cell[n];
        }
        let mid = (start + end) >> 1;
        let left = self._query(n << 1, start, mid, l, r);
        let right = self._query(n << 1 | 1, mid + 1, end, l, r);
        (self.op)(left, right)
    }

    /// Sets value `v` at index `i` if it satisfies `op`.
    pub fn insert(&mut self, i: usize, v: T) {
        self._insert(1, 1, self.n, i as i32, i as i32, v);
    }

    /// Sets value `v` in range `[l, r]` if it satisfies `op`.
    pub fn insert_range(&mut self, l: usize, r: usize, v: T) {
        self._insert(1, 1, self.n, l as i32, r as i32, v);
    }

    /// Queries a range of `[l, r]` and resolution happens via funtion `self.op`.
    pub fn query(&mut self, l: usize, r: usize) -> T {
        self._query(1, 1, self.n, l as i32, r as i32)
    }
}
