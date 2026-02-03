fn main() {
    let s = String::from("hello");

    // Много неизменяемых — ОК
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{}", r2);

    // Одна изменяемая — ОК
    let mut s2 = String::from("hello");
    let r = &mut s2;
    r.push_str(" world");
    println!("{}", r);

    // Смешанные ссылки — ОШИБКА!
    //let mut s3 = String::from("hello");
    //let r1 = &mut s3;      // неизменяемая
    //let r2 = &mut s3;
    // изменяемая ← КОМПИЛЯТОР ЗАПРЕТИТ!
    //r1.push_str(" guy");
    //r2.push_str(" world");

    //println!("{}", r1);
    //println!("{}", r2);
}
