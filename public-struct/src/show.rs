pub trait Show<T> {
    // type ShowType;
    fn show(&self) -> String;
}