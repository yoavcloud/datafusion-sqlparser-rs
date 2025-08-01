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

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "visitor")]
use sqlparser_derive::{Visit, VisitMut};

use super::super::dml::CreateTable;
use crate::ast::{
    ClusteredBy, ColumnDef, CommentDef, CreateTableOptions, Expr, FileFormat,
    HiveDistributionStyle, HiveFormat, Ident, ObjectName, OnCommit, OneOrManyWithParens, Query,
    RowAccessPolicy, Statement, StorageSerializationPolicy, TableConstraint, Tag,
    WrappedCollection,
};

use crate::parser::ParserError;

/// Builder for create table statement variant ([1]).
///
/// This structure helps building and accessing a create table with more ease, without needing to:
/// - Match the enum itself a lot of times; or
/// - Moving a lot of variables around the code.
///
/// # Example
/// ```rust
/// use sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder;
/// use sqlparser::ast::{ColumnDef, DataType, Ident, ObjectName};
/// let builder = CreateTableBuilder::new(ObjectName::from(vec![Ident::new("table_name")]))
///    .if_not_exists(true)
///    .columns(vec![ColumnDef {
///        name: Ident::new("c1"),
///        data_type: DataType::Int(None),
///        options: vec![],
/// }]);
/// // You can access internal elements with ease
/// assert!(builder.if_not_exists);
/// // Convert to a statement
/// assert_eq!(
///    builder.build().to_string(),
///    "CREATE TABLE IF NOT EXISTS table_name (c1 INT)"
/// )
/// ```
///
/// [1]: crate::ast::Statement::CreateTable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "visitor", derive(Visit, VisitMut))]
pub struct CreateTableBuilder {
    pub or_replace: bool,
    pub temporary: bool,
    pub external: bool,
    pub global: Option<bool>,
    pub if_not_exists: bool,
    pub transient: bool,
    pub volatile: bool,
    pub iceberg: bool,
    pub name: ObjectName,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<TableConstraint>,
    pub hive_distribution: HiveDistributionStyle,
    pub hive_formats: Option<HiveFormat>,
    pub file_format: Option<FileFormat>,
    pub location: Option<String>,
    pub query: Option<Box<Query>>,
    pub without_rowid: bool,
    pub like: Option<ObjectName>,
    pub clone: Option<ObjectName>,
    pub comment: Option<CommentDef>,
    pub on_commit: Option<OnCommit>,
    pub on_cluster: Option<Ident>,
    pub primary_key: Option<Box<Expr>>,
    pub order_by: Option<OneOrManyWithParens<Expr>>,
    pub partition_by: Option<Box<Expr>>,
    pub cluster_by: Option<WrappedCollection<Vec<Expr>>>,
    pub clustered_by: Option<ClusteredBy>,
    pub inherits: Option<Vec<ObjectName>>,
    pub strict: bool,
    pub copy_grants: bool,
    pub enable_schema_evolution: Option<bool>,
    pub change_tracking: Option<bool>,
    pub data_retention_time_in_days: Option<u64>,
    pub max_data_extension_time_in_days: Option<u64>,
    pub default_ddl_collation: Option<String>,
    pub with_aggregation_policy: Option<ObjectName>,
    pub with_row_access_policy: Option<RowAccessPolicy>,
    pub with_tags: Option<Vec<Tag>>,
    pub base_location: Option<String>,
    pub external_volume: Option<String>,
    pub catalog: Option<String>,
    pub catalog_sync: Option<String>,
    pub storage_serialization_policy: Option<StorageSerializationPolicy>,
    pub table_options: CreateTableOptions,
}

