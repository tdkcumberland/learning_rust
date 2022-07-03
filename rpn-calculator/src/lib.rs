#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    fn get_value(&self) -> Option<i32> {
        match self {
            CalculatorInput::Value(v) =>  Some(*v),
            _ => None,
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() == 0 || inputs.len() == 2 {
        None
    } 
    else if inputs.len() == 1 {
        inputs[0].get_value()
    }
    else 
    {
        let mut new_vec: Vec<i32> = Vec::new();

        for element in inputs.iter() {
            if let CalculatorInput::Value(v) = element {
                new_vec.push(*v)
            } else {
                // when this panics it will propagate a None since this is a primative
                let v2 = new_vec.pop()?;
                let v1 = new_vec.pop()?;
                let value = match element {
                    CalculatorInput::Add => v1 + v2,
                    CalculatorInput::Subtract => v1 - v2,
                    CalculatorInput::Multiply => v1 * v2,
                    CalculatorInput::Divide => v1 / v2,
                    _ => panic!("nope")
                };
                new_vec.push(value);
            }
        }

        if new_vec.len() == 1 { Some(new_vec[0])}
        else {None}
    }
}
