use tree_sitter::{Parser, Query, QueryCapture, QueryCursor, QueryMatch};

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_rust::language())
        .expect("Error loading Rust grammar");
    let query = Query::new(tree_sitter_rust::language(), RUST_TEMPLATES).unwrap();
    let source = r#"
        jinja.render_template("template");
        jinja.render_all("template");
        jinja.hello_world("template");
        "#;

    let tree = parser.parse(source.as_bytes(), None).unwrap();
    let closest_node = tree.root_node();
    let mut cursor_qry = QueryCursor::new();
    let captures = cursor_qry.captures(&query, closest_node, source.as_bytes());
    let z = collect_captures(captures, &query, source);
    dbg!(z.len());
}

pub fn collect_captures<'a>(
    captures: impl Iterator<Item = (QueryMatch<'a, 'a>, usize)>,
    query: &'a Query,
    source: &'a str,
) -> Vec<(&'a str, &'a str)> {
    format_captures(captures.map(|(m, i)| m.captures[i]), query, source)
}

fn format_captures<'a>(
    captures: impl Iterator<Item = QueryCapture<'a>>,
    query: &'a Query,
    source: &'a str,
) -> Vec<(&'a str, &'a str)> {
    captures
        .map(|capture| {
            (
                query.capture_names()[capture.index as usize].as_str(),
                capture.node.utf8_text(source.as_bytes()).unwrap(),
            )
        })
        .collect()
}

const RUST_TEMPLATES: &str = r#"
(call_expression
  	[
    	(field_expression
        	(field_identifier) @method_name
        )
        (identifier) @method_name
        (#any-of? @method_name "render_jinja" "get_template")
      ;;(#match? @method_name "(render_jinja|get_template)")
    ]
    (arguments
      (string_literal)+ @template_name
    )
)
"#;
