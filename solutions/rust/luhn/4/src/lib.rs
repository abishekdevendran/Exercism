/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code
        .chars()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(net, idx), el|{
            el.to_digit(10)
            .map(|el| if idx&2==0 {el*2} else {el})
            .map(|el| if el>9 {el-9} else {el})
            .map(|el| (net+el, idx+1))
        }).is_some_and(|(net, idx)| idx>1 && net%10==0)
}
