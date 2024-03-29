mod common;

#[cfg(test)]
mod tests {
    use crate::common::cli::run_file_via_command;
    #[test]
    fn interpret_block() {
        let lines = run_file_via_command("test_file/block.moss").unwrap();
        assert_eq!(lines[0], "2");
        assert_eq!(lines[1], "3");
    }

    #[test]
    fn interpret_for() {
        let lines = run_file_via_command("test_file/test_for.moss").unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
    }
    #[test]
    fn test_fn() {
        let lines = run_file_via_command("test_file/function/test_fn.moss").unwrap();
        // assert_eq!(lines.len(), 3);
        // 第一行一定大于 1706547443.625
        assert!(lines[0].parse::<f64>().unwrap() > 1706547443.625);
        assert_eq!(lines[1], "3");
        assert_eq!(lines[2], "14");
        assert_eq!(lines[3], "1");
        assert_eq!(lines[4], "0");
        assert_eq!(lines[5], "2");
        assert_eq!(lines[6], "3");
    }
    #[test]
    fn test_fn_closure() {
        let lines = run_file_via_command("test_file/function/closure.moss").unwrap();
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "1");
        assert_eq!(lines[3], "2");
    }
    #[test]
    fn test_fn_anonymous() {
        let lines = run_file_via_command("test_file/function/anon_fn.moss").unwrap();
        assert_eq!(lines[0], "0");
        assert_eq!(lines[1], "1");
        assert_eq!(lines[2], "2");
        assert_eq!(lines[3], "4");
        assert_eq!(lines[4], "9");
        assert_eq!(lines[5], "3");
        assert_eq!(lines[6], "100");
    }

    #[test]
    fn test_fn_temp() {
        let lines = run_file_via_command("test_file/function/anon_fn.moss").unwrap();
    }
}
