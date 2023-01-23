use bevy_ecs_ldtk::prelude::FieldValue;

fn explain_field(value: &FieldValue) -> String {
    match value {
        FieldValue::Int(Some(i)) => format!("has an integer of {}", i),
        FieldValue::Float(Some(f)) => format!("has a float of {}", f),
        FieldValue::Bool(b) => format!("is {}", b),
        FieldValue::String(Some(s)) => format!("says {}", s),
        FieldValue::Color(c) => format!("has the color {:?}", c),
        FieldValue::Enum(Some(e)) => format!("is the variant {}", e),
        FieldValue::FilePath(Some(f)) => format!("references {}", f),
        FieldValue::Point(Some(p)) => format!("is at ({}, {})", p.x, p.y),
        a => format!("is hard to explain: {:?}", a),
    }
}
