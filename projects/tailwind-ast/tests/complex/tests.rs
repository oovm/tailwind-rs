#[track_caller]
fn check_expand(input: &str, target: &str) {
    let styles = parse_tailwind(input).unwrap();
    let v: Vec<_> = styles.iter().map(|s| s.to_string()).collect();
    let output = v.join(" ");
    assert_eq!(target, output)
}

#[test]
fn test_expand() {
    check_expand("not-hover:sm:text-red-200", "not-hover:sm:text-red-200");
    check_expand("w(full sm:auto)", "w-full sm:w-auto");
    check_expand("w(1/2 sm:1/3 lg:1/6) p-2", "w-1/2 sm:w-1/3 lg:w-1/6 p-2");
    check_expand(
        "rotate(-3 hover:6 md:(3 hover:-6))",
        "-rotate-3 hover:rotate-6 md:rotate-3 md:hover:-rotate-6",
    );
    check_expand(
        "ring(& pink-700 offset(4 pink-200))",
        "ring ring-pink-700 ring-offset-4 ring-offset-pink-200",
    );
    check_expand(
        r#"
              bg-red-500 shadow-xs
              sm:(bg-red-600 shadow-sm)
              md:(
                bg-red-700 shadow(md)
              )
              lg:(
                bg-red-800 
                shadow(xl)
              )
        "#,
        "bg-red-500 shadow-xs sm:bg-red-600 sm:shadow-sm md:bg-red-700 md:shadow-md lg:bg-red-800 lg:shadow-xl",
    );
}
