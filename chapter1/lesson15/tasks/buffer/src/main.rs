mod buffer; mod resource;
use resource::TempResource;


fn main() {
    let mut a: i32 = 5;
    let mut b: i32 = 10;

    println!("a: {} b: {}", a, b);
    buffer::swap(&mut a, &mut b);
    println!("a: {} b: {}", a, b);

    let s1 = String::from("hello");
    println!("{:?}", buffer::duplicate(&s1));

    let mut fd1 = resource::FileDescriptor::new("fd1", 10, 15);
    let mut fd2 = resource::FileDescriptor::new("fd2", 10, 17);

    println!("fd1: {} fd2: {}", fd1.id(), fd2.id());
    buffer::swap(&mut fd1, &mut fd2);
    println!("fd1: {} fd2: {}", fd1.id(), fd2.id());

    // a: 5 b: 10
    // a: 10 b: 5
    // ("hello", "hello")
    // fd1: fd1 fd2: fd2
    // [CLEANUP] Resource 'fd1' dropped. Open: true. Lifetime used: 18446744073709551600 sec.
    // [CLEANUP] Resource 'fd2' dropped. Open: true. Lifetime used: 18446744073709551598 sec.
    // fd1: fd2 fd2: fd1
    // [CLEANUP] Resource 'fd1' dropped. Open: true. Lifetime used: 18446744073709551600 sec.
    // [CLEANUP] Resource 'fd2' dropped. Open: true. Lifetime used: 18446744073709551598 sec.
}
