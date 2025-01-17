use crate::inc::SemVerAction;
use crate::Inc;
use nu_plugin::{EngineInterface, EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{ast::CellPath, PluginSignature, SyntaxShape, Value};

impl Plugin for Inc {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("inc")
            .usage("Increment a value or version. Optionally use the column of a table.")
            .optional("cell_path", SyntaxShape::CellPath, "cell path to update")
            .switch(
                "major",
                "increment the major version (eg 1.2.1 -> 2.0.0)",
                Some('M'),
            )
            .switch(
                "minor",
                "increment the minor version (eg 1.2.1 -> 1.3.0)",
                Some('m'),
            )
            .switch(
                "patch",
                "increment the patch version (eg 1.2.1 -> 1.2.2)",
                Some('p'),
            )]
    }

    fn run(
        &self,
        name: &str,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        if name != "inc" {
            return Ok(Value::nothing(call.head));
        }

        let mut inc = self.clone();

        let cell_path: Option<CellPath> = call.opt(0)?;

        inc.cell_path = cell_path;

        if call.has_flag("major")? {
            inc.for_semver(SemVerAction::Major);
        }
        if call.has_flag("minor")? {
            inc.for_semver(SemVerAction::Minor);
        }
        if call.has_flag("patch")? {
            inc.for_semver(SemVerAction::Patch);
        }

        inc.inc(call.head, input)
    }
}
