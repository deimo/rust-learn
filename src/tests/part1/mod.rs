use crate::part1_closure;

#[cfg(test)]
#[test]
pub fn test1() {
    let res = part1_closure::demo1();
    assert_eq!(res, 3);
}

#[test]
// #[should_panic]
pub fn test2() {
    part1_closure::demo2();
}

#[test]
pub fn test3() {
    let x = vec![1, 2, 3];
    part1_closure::fn_trait_demo::fn_once(|z| z == x.len());
}

#[test]
pub fn test4() {
    part1_closure::fn_trait_demo::fn_demo();
}

#[test]
pub fn test5() {
    part1_closure::fn_trait_demo::fn_mut();
}

