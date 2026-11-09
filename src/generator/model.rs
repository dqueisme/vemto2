use crate::models::{ColumnType, Project, RelationType, Table};
use heck::ToPascalCase;

pub struct ModelGenerator;

impl ModelGenerator {
    pub fn generate(table: &Table, project: &Project) -> (String, String) {
        let model_name = if table.model_name.is_empty() {
            table.name.to_pascal_case()
        } else {
            table.model_name.clone()
        };

        let filename = format!("{}.php", model_name);

        let mut traits = vec!["use HasFactory;".to_string()];
        let mut trait_imports = vec!["use Illuminate\\Database\\Eloquent\\Factories\\HasFactory;".to_string()];

        if table.soft_deletes {
            traits.push("use SoftDeletes;".to_string());
            trait_imports.push("use Illuminate\\Database\\Eloquent\\SoftDeletes;".to_string());
        }

        // Fillable attributes
        let fillable_columns: Vec<String> = table
            .columns
            .iter()
            .filter(|c| c.col_type != ColumnType::Id && c.col_type != ColumnType::BigIncrements)
            .map(|c| format!("'{}'", c.name))
            .collect();

        // Casts attributes
        let cast_entries: Vec<String> = table
            .columns
            .iter()
            .filter_map(|c| {
                c.col_type
                    .to_eloquent_cast()
                    .map(|cast| format!("        '{}' => {},", c.name, cast))
            })
            .collect();

        // Relations methods
        let mut relation_methods = Vec::new();
        let mut has_relation_imports = false;

        for rel in &table.relationships {
            has_relation_imports = true;
            let target_class = rel.target_model.to_pascal_case();
            let method_name = &rel.name;

            let method_body = match rel.rel_type {
                RelationType::HasOne => {
                    format!("        return $this->hasOne({}::class);", target_class)
                }
                RelationType::HasMany => {
                    format!("        return $this->hasMany({}::class);", target_class)
                }
                RelationType::BelongsTo => {
                    format!("        return $this->belongsTo({}::class);", target_class)
                }
                RelationType::BelongsToMany => {
                    format!("        return $this->belongsToMany({}::class);", target_class)
                }
            };

            let return_type = match rel.rel_type {
                RelationType::HasOne => "HasOne",
                RelationType::HasMany => "HasMany",
                RelationType::BelongsTo => "BelongsTo",
                RelationType::BelongsToMany => "BelongsToMany",
            };

            relation_methods.push(format!(
r#"    public function {method_name}(): {return_type}
    {{
{method_body}
    }}"#,
                method_name = method_name,
                return_type = return_type,
                method_body = method_body
            ));
        }

        if has_relation_imports {
            trait_imports.push("use Illuminate\\Database\\Eloquent\\Relations\\HasOne;".to_string());
            trait_imports.push("use Illuminate\\Database\\Eloquent\\Relations\\HasMany;".to_string());
            trait_imports.push("use Illuminate\\Database\\Eloquent\\Relations\\BelongsTo;".to_string());
            trait_imports.push("use Illuminate\\Database\\Eloquent\\Relations\\BelongsToMany;".to_string());
        }

        trait_imports.sort();
        trait_imports.dedup();

        let imports_str = trait_imports.join("\n");
        let traits_str = traits.iter().map(|t| format!("    {}", t)).collect::<Vec<_>>().join("\n");

        let fillable_str = if fillable_columns.is_empty() {
            "    protected $fillable = [];".to_string()
        } else {
            format!(
                "    protected $fillable = [\n        {},\n    ];",
                fillable_columns.join(",\n        ")
            )
        };

        let casts_str = if cast_entries.is_empty() {
            "    protected $casts = [];".to_string()
        } else {
            format!(
                "    protected $casts = [\n{}\n    ];",
                cast_entries.join("\n")
            )
        };

        let relations_str = if relation_methods.is_empty() {
            String::new()
        } else {
            format!("\n\n{}", relation_methods.join("\n\n"))
        };

        let code = format!(
r#"<?php

namespace {namespace}\Models;

use Illuminate\Database\Eloquent\Model;
{imports_str}

class {model_name} extends Model
{{
{traits_str}

    protected $table = '{table_name}';

{fillable_str}

{casts_str}{relations_str}
}}
"#,
            namespace = project.namespace,
            imports_str = imports_str,
            model_name = model_name,
            traits_str = traits_str,
            table_name = table.name,
            fillable_str = fillable_str,
            casts_str = casts_str,
            relations_str = relations_str
        );

        (filename, code)
    }
}
