/// Me: Mom, can I have Jane Street?
///
/// Mom: No, we have Jane Street at home
/// 
/// Jane Street at home:
#[macro_export]
macro_rules! expect {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (lexpr, rexpr) => {
                let __f = |s: &str| -> String {
                    let lines = s.lines().collect::<Vec<_>>();
                    lines
                        .into_iter()
                        .map(str::trim)
                        .filter(|l| !l.is_empty())
                        .map(str::to_string)
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                let __left = __f(lexpr);
                let __right = __f(rexpr);
                if __left != __right {
                    panic!(
                        "expect test failed\n\noutput:\n{}\n\nexpected:\n{}\n",
                        __left,
                        __right,
                    );
                }
            }
        }
    }};
}
