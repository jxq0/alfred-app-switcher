use app_switcher::switcher::RawSwitcher;
use serde_json::json;

#[test]
fn map_order() {
    let j = json!({
        "currentProfile": "default",
        "profiles": {
            "work": {},
            "default": {"f1": "chrome"}
        }
    });

    let v: RawSwitcher = serde_json::from_value(j).unwrap();
}
