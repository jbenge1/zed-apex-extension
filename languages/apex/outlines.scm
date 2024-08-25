(class_declaration
    "class" @context
    name: (_) @name) @item

(constructor_declaration
    name: (_) @name
    parameters: (formal_parameters
        "(" @context
        ")" @context)) @item

(method_declaration
    name: (_) @name
    parameters: (formal_parameters
      "(" @context
      ")" @context)) @item

(field_declaration
    declarator: (variable_declarator
        name: (_) @name)) @item
