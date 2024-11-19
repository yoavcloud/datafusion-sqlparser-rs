// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#[macro_use]
mod test_utils;

use test_utils::*;

use sqlparser::ast::*;
use sqlparser::dialect::GenericDialect;
use sqlparser::dialect::RedshiftSqlDialect;

#[test]
fn test_square_brackets_over_db_schema_table_name() {
    let select = redshift().verified_only_select("SELECT [col1] FROM [test_schema].[test_table]");
    assert_eq!(
        select.projection[0],
        SelectItem::UnnamedExpr(Expr::Identifier(Ident {
            value: "col1".to_string(),
            quote_style: Some('[')
        })),
    );
    assert_eq!(
        select.from[0],
        TableWithJoins {
            relation: TableFactor::Table {
                name: ObjectName(vec![
                    Ident {
                        value: "test_schema".to_string(),
                        quote_style: Some('[')
                    },
                    Ident {
                        value: "test_table".to_string(),
                        quote_style: Some('[')
                    }
                ]),
                alias: None,
                args: None,
                with_hints: vec![],
                version: None,
                partitions: vec![],
                with_ordinality: false,
                partiql: None,
            },
            joins: vec![],
        }
    );
}

#[test]
fn brackets_over_db_schema_table_name_with_whites_paces() {
    match redshift().parse_sql_statements("SELECT [   col1  ] FROM [  test_schema].[ test_table]") {
        Ok(statements) => {
            assert_eq!(statements.len(), 1);
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_double_quotes_over_db_schema_table_name() {
    let select =
        redshift().verified_only_select("SELECT \"col1\" FROM \"test_schema\".\"test_table\"");
    assert_eq!(
        select.projection[0],
        SelectItem::UnnamedExpr(Expr::Identifier(Ident {
            value: "col1".to_string(),
            quote_style: Some('"')
        })),
    );
    assert_eq!(
        select.from[0],
        TableWithJoins {
            relation: TableFactor::Table {
                name: ObjectName(vec![
                    Ident {
                        value: "test_schema".to_string(),
                        quote_style: Some('"')
                    },
                    Ident {
                        value: "test_table".to_string(),
                        quote_style: Some('"')
                    }
                ]),
                alias: None,
                args: None,
                with_hints: vec![],
                version: None,
                partitions: vec![],
                with_ordinality: false,
                partiql: None,
            },
            joins: vec![],
        }
    );
}

#[test]
fn parse_delimited_identifiers() {
    // check that quoted identifiers in any position remain quoted after serialization
    let select = redshift().verified_only_select(
        r#"SELECT "alias"."bar baz", "myfun"(), "simple id" AS "column alias" FROM "a table" AS "alias""#,
    );
    // check FROM
    match only(select.from).relation {
        TableFactor::Table {
            name,
            alias,
            args,
            with_hints,
            version,
            with_ordinality: _,
            partitions: _,
            partiql: _,
        } => {
            assert_eq!(vec![Ident::with_quote('"', "a table")], name.0);
            assert_eq!(Ident::with_quote('"', "alias"), alias.unwrap().name);
            assert!(args.is_none());
            assert!(with_hints.is_empty());
            assert!(version.is_none());
        }
        _ => panic!("Expecting TableFactor::Table"),
    }
    // check SELECT
    assert_eq!(3, select.projection.len());
    assert_eq!(
        &Expr::CompoundIdentifier(vec![
            Ident::with_quote('"', "alias"),
            Ident::with_quote('"', "bar baz"),
        ]),
        expr_from_projection(&select.projection[0]),
    );
    assert_eq!(
        &Expr::Function(Function {
            name: ObjectName(vec![Ident::with_quote('"', "myfun")]),
            parameters: FunctionArguments::None,
            args: FunctionArguments::List(FunctionArgumentList {
                duplicate_treatment: None,
                args: vec![],
                clauses: vec![],
            }),
            null_treatment: None,
            filter: None,
            over: None,
            within_group: vec![],
        }),
        expr_from_projection(&select.projection[1]),
    );
    match &select.projection[2] {
        SelectItem::ExprWithAlias { expr, alias } => {
            assert_eq!(&Expr::Identifier(Ident::with_quote('"', "simple id")), expr);
            assert_eq!(&Ident::with_quote('"', "column alias"), alias);
        }
        _ => panic!("Expected ExprWithAlias"),
    }

    redshift().verified_stmt(r#"CREATE TABLE "foo" ("bar" "int")"#);
    redshift().verified_stmt(r#"ALTER TABLE foo ADD CONSTRAINT "bar" PRIMARY KEY (baz)"#);
    //TODO verified_stmt(r#"UPDATE foo SET "bar" = 5"#);
}

fn redshift() -> TestedDialects {
    TestedDialects::new(vec![Box::new(RedshiftSqlDialect {})])
}

fn redshift_and_generic() -> TestedDialects {
    TestedDialects::new(vec![
        Box::new(RedshiftSqlDialect {}),
        Box::new(GenericDialect {}),
    ])
}

#[test]
fn test_sharp() {
    let sql = "SELECT #_of_values";
    let select = redshift().verified_only_select(sql);
    assert_eq!(
        SelectItem::UnnamedExpr(Expr::Identifier(Ident::new("#_of_values"))),
        select.projection[0]
    );
}

#[test]
fn test_create_view_with_no_schema_binding() {
    redshift_and_generic()
        .verified_stmt("CREATE VIEW myevent AS SELECT eventname FROM event WITH NO SCHEMA BINDING");
}

#[test]
fn test_redshift_json_path() {
    let sql = "SELECT cust.c_orders[0].o_orderkey FROM customer_orders_lineitem";
    let select = redshift().verified_only_select(sql);

    assert_eq!(
        &Expr::JsonAccess{
            value: Box::new(Expr::CompoundIdentifier(vec![
                Ident::new("cust"),
                Ident::new("c_orders")
            ])),
            path: JsonPath{
                path: vec![
                    JsonPathElem::Bracket{key: Expr::Value(Value::Number("0".to_string(), false))},
                    JsonPathElem::Dot{key: "o_orderkey".to_string(), quoted: false}
                ]
            }
        },
        expr_from_projection(only(&select.projection))
    );

    let sql = "SELECT cust.c_orders[0]['id'] FROM customer_orders_lineitem";
    let select = redshift().verified_only_select(sql);
    assert_eq!(
        &Expr::JsonAccess{
            value: Box::new(Expr::CompoundIdentifier(vec![
                Ident::new("cust"),
                Ident::new("c_orders")
            ])),
            path: JsonPath{
                path: vec![
                    JsonPathElem::Bracket{key: Expr::Value(Value::Number("0".to_string(), false))},
                    JsonPathElem::Bracket{key: Expr::Value(Value::SingleQuotedString("id".to_owned()))}
                ]
            }
        },
        expr_from_projection(only(&select.projection))
    );
}

#[test]
fn test_parse_json_path_from() {
    let select = redshift().verified_only_select("SELECT * FROM src[0].a AS a");
    match &select.from[0].relation {
        TableFactor::Table { name, partiql, .. } => {
            assert_eq!(name, &ObjectName(vec![Ident::new("src")]));
            assert_eq!(
                partiql,
                &Some(JsonPath{
                    path: vec![
                        JsonPathElem::Bracket{key: Expr::Value(Value::Number("0".to_string(), false))},
                        JsonPathElem::Dot{key: "a".to_string(), quoted: false}
                    ]
                })
            );
        }
        _ => panic!(),
    }

    let select = redshift().verified_only_select("SELECT * FROM src[0].a[1].b AS a");
    match &select.from[0].relation {
        TableFactor::Table { name, partiql, .. } => {
            assert_eq!(name, &ObjectName(vec![Ident::new("src")]));
            assert_eq!(
                partiql,
                &Some(JsonPath{
                    path: vec![
                        JsonPathElem::Bracket{key: Expr::Value(Value::Number("0".to_string(), false))},
                        JsonPathElem::Dot{key: "a".to_string(), quoted: false},
                        JsonPathElem::Bracket{key: Expr::Value(Value::Number("1".to_string(), false))},
                        JsonPathElem::Dot{key: "b".to_string(), quoted: false},
                    ]
                })
            );
        }
        _ => panic!(),
    }

    let select = redshift().verified_only_select("SELECT * FROM src.a.b");
    match &select.from[0].relation {
        TableFactor::Table { name, partiql, .. } => {
            assert_eq!(name, &ObjectName(vec![Ident::new("src"), Ident::new("a"), Ident::new("b")]));
            assert_eq!(partiql, &None);
        }
        _ => panic!(),
    }

}