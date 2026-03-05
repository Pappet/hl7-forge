use rolling_file::*;
fn main() {
    let _ =
        BasicRollingFileAppender::new("test.log", RollingConditionBasic::new().max_size(1024), 5)
            .unwrap();
}
