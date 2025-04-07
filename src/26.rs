// Example Rust code for collaborative file management: project management
fn main() {
    let files = vec![
        "file1.txt",
        "folder1/file2.txt",
        "subfolder/child/file3.txt",
        "project_management/project_details.json",
    ];

    for &file in files.iter().rev() {
        if file.starts_with("project_") || file.ends_with(".json") {
            println!("Deleting: {}", file);
            std::fs::remove_file(file).expect("Failed to delete the file");
        }
    }

    let project_details = r#"{
        "title": "Project A",
        "description": "This is a description of Project A."
    }"#;

    if !project_details.trim().is_empty() {
        println!("Saving project details: {}", project_details);
        std::fs::write("project_management/project_details.json", project_details).expect("Failed to write the file");
    }
}
