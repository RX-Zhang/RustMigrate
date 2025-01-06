
const DIFF: fn(i32, i32) -> i32 = |x: i32, y: i32| {
(x - y) & 0x7fffffff
};
