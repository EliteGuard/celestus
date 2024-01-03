pub fn upsert_obj_prop(
    json_object: &mut serde_json::Value,
    property: &String,
    value: serde_json::Value,
    override_type: bool,
) {
    let pair = json_object.get_mut(property);
    if pair.is_none() {
        json_object[property] = value;
        return;
    }

    update_obj_prop(json_object, property, value, override_type);
}

pub fn insert_obj_prop(
    json_object: &mut serde_json::Value,
    property: &String,
    value: serde_json::Value,
) {
    let pair = json_object.get_mut(property);
    if pair.is_none() {
        json_object[property] = value;
    }
}

pub fn update_obj_prop(
    json_object: &mut serde_json::Value,
    property: &String,
    value: serde_json::Value,
    override_type: bool,
) {
    if override_type {
        json_object[property] = value;
        return;
    }

    let existing_type = json_object.get_mut(property).unwrap();

    match existing_type {
        serde_json::Value::Null => {
            if value.is_null() {
                json_object[property] = value;
            }
        }
        serde_json::Value::Bool(_) => {
            if value.is_boolean() {
                json_object[property] = value;
            }
        }
        serde_json::Value::Number(_) => {
            if value.is_number() {
                json_object[property] = value;
            }
        }
        serde_json::Value::String(_) => {
            if value.is_string() {
                json_object[property] = value;
            }
        }
        serde_json::Value::Array(_) => {
            if value.is_array() {
                json_object[property] = value;
            }
        }
        serde_json::Value::Object(_) => {
            if value.is_object() {
                json_object[property] = value;
            }
        }
    };
}
