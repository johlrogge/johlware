pub trait Read<T> {
    fn read(&self) -> T;
}

pub trait Write<T> {
    fn write(&self, value:T);
}
