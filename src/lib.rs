pub mod math;

fn calc() {
    // 公開されてるAPIなのでOK
    let pow = math::exponentiation::pow(10);
    // 公開されていないAPIなのでビルドエラーが起こる
    // let pow_inner = math::exponentiation::pow_impl(10);
    assert_eq!(pow, 100);
}
