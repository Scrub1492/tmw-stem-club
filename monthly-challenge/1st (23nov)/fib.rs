/// 12 Nov 2023
/// 一周目
/// 
/// Problem of the week: フィボナッチ
struct Fibonacci<T> {
    // _mを消してからフィールドを追加できる
    _m: std::marker::PhantomData<T>,
}

impl<T: std::ops::Add<Output = T> + Copy> Fibonacci<T> {
    fn new(first_seed: T, second_seed: T) -> Self {
        todo!()
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// ’first'と'second'からフィボナッチシーケンスイテレーターを戻す関数
/// 'std::ops::Add'を実装しているタイプに対して動きます。
pub fn fibonacci<T>(first_seed: T, second_seed: T) -> impl Iterator<Item = T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    // todo: std::iter::empty()を代えよ
    std::iter::empty()
}

// Public test case
mod tests {
    use super::*;

    #[test]
    fn test_fib_iter() {
        assert_eq!(
            fibonacci(0, 1)
                .take(10)
                .collect::<Vec<_>>(),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        );
    }
}
