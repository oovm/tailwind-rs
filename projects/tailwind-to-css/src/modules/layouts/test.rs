use crate::{tw_idempotency, TailwindBuilder};

#[test]
fn id_aspect() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("aspect-auto", &mut builder);
    tw_idempotency("aspect-square", &mut builder);
    tw_idempotency("aspect-video", &mut builder);
    tw_idempotency("aspect-1/1", &mut builder);
    tw_idempotency("aspect-1/1 aspect-square", &mut builder);
}

#[test]
fn id_columns() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("columns-12", &mut builder);
    tw_idempotency("columns-xs", &mut builder);
    tw_idempotency("columns-auto", &mut builder);
    tw_idempotency("columns-12 columns-xs columns-auto", &mut builder);
}

#[test]
fn id_break() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("break-before-auto", &mut builder);
    tw_idempotency("break-inside-auto", &mut builder);
    tw_idempotency("break-after-auto", &mut builder);
    tw_idempotency("break-before-auto break-inside-auto break-after-auto", &mut builder);
}

#[test]
fn id_box_decoration() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("box-decoration-clone", &mut builder);
    tw_idempotency("box-decoration-slice", &mut builder);
    tw_idempotency("box-decoration-clone box-decoration-slice", &mut builder);
}

#[test]
fn id_box_sizing() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("box-border", &mut builder);
    tw_idempotency("box-content", &mut builder);
    tw_idempotency("box-border box-content", &mut builder);
}

#[test]
fn build_display() {
    let mut builder = TailwindBuilder::default();
    let out = builder.inline("flex");
    assert_eq!(out, "box-sizing:border-box;");
    let out = builder.inline("hidden");
    assert_eq!(out, "box-sizing:content-box;");
}

#[test]
fn id_float() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("float-none", &mut builder);
    tw_idempotency("float-left", &mut builder);
    tw_idempotency("float-right", &mut builder);
    tw_idempotency("float-left float-right float-none", &mut builder);
}

fn id_clear() {
    let mut builder = TailwindBuilder::default();
    tw_idempotency("float-left float-right", &mut builder);
    tw_idempotency("float-both float-none", &mut builder);
}
