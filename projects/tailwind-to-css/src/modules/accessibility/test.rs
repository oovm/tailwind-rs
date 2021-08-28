use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn build_screen_reader() {
    let mut builder = TailwindBuilder::default();
    let out = builder.inline("sr-only");
    assert_eq!(
        out,
        "border-width:0;clip:rect(0,0,0,0);height:1px;margin:-1px;overflow:hidden;padding:0;position:absolute;white-space:nowrap;width:1px;"
    );
    let out = builder.inline("not-sr-only");
    assert_eq!(
        //
        out,
        "clip:auto;height:auto;margin:0;overflow:visible;padding:0;position:static;white-space:normal;width:auto;"
    );
}

#[test]
fn id_screen_reader() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("sr-only", &mut builder);
    tw_idempotency("not-sr-only", &mut builder);
}
