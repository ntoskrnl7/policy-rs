#[cfg(test)]
mod tests {
    use policy::{Base, Loader};
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    #[derive(Loader, Serialize, Deserialize, Clone, Debug)]
    struct TestPolicy {
        pub value1: Option<Duration>,
        pub value2: Option<Duration>,
        pub value3: Duration,
    }

    impl Base for TestPolicy {
        fn adjust_default_options(&mut self) -> Self {
            if let None = self.value2 {
                self.value2 = Some(Duration::from_secs(2))
            }
            self.to_owned()
        }

        fn default() -> Self {
            TestPolicy {
                value1: Some(Duration::from_secs(1)),
                value2: Some(Duration::from_secs(2)),
                value3: Duration::from_secs(3),
            }
        }
    }

    #[test]
    fn default_test() {
        let test_policy = TestPolicy::default();
        assert!(matches!(test_policy.value1, Some(val) if val == Duration::from_secs(1)));
        assert!(matches!(test_policy.value2, Some(val) if val == Duration::from_secs(2)));
        assert_eq!(test_policy.value3, Duration::from_secs(3));
    }

    #[test]
    fn from_yaml_file_test() -> std::io::Result<()> {
        let test_policy = TestPolicy::from_path("./tests/test.yaml")?;
        assert!(matches!(test_policy.value1, Some(val) if val == Duration::from_secs(1)));
        assert!(matches!(test_policy.value2, Some(val) if val == Duration::from_secs(2)));
        assert_eq!(test_policy.value3, Duration::from_secs(3));
        Ok(())
    }

    #[test]
    fn from_json_file_test() -> std::io::Result<()> {
        let test_policy = TestPolicy::from_path("./tests/test.json")?;
        assert!(matches!(test_policy.value1, Some(val) if val == Duration::from_secs(1)));
        assert!(matches!(test_policy.value2, Some(val) if val == Duration::from_secs(2)));
        assert_eq!(test_policy.value3, Duration::from_secs(3));
        Ok(())
    }

    #[test]
    fn from_toml_file_test() -> std::io::Result<()> {
        let test_policy = TestPolicy::from_path("./tests/test.toml")?;
        assert!(matches!(test_policy.value1, Some(val) if val == Duration::from_secs(1)));
        assert!(matches!(test_policy.value2, Some(val) if val == Duration::from_secs(2)));
        assert_eq!(test_policy.value3, Duration::from_secs(3));
        Ok(())
    }

    #[test]
    fn serde_yaml_test() {
        let test_policy = TestPolicy::default();
        let yaml_str = serde_yaml::to_string(&test_policy).unwrap();
        let test_policy = serde_yaml::from_str::<TestPolicy>(&yaml_str).unwrap();
        assert!(matches!(test_policy.value1, Some(val) if val == Duration::from_secs(1)));
        assert!(matches!(test_policy.value2, Some(val) if val == Duration::from_secs(2)));
        assert_eq!(test_policy.value3, Duration::from_secs(3));
    }

    #[test]
    fn serde_json_test() {
        let test_policy = TestPolicy::default();
        let json_str = serde_json::to_string(&test_policy).unwrap();
        let test_policy = serde_json::from_str::<TestPolicy>(&json_str).unwrap();
        assert!(matches!(test_policy.value1, Some(val) if val == Duration::from_secs(1)));
        assert!(matches!(test_policy.value2, Some(val) if val == Duration::from_secs(2)));
        assert_eq!(test_policy.value3, Duration::from_secs(3));
    }

    #[test]
    fn serde_toml_test() {
        let test_policy = TestPolicy::default();
        let toml_str = toml::to_string(&test_policy).unwrap();
        let test_policy = toml::from_str::<TestPolicy>(&toml_str).unwrap();
        assert!(matches!(test_policy.value1, Some(val) if val == Duration::from_secs(1)));
        assert!(matches!(test_policy.value2, Some(val) if val == Duration::from_secs(2)));
        assert_eq!(test_policy.value3, Duration::from_secs(3));
    }
}