impl CreateTableBuilder {
    pub fn new(name: ObjectName) -> Self {
        Self {
            or_replace: false,
            temporary: false,
            external: false,
            global: None,
            if_not_exists: false,
            transient: false,
            volatile: false,
            iceberg: false,
            name,
            columns: vec![],
            constraints: vec![],
            hive_distribution: HiveDistributionStyle::NONE,
            hive_formats: None,
            file_format: None,
            location: None,
            query: None,
            without_rowid: false,
            like: None,
            clone: None,
            comment: None,
            on_commit: None,
            on_cluster: None,
            primary_key: None,
            order_by: None,
            partition_by: None,
            cluster_by: None,
            clustered_by: None,
            inherits: None,
            strict: false,
            copy_grants: false,
            enable_schema_evolution: None,
            change_tracking: None,
            data_retention_time_in_days: None,
            max_data_extension_time_in_days: None,
            default_ddl_collation: None,
            with_aggregation_policy: None,
            with_row_access_policy: None,
            with_tags: None,
            base_location: None,
            external_volume: None,
            catalog: None,
            catalog_sync: None,
            storage_serialization_policy: None,
            table_options: CreateTableOptions::None,
        }
    }
    pub fn or_replace(mut self, or_replace: bool) -> Self {
        self.or_replace = or_replace;
        self
    }

    pub fn temporary(mut self, temporary: bool) -> Self {
        self.temporary = temporary;
        self
    }

    pub fn external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    pub fn global(mut self, global: Option<bool>) -> Self {
        self.global = global;
        self
    }

    pub fn if_not_exists(mut self, if_not_exists: bool) -> Self {
        self.if_not_exists = if_not_exists;
        self
    }

    pub fn transient(mut self, transient: bool) -> Self {
        self.transient = transient;
        self
    }

    pub fn volatile(mut self, volatile: bool) -> Self {
        self.volatile = volatile;
        self
    }

    pub fn iceberg(mut self, iceberg: bool) -> Self {
        self.iceberg = iceberg;
        self
    }

    pub fn columns(mut self, columns: Vec<ColumnDef>) -> Self {
        self.columns = columns;
        self
    }

