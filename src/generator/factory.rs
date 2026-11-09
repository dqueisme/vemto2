use crate::models::{ColumnType, Project, Table};
use heck::ToPascalCase;

pub struct FactoryGenerator;

impl FactoryGenerator {
    pub fn generate(table: &Table, project: &Project) -> (String, String) {
        let model_name = if table.model_name.is_empty() {
            table.name.to_pascal_case()
        } else {
            table.model_name.clone()
        };

        let filename = format!("{}Factory.php", model_name);

        let mut defs = Vec::new();

        for col in &table.columns {
            if col.col_type == ColumnType::Id || col.col_type == ColumnType::BigIncrements {
                continue;
            }
            if col.name == "remember_token" {
                defs.push(format!("            '{}' => Str::random(10),", col.name));
                continue;
            }

            let faker_expr = match &col.col_type {
                ColumnType::String => {
                    if col.name.contains("email") {
                        "fake()->unique()->safeEmail()".to_string()
                    } else if col.name.contains("name") {
                        "fake()->name()".to_string()
                    } else if col.name.contains("password") {
                        "static::$password ??= Hash::make('password')".to_string()
                    } else if col.name.contains("title") {
                        "fake()->sentence(4)".to_string()
                    } else if col.name.contains("slug") {
                        "fake()->slug()".to_string()
                    } else {
                        "fake()->word()".to_string()
                    }
                }
                ColumnType::Text | ColumnType::LongText => "fake()->paragraphs(3, true)".to_string(),
                ColumnType::Integer | ColumnType::BigInteger | ColumnType::SmallInteger | ColumnType::TinyInteger => {
                    "fake()->randomNumber()".to_string()
                }
                ColumnType::UnsignedBigInteger | ColumnType::UnsignedInteger => {
                    if col.name.ends_with("_id") {
                        "1".to_string()
                    } else {
                        "fake()->randomNumber()".to_string()
                    }
                }
                ColumnType::Boolean => "fake()->boolean()".to_string(),
                ColumnType::Decimal | ColumnType::Float | ColumnType::Double => "fake()->randomFloat(2, 10, 100)".to_string(),
                ColumnType::Date => "fake()->date()".to_string(),
                ColumnType::DateTime | ColumnType::Timestamp => "fake()->dateTime()".to_string(),
                ColumnType::Time => "fake()->time()".to_string(),
                ColumnType::Json => "[]".to_string(),
                ColumnType::Uuid => "fake()->uuid()".to_string(),
                _ => continue,
            };

            defs.push(format!("            '{}' => {},", col.name, faker_expr));
        }

        let defs_str = defs.join("\n");

        let code = format!(
r#"<?php

namespace Database\Factories;

use {namespace}\Models\{model_name};
use Illuminate\Database\Eloquent\Factories\Factory;
use Illuminate\Support\Facades\Hash;
use Illuminate\Support\Str;

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\{namespace}\Models\{model_name}>
 */
class {model_name}Factory extends Factory
{{
    /**
     * The name of the factory's corresponding model.
     *
     * @var class-string<\{namespace}\Models\{model_name}>
     */
    protected $model = {model_name}::class;

    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    public function definition(): array
    {{
        return [
{defs_str}
        ];
    }}
}}
"#,
            namespace = project.namespace,
            model_name = model_name,
            defs_str = defs_str
        );

        (filename, code)
    }
}
