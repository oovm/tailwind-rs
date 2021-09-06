use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn build_delay() {
    let mut builder = TailwindBuilder::default();
    let out = builder.inline("delay-1000");
    assert_eq!(out, "transition-delay:1000ms;");
}

#[test]
fn id_rounded() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("bg-local", &mut builder);
    tw_idempotency("bg-fixed", &mut builder);
    tw_idempotency("bg-local bg-fixed", &mut builder);
}

#[test]
fn build_duration() {
    let mut builder = TailwindBuilder::default();
    let out = builder.inline("duration-1000");
    assert_eq!(out, "transition-duration:1000ms;");
}