    pub fn constraints(mut self, constraints: Vec<TableConstraint>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn hive_distribution(mut self, hive_distribution: HiveDistributionStyle) -> Self {
        self.hive_distribution = hive_distribution;
        self
    }

    pub fn hive_formats(mut self, hive_formats: Option<HiveFormat>) -> Self {
        self.hive_formats = hive_formats;
        self
    }

    pub fn file_format(mut self, file_format: Option<FileFormat>) -> Self {
        self.file_format = file_format;
        self
    }
    pub fn location(mut self, location: Option<String>) -> Self {
        self.location = location;
        self
    }

    pub fn query(mut self, query: Option<Box<Query>>) -> Self {
        self.query = query;
        self
    }
    pub fn without_rowid(mut self, without_rowid: bool) -> Self {
        self.without_rowid = without_rowid;
        self
    }

    pub fn like(mut self, like: Option<ObjectName>) -> Self {
        self.like = like;
        self
    }

    // Different name to allow the object to be cloned
    pub fn clone_clause(mut self, clone: Option<ObjectName>) -> Self {
        self.clone = clone;
        self
    }

    pub fn comment_after_column_def(mut self, comment: Option<CommentDef>) -> Self {
        self.comment = comment;
        self
    }

    pub fn on_commit(mut self, on_commit: Option<OnCommit>) -> Self {
        self.on_commit = on_commit;
        self
    }

    pub fn on_cluster(mut self, on_cluster: Option<Ident>) -> Self {
        self.on_cluster = on_cluster;
        self
    }

    pub fn primary_key(mut self, primary_key: Option<Box<Expr>>) -> Self {
        self.primary_key = primary_key;
        self
    }

    pub fn order_by(mut self, order_by: Option<OneOrManyWithParens<Expr>>) -> Self {
        self.order_by = order_by;
        self
    }

    pub fn partition_by(mut self, partition_by: Option<Box<Expr>>) -> Self {
        self.partition_by = partition_by;
        self
    }

    pub fn cluster_by(mut self, cluster_by: Option<WrappedCollection<Vec<Expr>>>) -> Self {
        self.cluster_by = cluster_by;
        self
    }

    pub fn clustered_by(mut self, clustered_by: Option<ClusteredBy>) -> Self {
        self.clustered_by = clustered_by;
        self
    }

    pub fn inherits(mut self, inherits: Option<Vec<ObjectName>>) -> Self {
        self.inherits = inherits;
        self
    }

    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    pub fn copy_grants(mut self, copy_grants: bool) -> Self {
        self.copy_grants = copy_grants;
        self
    }

    pub fn enable_schema_evolution(mut self, enable_schema_evolution: Option<bool>) -> Self {
        self.enable_schema_evolution = enable_schema_evolution;
        self
    }

    pub fn change_tracking(mut self, change_tracking: Option<bool>) -> Self {
        self.change_tracking = change_tracking;
        self
    }

    pub fn data_retention_time_in_days(mut self, data_retention_time_in_days: Option<u64>) -> Self {
        self.data_retention_time_in_days = data_retention_time_in_days;
        self
    }

    pub fn max_data_extension_time_in_days(
        mut self,
        max_data_extension_time_in_days: Option<u64>,
    ) -> Self {
        self.max_data_extension_time_in_days = max_data_extension_time_in_days;
        self
    }

    pub fn default_ddl_collation(mut self, default_ddl_collation: Option<String>) -> Self {
        self.default_ddl_collation = default_ddl_collation;
        self
    }

    pub fn with_aggregation_policy(mut self, with_aggregation_policy: Option<ObjectName>) -> Self {
        self.with_aggregation_policy = with_aggregation_policy;
        self
    }

    pub fn with_row_access_policy(
        mut self,
        with_row_access_policy: Option<RowAccessPolicy>,
    ) -> Self {
        self.with_row_access_policy = with_row_access_policy;
        self
    }

    pub fn with_tags(mut self, with_tags: Option<Vec<Tag>>) -> Self {
        self.with_tags = with_tags;
        self
    }

    pub fn base_location(mut self, base_location: Option<String>) -> Self {
        self.base_location = base_location;
        self
    }

    pub fn external_volume(mut self, external_volume: Option<String>) -> Self {
        self.external_volume = external_volume;
        self
    }

    pub fn catalog(mut self, catalog: Option<String>) -> Self {
        self.catalog = catalog;
        self
    }

    pub fn catalog_sync(mut self, catalog_sync: Option<String>) -> Self {
        self.catalog_sync = catalog_sync;
        self
    }

    pub fn storage_serialization_policy(
        mut self,
        storage_serialization_policy: Option<StorageSerializationPolicy>,
    ) -> Self {
        self.storage_serialization_policy = storage_serialization_policy;
        self
    }

    pub fn table_options(mut self, table_options: CreateTableOptions) -> Self {
        self.table_options = table_options;
        self
    }

    /// Returns true if the statement has exactly one source of info on the schema of the new table.
    /// This is Snowflake-specific, some dialects allow more than one source.
    pub(crate) fn validate_schema_info(&self) -> bool {
        let mut sources = 0;
        if !self.columns.is_empty() {
            sources += 1;
        }
        if self.query.is_some() {
            sources += 1;
        }
        if self.like.is_some() {
            sources += 1;
        }
        if self.clone.is_some() {
            sources += 1;
        }

        sources == 1
    }

    pub fn build(self) -> Statement {
        Statement::CreateTable(CreateTable {
            or_replace: self.or_replace,
            temporary: self.temporary,
            external: self.external,
            global: self.global,
            if_not_exists: self.if_not_exists,
            transient: self.transient,
            volatile: self.volatile,
            iceberg: self.iceberg,
            name: self.name,
            columns: self.columns,
            constraints: self.constraints,
            hive_distribution: self.hive_distribution,
            hive_formats: self.hive_formats,
            file_format: self.file_format,
            location: self.location,
            query: self.query,
            without_rowid: self.without_rowid,
            like: self.like,
            clone: self.clone,
            comment: self.comment,
            on_commit: self.on_commit,
            on_cluster: self.on_cluster,
            primary_key: self.primary_key,
            order_by: self.order_by,
            partition_by: self.partition_by,
            cluster_by: self.cluster_by,
            clustered_by: self.clustered_by,
            inherits: self.inherits,
            strict: self.strict,
            copy_grants: self.copy_grants,
            enable_schema_evolution: self.enable_schema_evolution,
            change_tracking: self.change_tracking,
            data_retention_time_in_days: self.data_retention_time_in_days,
            max_data_extension_time_in_days: self.max_data_extension_time_in_days,
            default_ddl_collation: self.default_ddl_collation,
            with_aggregation_policy: self.with_aggregation_policy,
            with_row_access_policy: self.with_row_access_policy,
            with_tags: self.with_tags,
            base_location: self.base_location,
            external_volume: self.external_volume,
            catalog: self.catalog,
            catalog_sync: self.catalog_sync,
            storage_serialization_policy: self.storage_serialization_policy,
            table_options: self.table_options,
        })
    }
}

impl TryFrom<Statement> for CreateTableBuilder {
    type Error = ParserError;

