use api::*;

struct GuestTest {
    some_state: bool,
}

impl Extension for GuestTest {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self { some_state: true }
    }

    fn invoke_func(&self, fn_name: &str, args: &[DataVariant]) -> Result<DataVariant, String> {
        match fn_name.to_ascii_lowercase().as_str() {
            "some_func" => some_func(args),
            "add" => add(args),
            "check_state" => check_state(args, self),
            _ => Err("Function not found".to_string()),
        }
    }
}

fn some_func(_: &[DataVariant]) -> Result<DataVariant, String> {
    Ok(DataVariant::Text("Hello".to_string()))
}

fn add(args: &[DataVariant]) -> Result<DataVariant, String> {
    match args {
        [DataVariant::Number(first), DataVariant::Number(second)] => {
            Ok(DataVariant::Number(first + second))
        }
        _ => Err("FAILED".to_string()),
    }
}

fn check_state(_: &[DataVariant], ext: &GuestTest) -> Result<DataVariant, String> {
    Ok(DataVariant::Boolean(ext.some_state))
}

register_extension!(GuestTest);
