pub fn infinite<T>(res: &[T]) -> impl Iterator<Item = &T> + '_ {
    std::iter::successors(Some(0), |&ind| Some((ind + 1) % res.len())).map(|ind| &res[ind])
}
