mod resource; mod manager;
use resource::TempResource;

fn main() {
    let fd1 = resource::FileDescriptor::new("fd1", 10, 15);
    let fd2 = resource::FileDescriptor::new("fd2", 10, 17);
    let fd3 = resource::FileDescriptor::new("fd3", 10, 20);
    let mut resman = manager::ResourceManager::new();

    match resman.register(fd1.id(), fd1.created_at()) {
        Ok(()) => println!("успех"),
        Err(e) => println!("{}", e)
    }

    match resman.register(fd2.id(), fd2.created_at()) {
        Ok(()) => println!("успех"),
        Err(e) => println!("{}", e)
    }

    match resman.register(fd3.id(), fd3.created_at()) {
        Ok(()) => println!("успех"),
        Err(e) => println!("{}", e)
    }

    let timeouts = resman.check_timeouts(150);
    let binding = ["fd1".to_string(), "fd2".to_string(), "nonexistent".to_string()];
    let report = resman.get_report(&binding);

    println!("{:?}", timeouts);
    println!("{:?}", report);

    // успех
    // успех
    // успех
    // ["fd1", "fd2", "fd3"]
    // ["fd1", "fd2"]
    // [CLEANUP] Resource 'fd3' dropped. Open: true. Lifetime used: 18446744073709551595 sec.
    // [CLEANUP] Resource 'fd2' dropped. Open: true. Lifetime used: 18446744073709551598 sec.
    // [CLEANUP] Resource 'fd1' dropped. Open: true. Lifetime used: 18446744073709551600 sec.
}
