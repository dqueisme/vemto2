#[cfg(test)]
mod tests {
    use crate::models::*;

    #[test]
    fn test_project_serialization() {
        let project = Project::default_laravel_project();
        let json = project.to_json().unwrap();
        let decoded = Project::from_json(&json).unwrap();
        assert_eq!(project, decoded);
    }
}
