#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    length: f64,
    height: f64,
}

macro_rules! validates {
    ($instance:expr, $( $field:ident => $rule:ident $( ( $value:expr ) )? ),+ $(,)?) => {{
        let mut errors = Vec::<String>::new();

        $(
            validate_rule!(errors, $instance, $field, $rule $(, $value)?);
        )+

        errors
    }};
}

macro_rules! validate_rule {
    ($errors:expr, $instance:expr, $field:ident, presence) => {{
        if $instance.$field.to_string().trim().is_empty() {
            $errors.push(format!("{} can't be blank", stringify!($field)));
        }
    }};

    ($errors:expr, $instance:expr, $field:ident, full_name) => {{
        let value = $instance.$field.to_string();
        let parts: Vec<&str> = value.split_whitespace().collect();

        if parts.len() < 2 {
            $errors.push(format!("{} is not a full name", stringify!($field)));
        }
    }};

    ($errors:expr, $instance:expr, $field:ident, greater_than, $value:expr) => {{
        if !($instance.$field > $value) {
            $errors.push(format!(
                "{} must be greater than {}",
                stringify!($field),
                $value
            ));
        }
    }};

    ($errors:expr, $instance:expr, $field:ident, less_than, $value:expr) => {{
        if !($instance.$field < $value) {
            $errors.push(format!(
                "{} must be less than {}",
                stringify!($field),
                $value
            ));
        }
    }};

    ($errors:expr, $instance:expr, $field:ident, min, $value:expr) => {{
        if !($instance.$field >= $value) {
            $errors.push(format!(
                "{} must be at least {}",
                stringify!($field),
                $value
            ));
        }
    }};

    ($errors:expr, $instance:expr, $field:ident, max, $value:expr) => {{
        if !($instance.$field <= $value) {
            $errors.push(format!(
                "{} must be at most {}",
                stringify!($field),
                $value
            ));
        }
    }};
}

#[test]
fn it_works() {
    let person1 = Person {
        name: String::from("Carlos Eduardo"),
        age: 28,
        length: 100.0,
        height: 1.76,
    };

    let errors = validates!(
        person1,
        name => presence,
        name => full_name,
        age => greater_than(18),
        age => less_than(120),
        height => min(1.50),
        height => max(2.10),
    );

    assert!(errors.is_empty());
}

#[test]
fn it_collects_errors() {
    let person = Person {
        name: String::from("Carlos"),
        age: 15,
        length: 100.0,
        height: 1.20,
    };

    let errors = validates!(
        person,
        name => full_name,
        age => greater_than(18),
        height => min(1.50),
    );

    assert_eq!(errors.len(), 3);
}
