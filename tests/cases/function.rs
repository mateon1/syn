fn f() {
    (
        || (),
        |_| (),
        || -> () { () },
        |a| a,
        |a, b| a + b,
        |a: u8, b: u8| a + b,
        |a, b| -> u8 { a + b },
        move |a, b| a + b,
    )
}

fn ascript() {
    (
        G::<u8>::f::<S>(),
        <G<u8> as m::Trait<u8>>::Assoc::f::<S>(),
    )
}

fn conditional() {
    if a == b {
        0
    } else if a > b {
        1
    } else {
        -1
    }
}