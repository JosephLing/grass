#![cfg(test)]

#[macro_use]
mod macros;

test!(
    type_is_string,
    "a {\n  color: type-of(foo(1+1));\n}\n",
    "a {\n  color: string;\n}\n"
);
test!(
    evaluates_arguments,
    "a {\n  color: foo(1+1);\n}\n",
    "a {\n  color: foo(2);\n}\n"
);
test!(
    arguments_are_comma_separated,
    "a {\n  color: foo(1+1, 2+3, 4+5);\n}\n",
    "a {\n  color: foo(2, 5, 9);\n}\n"
);
test!(
    converts_sql_quotes,
    "a {\n  color: foo('hi');\n}\n",
    "a {\n  color: foo(\"hi\");\n}\n"
);
test!(
    super_selector,
    "a {\n  color: foo(&);\n}\n",
    "a {\n  color: foo(a);\n}\n"
);
test!(
    nested_plain_css_fn,
    "a {\n  color: foo(foo(foo(foo(1+1))));\n}\n",
    "a {\n  color: foo(foo(foo(foo(2))));\n}\n"
);
error!(
    disallows_named_arguments,
    "a {\n  color: foo($a: 1+1);\n}\n",
    "Error: Plain CSS functions don't support keyword arguments."
);
test!(
    evalutes_variables,
    "a {\n  $primary: #f2ece4;\n  $accent: #e1d7d2;\n  color: radial-gradient($primary, $accent);\n}\n",
    "a {\n  color: radial-gradient(#f2ece4, #e1d7d2);\n}\n"
);
