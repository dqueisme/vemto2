use crate::models::{Column, ColumnType, Table};
use heck::ToSnakeCase;

pub struct MigrationGenerator;

impl MigrationGenerator {
    pub fn generate_create_table(table: &Table, timestamp_prefix: &str) -> (String, String) {
        let table_snake = table.name.to_snake_case();
        let filename = format!("{}_create_{}_table.php", timestamp_prefix, table_snake);

        let mut columns_code = Vec::new();

        for col in &table.columns {
            columns_code.push(Self::column_definition(col));
        }

        if table.timestamps {
            columns_code.push("            $table->timestamps();".to_string());
        }

        if table.soft_deletes {
            columns_code.push("            $table->softDeletes();".to_string());
        }

        let columns_str = columns_code.join("\n");

        let code = format!(
r#"<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{{
    /**
     * Run the migrations.
     */
    public function up(): void
    {{
        Schema::create('{table_name}', function (Blueprint $table) {{
{columns_str}
        }});
    }}

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {{
        Schema::dropIfExists('{table_name}');
    }}
}};
"#,
            table_name = table_snake,
            columns_str = columns_str
        );

        (filename, code)
    }

    fn column_definition(col: &Column) -> String {
        let mut def = match &col.col_type {
            ColumnType::Id => {
                if col.name == "id" {
                    "$table->id()".to_string()
                } else {
                    format!("$table->id('{}')", col.name)
                }
            }
            ColumnType::String => {
                if let Some(len) = col.length {
                    format!("$table->string('{}', {})", col.name, len)
                } else {
                    format!("$table->string('{}')", col.name)
                }
            }
            other => {
                format!("$table->{}('{}')", other.to_migration_method(), col.name)
            }
        };

        if col.nullable && col.col_type != ColumnType::Id {
            def.push_str("->nullable()");
        }

        if col.unique {
            def.push_str("->unique()");
        }

        if let Some(ref def_val) = col.default_value {
            if def_val == "true" || def_val == "false" || def_val.parse::<f64>().is_ok() {
                def.push_str(&format!("->default({})", def_val));
            } else {
                def.push_str(&format!("->default('{}')", def_val.replace('\'', "\\'")));
            }
        }

        if let Some(ref comment) = col.comment {
            def.push_str(&format!("->comment('{}')", comment.replace('\'', "\\'")));
        }

        format!("            {};", def)
    }
}
