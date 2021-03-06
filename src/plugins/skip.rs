use nu::{
    serve_plugin, CallInfo, CoerceInto, Plugin, Primitive, ReturnSuccess, ReturnValue, ShellError,
    Signature, SyntaxShape, Tagged, TaggedItem, Value,
};

struct Skip {
    skip_amount: i64,
}

impl Skip {
    fn new() -> Skip {
        Skip { skip_amount: 0 }
    }
}

impl Plugin for Skip {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("skip")
            .desc("Skip a number of rows")
            .rest(SyntaxShape::Number)
            .filter())
    }
    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        if let Some(args) = call_info.args.positional {
            for arg in args {
                match arg {
                    Tagged {
                        item: Value::Primitive(Primitive::Int(i)),
                        tag,
                    } => {
                        self.skip_amount = i.tagged(tag).coerce_into("converting for skip")?;
                    }
                    _ => {
                        return Err(ShellError::labeled_error(
                            "Unrecognized type in params",
                            "expected an integer",
                            arg.tag(),
                        ))
                    }
                }
            }
        }

        Ok(vec![])
    }

    fn filter(&mut self, input: Tagged<Value>) -> Result<Vec<ReturnValue>, ShellError> {
        if self.skip_amount == 0 {
            Ok(vec![ReturnSuccess::value(input)])
        } else {
            self.skip_amount -= 1;
            Ok(vec![])
        }
    }
}

fn main() {
    serve_plugin(&mut Skip::new());
}
