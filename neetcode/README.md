# NeetCode

Practicing the [NeetCode 150](https://neetcode.io/practice/practice/neetcode150)
in Rust.

Every **Easy** problem in the NeetCode 150 is its own standalone Cargo project,
grouped by roadmap category:

```
neetcode/
  arrays_hashing/
    two_sum/
      Cargo.toml
      src/lib.rs              # the stub you implement (todo!())
      tests/two_sum.rs        # the spec: example cases as an integration test
  trees/
    invert_binary_tree/
    ...
```

Each `src/lib.rs` has a `todo!()` stub; the matching `tests/` file holds the
example cases. Implement the body, then run that project's tests:

```bash
cd arrays_hashing/two_sum
cargo test          # fails on todo! until you implement it, then goes green
```

Linked-list and tree problems embed the `ListNode` / `TreeNode` definitions
LeetCode gives you, plus a small builder in their test file. Mediums and Hards
will slot into the same category folders later.

| Category | Problem | LeetCode |
|----------|---------|----------|
| Arrays & Hashing | `contains_duplicate` | 217 |
| Arrays & Hashing | `is_anagram` | 242 |
| Arrays & Hashing | `two_sum` | 1 |
| Two Pointers | `is_palindrome` | 125 |
| Sliding Window | `max_profit` | 121 |
| Stack | `is_valid` | 20 |
| Binary Search | `search` | 704 |
| Linked List | `reverse_list` | 206 |
| Linked List | `merge_two_lists` | 21 |
| Linked List | `has_cycle` | 141 |
| Trees | `invert_tree` | 226 |
| Trees | `max_depth` | 104 |
| Trees | `diameter_of_binary_tree` | 543 |
| Trees | `is_balanced` | 110 |
| Trees | `is_same_tree` | 100 |
| Trees | `is_subtree` | 572 |
| Heap / Priority Queue | `KthLargest` | 703 |
| Heap / Priority Queue | `last_stone_weight` | 1046 |
| 1-D DP | `climb_stairs` | 70 |
| 1-D DP | `min_cost_climbing_stairs` | 746 |
| Bit Manipulation | `single_number` | 136 |
| Bit Manipulation | `hamming_weight` | 191 |
| Bit Manipulation | `count_bits` | 338 |
| Bit Manipulation | `reverse_bits` | 190 |
| Bit Manipulation | `missing_number` | 268 |
| Math & Geometry | `is_happy` | 202 |
| Math & Geometry | `plus_one` | 66 |
