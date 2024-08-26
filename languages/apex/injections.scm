; Comments
([
  (line_comment)
  (block_comment)
] @injection.content
  (#set! injection.language "comment"))

((query_expression
  (soql_query) @injection.content)
 (#set! injection.language "soql")
 (#set! injection.include-children))
