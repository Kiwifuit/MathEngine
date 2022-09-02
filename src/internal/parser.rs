use pomsky_macro::pomsky;

const ENGINE_PARSER: &str = pomsky!(
:left([d w]*)
' '*
:opcode(['+' '-' '/' '^'])
' '*
:right([d w]*)
);
