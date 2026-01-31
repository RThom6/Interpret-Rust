# Helper file to quickly generate AST classes
import sys


def main(args):
    if len(args) != 1:
        print("Usage: generate_ast.py <output_directory>")
        sys.exit(1)

    define_ast(
        "../src/",
        "expr",
        [
            "Literal : Object value",
            "Grouping : Box<Expr> expression",
            "Unary : Token operator, Box<Expr> right",
            "Binary : Box<Expr> left, Token operator, Box<Expr> right",
        ],
    )


def define_ast(output_dir, base_name, types):
    with open(f"{output_dir}/{base_name}.rs", "w") as file:
        file.write(f"mod {base_name};\n")
        file.write("use crate::token::{Token}\n\n")
        file.write(f"pub enum {base_name}" + "{\n")

        for type in types:
            file.write(f"\t{type}({type}),\n")
        file.write("}")

        for type in types:
            type_split = type.split(":")
            class_name = type_split[0].trim()
            fields = type_split[1].trim()
            define_type(file, class_name, fields)


def define_type(file, class_name, field_list):
    file.write("\t#[derive(Debug,Clone)]\n")
    file.write(f"\tpub struct {class_name}")


# Expression -> Literal | Unary | Binary | Grouping
# Literal -> NUMBER | STRING | true | false | null
# Grouping -> ( Expression ) | { Expression }
# Unary -> UnaryOperator Expression
# UnaryOperator -> - | !
# Binary -> Expression Operator Expression
# Operator -> == | != | < | <= | > | >= | + | - | * | /

if __name__ == "__main__":
    main(sys.argv[1:])
