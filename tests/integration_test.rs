use vemto_core::generator::CodeGenerator;
use vemto_core::models::*;

#[test]
fn test_integration_full_code_generation_pipeline() {
    let temp_dir = std::env::temp_dir().join(format!("vemto_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));

    let mut project = Project::new("ECommerce");
    project.namespace = "App".to_string();

    let mut categories = Table::new("categories", "Category");
    categories.add_column(Column::string("name"));
    let mut slug = Column::string("slug");
    slug.unique = true;
    categories.add_column(slug);

    let mut products = Table::new("products", "Product");
    products.add_column(Column::foreign_id("category_id"));
    products.add_column(Column::string("name"));
    let mut price = Column::new("price", ColumnType::Decimal);
    price.default_value = Some("0.00".into());
    products.add_column(price);
    products.add_column(Column::new("description", ColumnType::Text));

    categories.relationships.push(Relationship {
        name: "products".into(),
        rel_type: RelationType::HasMany,
        target_model: "Product".into(),
        foreign_key: Some("category_id".into()),
        local_key: Some("id".into()),
    });

    products.relationships.push(Relationship {
        name: "category".into(),
        rel_type: RelationType::BelongsTo,
        target_model: "Category".into(),
        foreign_key: Some("category_id".into()),
        local_key: Some("id".into()),
    });

    project.tables.push(categories);
    project.tables.push(products);

    // Save project to json
    let json_path = temp_dir.join("project.json");
    project.save_to_file(&json_path).expect("Failed to save project json");

    let loaded = Project::load_from_file(&json_path).expect("Failed to load project json");
    assert_eq!(project, loaded);

    // Write all Laravel files
    let export_dir = temp_dir.join("laravel_app");
    let count = CodeGenerator::write_to_directory(&project, &export_dir).expect("Failed to generate code");
    assert_eq!(count, 8); // 2 tables * (Migration + Model + Factory + Controller)

    // Verify written files exist
    assert!(export_dir.join("app/Models/Category.php").exists());
    assert!(export_dir.join("app/Models/Product.php").exists());
    assert!(export_dir.join("app/Http/Controllers/CategoryController.php").exists());
    assert!(export_dir.join("app/Http/Controllers/ProductController.php").exists());
    assert!(export_dir.join("database/factories/CategoryFactory.php").exists());
    assert!(export_dir.join("database/factories/ProductFactory.php").exists());

    let product_model = std::fs::read_to_string(export_dir.join("app/Models/Product.php")).unwrap();
    assert!(product_model.contains("public function category(): BelongsTo"));
    assert!(product_model.contains("'price' => 'decimal:2'"));

    // Clean up
    let _ = std::fs::remove_dir_all(&temp_dir);
}
