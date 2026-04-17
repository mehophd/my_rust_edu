pub fn swap<T: Clone>(a: &mut T, b: &mut T) {
    let c = a.clone();
    *a = b.clone();
    *b = c;
}

pub fn duplicate<T: Clone>(value: &T) -> (T, T) {
    // Возвращает кортеж из двух клонов value
    (value.clone(), value.clone())
}