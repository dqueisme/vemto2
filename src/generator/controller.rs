use crate::models::{ColumnType, Project, Table};
use heck::{ToLowerCamelCase, ToPascalCase, ToSnakeCase};

pub struct ControllerGenerator;

impl ControllerGenerator {
    pub fn generate_crud(table: &Table, project: &Project) -> (String, String) {
        let model_name = if table.model_name.is_empty() {
            table.name.to_pascal_case()
        } else {
            table.model_name.clone()
        };

        let controller_name = format!("{}Controller", model_name);
        let filename = format!("{}.php", controller_name);

        let var_single = model_name.to_lower_camel_case();
        let var_plural = table.name.to_lower_camel_case();
        let view_path = table.name.to_snake_case();

        // Build validation rules for store/update
        let mut rules = Vec::new();
        for col in &table.columns {
            if col.col_type == ColumnType::Id || col.col_type == ColumnType::BigIncrements {
                continue;
            }
            if col.name == "remember_token" || col.name == "email_verified_at" {
                continue;
            }

            let mut col_rules: Vec<String> = Vec::new();
            if col.nullable {
                col_rules.push("'nullable'".to_string());
            } else {
                col_rules.push("'required'".to_string());
            }

            match &col.col_type {
                ColumnType::String => {
                    col_rules.push("'string'".to_string());
                    if let Some(len) = col.length {
                        col_rules.push(format!("'max:{}'", len));
                    } else {
                        col_rules.push("'max:255'".to_string());
                    }
                    if col.name.contains("email") {
                        col_rules.push("'email'".to_string());
                    }
                }
                ColumnType::Text | ColumnType::LongText => {
                    col_rules.push("'string'".to_string());
                }
                ColumnType::Integer | ColumnType::BigInteger | ColumnType::SmallInteger | ColumnType::TinyInteger => {
                    col_rules.push("'integer'".to_string());
                }
                ColumnType::UnsignedBigInteger | ColumnType::UnsignedInteger => {
                    col_rules.push("'integer'".to_string());
                    col_rules.push("'min:1'".to_string());
                }
                ColumnType::Boolean => {
                    col_rules.push("'boolean'".to_string());
                }
                ColumnType::Decimal | ColumnType::Float | ColumnType::Double => {
                    col_rules.push("'numeric'".to_string());
                }
                ColumnType::Date => {
                    col_rules.push("'date'".to_string());
                }
                ColumnType::DateTime | ColumnType::Timestamp => {
                    col_rules.push("'date'".to_string());
                }
                ColumnType::Json => {
                    col_rules.push("'array'".to_string());
                }
                _ => {}
            }

            rules.push(format!("            '{}' => [{}],", col.name, col_rules.join(", ")));
        }

        let rules_str = rules.join("\n");

        let code = format!(
r#"<?php

namespace {namespace}\Http\Controllers;

use {namespace}\Models\{model_name};
use Illuminate\Http\Request;
use Illuminate\Http\RedirectResponse;
use Illuminate\View\View;

class {controller_name} extends Controller
{{
    /**
     * Display a listing of the resource.
     */
    public function index(): View
    {{
        ${var_plural} = {model_name}::latest()->paginate(15);

        return view('{view_path}.index', compact('{var_plural}'));
    }}

    /**
     * Show the form for creating a new resource.
     */
    public function create(): View
    {{
        return view('{view_path}.create');
    }}

    /**
     * Store a newly created resource in storage.
     */
    public function store(Request $request): RedirectResponse
    {{
        $validated = $request->validate([
{rules_str}
        ]);

        {model_name}::create($validated);

        return redirect()->route('{view_path}.index')
            ->with('success', '{model_name} created successfully.');
    }}

    /**
     * Display the specified resource.
     */
    public function show({model_name} ${var_single}): View
    {{
        return view('{view_path}.show', compact('{var_single}'));
    }}

    /**
     * Show the form for editing the specified resource.
     */
    public function edit({model_name} ${var_single}): View
    {{
        return view('{view_path}.edit', compact('{var_single}'));
    }}

    /**
     * Update the specified resource in storage.
     */
    public function update(Request $request, {model_name} ${var_single}): RedirectResponse
    {{
        $validated = $request->validate([
{rules_str}
        ]);

        ${var_single}->update($validated);

        return redirect()->route('{view_path}.index')
            ->with('success', '{model_name} updated successfully.');
    }}

    /**
     * Remove the specified resource from storage.
     */
    public function destroy({model_name} ${var_single}): RedirectResponse
    {{
        ${var_single}->delete();

        return redirect()->route('{view_path}.index')
            ->with('success', '{model_name} deleted successfully.');
    }}
}}
"#,
            namespace = project.namespace,
            model_name = model_name,
            controller_name = controller_name,
            var_single = var_single,
            var_plural = var_plural,
            view_path = view_path,
            rules_str = rules_str
        );

        (filename, code)
    }
}
