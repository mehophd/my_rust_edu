struct BorrowedCache<'a, 'b> {
    key: &'a str,
    value: &'b str,
}

impl<'a, 'b> BorrowedCache<'a, 'b> {
    fn new(key: &'a str, value: &'b str) -> Self {
        Self {key: key, value: value}
    }

    fn get_key(&self) -> &'a str {
        self.key
    }
}

fn main() {
    let long_key = String::from("long_lived_key");
    let mut cache = BorrowedCache::new(&long_key, "some");
    {
        let short_key = String::from("short");  
        cache = BorrowedCache::new(&short_key, "some");
        println!("{}", cache.get_key());
    }
    
    println!("{}", cache.get_key());  // ошибка из-за того, что short_key умирает раньше, чем cache
    /*
    error[E0597]: `short_key` does not live long enough
    --> src/main.rs:21:36
    |
    20 |         let short_key = String::from("short");  
    |             --------- binding `short_key` declared here
    21 |         cache = BorrowedCache::new(&short_key, "some");
    |                                    ^^^^^^^^^^ borrowed value does not live long enough
    22 |         println!("{}", cache.get_key());
    23 |     }
    |     - `short_key` dropped here while still borrowed
    24 |     
    25 |     println!("{}", cache.get_key());  // ошибка
    |                    ----- borrow later used here

    warning: value assigned to `cache` is never read
    --> src/main.rs:18:21
    |
    18 |     let mut cache = BorrowedCache::new(&long_key, "some");
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

    For more information about this error, try `rustc --explain E0597`.
    warning: `error` (bin "error") generated 1 warning
    error: could not compile `error` (bin "error") due to 1 previous error; 1 warning emitted
    */
}