    // As the builder can be transformed back to a statement, it shouldn't be a problem to take the
    // ownership.
    fn try_from(stmt: Statement) -> Result<Self, Self::Error> {
        match stmt {
            Statement::CreateTable(CreateTable {
                or_replace,
                temporary,
                external,
                global,
                if_not_exists,
                transient,
                volatile,
                iceberg,
                name,
                columns,
                constraints,
                hive_distribution,
                hive_formats,
                file_format,
                location,
                query,
                without_rowid,
                like,
                clone,
                comment,
                on_commit,
                on_cluster,
                primary_key,
                order_by,
                partition_by,
                cluster_by,
                clustered_by,
                inherits,
                strict,
                copy_grants,
                enable_schema_evolution,
                change_tracking,
                data_retention_time_in_days,
                max_data_extension_time_in_days,
                default_ddl_collation,
                with_aggregation_policy,
                with_row_access_policy,
                with_tags,
                base_location,
                external_volume,
                catalog,
                catalog_sync,
                storage_serialization_policy,
                table_options,
            }) => Ok(Self {
                or_replace,
                temporary,
                external,
                global,
                if_not_exists,
                transient,
                name,
                columns,
                constraints,
                hive_distribution,
                hive_formats,
                file_format,
                location,
                query,
                without_rowid,
                like,
                clone,
                comment,
                on_commit,
                on_cluster,
                primary_key,
                order_by,
                partition_by,
                cluster_by,
                clustered_by,
                inherits,
                strict,
                iceberg,
                copy_grants,
                enable_schema_evolution,
                change_tracking,
                data_retention_time_in_days,
                max_data_extension_time_in_days,
                default_ddl_collation,
                with_aggregation_policy,
                with_row_access_policy,
                with_tags,
                volatile,
                base_location,
                external_volume,
                catalog,
                catalog_sync,
                storage_serialization_policy,
                table_options,
            }),
            _ => Err(ParserError::ParserError(format!(
                "Expected create table statement, but received: {stmt}"
            ))),
        }
    }
}

/// Helper return type when parsing configuration for a `CREATE TABLE` statement.
#[derive(Default)]
pub(crate) struct CreateTableConfiguration {
    pub partition_by: Option<Box<Expr>>,
    pub cluster_by: Option<WrappedCollection<Vec<Expr>>>,
    pub inherits: Option<Vec<ObjectName>>,
    pub table_options: CreateTableOptions,
}

#[cfg(test)]
mod tests {
    use crate::ast::helpers::stmt_create_table::CreateTableBuilder;
    use crate::ast::{Ident, ObjectName, Statement};
    use crate::parser::ParserError;

    #[test]
    pub fn test_from_valid_statement() {
        let builder = CreateTableBuilder::new(ObjectName::from(vec![Ident::new("table_name")]));

        let stmt = builder.clone().build();

        assert_eq!(builder, CreateTableBuilder::try_from(stmt).unwrap());
    }

    #[test]
    pub fn test_from_invalid_statement() {
        let stmt = Statement::Commit {
            chain: false,
            end: false,
            modifier: None,
        };

        assert_eq!(
            CreateTableBuilder::try_from(stmt).unwrap_err(),
            ParserError::ParserError(
                "Expected create table statement, but received: COMMIT".to_owned()
            )
        );
    }
}
