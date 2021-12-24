mod html;

#[test]
fn ready() {
    println!("it works!")
}

// TODO
// #[cfg(test)]
// pub fn tw_idempotency(input1: &str, builder: &mut TailwindBuilder) {
//     let input2 = &builder.trace(input1).unwrap();
//     assert_eq!(builder.inline(input1).unwrap(), builder.inline(input2).unwrap())
// }
