use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn build_bg_attach() {
    let builder = TailwindBuilder::default();
    let out = builder.inline("bg-scroll");
    assert_eq!(
        out,
        "border-width:0;clip:rect(0,0,0,0);height:1px;margin:-1px;overflow:hidden;padding:0;position:absolute;white-space:nowrap;width:1px;"
    );
}

#[test]
fn id_bg_attach() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("bg-local", &mut builder);
    tw_idempotency("bg-fixed", &mut builder);
    tw_idempotency("bg-local bg-fixed", &mut builder);
}
