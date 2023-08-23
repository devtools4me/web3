pub fn reverse<T>(v: Vec<T>) -> Vec<T> {
    v.into_iter().rev().collect()
}

pub fn convert<A, B>(v: Vec<A>, f: impl FnMut(A) -> B) -> Vec<B> {
    v.into_iter().map(f).collect()
}
