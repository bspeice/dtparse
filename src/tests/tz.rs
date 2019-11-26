use parse;

#[test]
fn est() {
    // Issue originally reported in https://github.com/bspeice/dtparse/issues/18
    let dt = parse("Fri, 21 Aug 2015 18:37:44 EST");

    assert!(dt.is_ok());
    assert!(dt.unwrap().1.is_some());
}

#[test]
fn cest() {
    // Issue originally reported in https://github.com/bspeice/dtparse/issues/18
    let dt = parse("Fri, 21 Aug 2015 18:37:44 CEST");

    assert!(dt.is_ok());
    // TODO: Fix
    // assert!(dt.unwrap().1.is_some());
}
