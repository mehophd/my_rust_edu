mod cache_entry; mod cache;


fn main() {
    let mut simplecache = cache::SimpleCache::new(100);
    let de1 = cache_entry::DataEntry::new("log1", vec![0; 10], 3);
    let de2 = cache_entry::DataEntry::new("log2", vec![0; 20], 3);
    let de3 = cache_entry::DataEntry::new("log3", vec![0; 30], 3);
    match simplecache.insert(de1) {
        Ok(()) => {},
        Err(e) => { println!("{}", e);}
    }
    match simplecache.insert(de2) {
        Ok(()) => {},
        Err(e) => { println!("{}", e);}
    }
    match simplecache.insert(de3) {
        Ok(()) => {},
        Err(e) => { println!("{}", e);}
    }
    let test1 = simplecache.get("a");
    let test2 = simplecache.get("log1");

    match test1 {
        Some(a) => {},
        None => { println!("Ничего"); }
    }

    match test2 {
        Some(a) => {},
        None => { println!("Ничего"); }
    }

    //Ничего
    //[DROP] Low-priority entry 'log3' cleaned
    //[DROP] Low-priority entry 'log1' cleaned
    //[DROP] Low-priority entry 'log2' cleaned
}
