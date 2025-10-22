// Tests for Template struct with optional suffix
// Place in tests/fileio_tests.rs

use json_idl_code_gen::fileio::Config;

#[test]
fn test_template_suffix_optional() {
    let yaml = r#"
    templates:
      - file: "macros.jinja"
        name: "macros.jinja"
      - file: "datatypes.h.jinja"
        name: "header_tpl"
        suffix: "h"
    "#;
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.templates.len(), 2);
    assert_eq!(config.templates[0].name, "macros.jinja");
    assert_eq!(config.templates[0].suffix, None);
    assert_eq!(config.templates[1].name, "header_tpl");
    assert_eq!(config.templates[1].suffix.as_deref(), Some("h"));
}
