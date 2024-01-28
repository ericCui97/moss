use std::process::Command;

//pub fn run_file_via_command(path: &str)->Result<Vec<&str>,String>{
//    let output = Command::new("cargo")
//        .arg("run")
//        .arg(path)
//        .output()
//        .expect("failed to execute process");
//	let lines = std::str::from_utf8(output.stdout.clone().as_slice())
//		.unwrap()
//		.split('\n')
//		.collect::<Vec<&str>>();
//Ok(lines)
//}

pub fn run_file_via_command(path: &str) -> Result<Vec<String>, String> {
    let output = Command::new("cargo")
        .arg("run")
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Command failed with exit code {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let lines = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to convert stdout to string: {}", e))?
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(lines)
}
