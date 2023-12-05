use std::mem;

pub struct ReducesIter<Iter, Value, Reducer>
where
    Iter: Iterator,
    Reducer: Fn(&mut Value, Iter::Item) -> bool,
{
    iter: Iter,
    initial: Value,
    current_value: Option<Value>,
    reducer: Reducer,
}

impl<Iter, Value, Reducer> Iterator for ReducesIter<Iter, Value, Reducer>
where
    Iter: Iterator,
    Reducer: Fn(&mut Value, Iter::Item) -> bool,
    Value: Clone,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = self.iter.next();
            if entry.is_none() {
                self.current_value.as_ref()?;
                return mem::take(&mut self.current_value);
            }
            let mut current_value = self
                .current_value
                .take()
                .unwrap_or_else(|| self.initial.clone());
            let entry_value = entry.unwrap();
            let should_continue = (self.reducer)(&mut current_value, entry_value);

            if !should_continue {
                return Some(current_value);
            } else {
                self.current_value = Some(current_value);
            }
        }
    }
}

pub trait Reduces<Iter>
where
    Iter: Iterator,
{
    fn reduces<Value, Reducer>(
        self,
        initial: Value,
        reducer: Reducer,
    ) -> ReducesIter<Iter, Value, Reducer>
    where
        Reducer: Fn(&mut Value, Iter::Item) -> bool,
        Value: Clone;
}

impl<Iter> Reduces<Iter> for Iter
where
    Iter: Iterator,
{
    fn reduces<Value, Reducer>(
        self,
        initial: Value,
        reducer: Reducer,
    ) -> ReducesIter<Iter, Value, Reducer>
    where
        Reducer: Fn(&mut Value, Iter::Item) -> bool,
        Value: Clone,
    {
        ReducesIter {
            iter: self,
            initial,
            current_value: None,
            reducer,
        }
    }
}
