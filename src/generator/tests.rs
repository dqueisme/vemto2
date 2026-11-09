#[cfg(test)]
mod tests {
    use crate::generator::*;
    use crate::models::*;

    #[test]
    fn test_migration_generation() {
        let mut table = Table::new("posts", "Post");
        table.add_column(Column::foreign_id("user_id"));
        table.add_column(Column::string("title"));
        let mut content = Column::new("content", ColumnType::LongText);
        content.nullable = true;
        table.add_column(content);

        let (filename, code) = migration::MigrationGenerator::generate_create_table(&table, "2026_01_01_000000");
        assert_eq!(filename, "2026_01_01_000000_create_posts_table.php");
        assert!(code.contains("Schema::create('posts', function (Blueprint $table) {"));
        assert!(code.contains("$table->id();"));
        assert!(code.contains("$table->unsignedBigInteger('user_id');"));
        assert!(code.contains("$table->string('title');"));
        assert!(code.contains("$table->longText('content')->nullable();"));
        assert!(code.contains("$table->timestamps();"));
    }

    #[test]
    fn test_model_generation() {
        let project = Project::default_laravel_project();
        let posts_table = project.tables.iter().find(|t| t.name == "posts").unwrap();

        let (filename, code) = model::ModelGenerator::generate(posts_table, &project);
        assert_eq!(filename, "Post.php");
        assert!(code.contains("namespace App\\Models;"));
        assert!(code.contains("class Post extends Model"));
        assert!(code.contains("protected $table = 'posts';"));
        assert!(code.contains("'user_id',"));
        assert!(code.contains("'title',"));
        assert!(code.contains("'is_published' => 'boolean'"));
        assert!(code.contains("public function user(): BelongsTo"));
        assert!(code.contains("return $this->belongsTo(User::class);"));
    }

    #[test]
    fn test_factory_generation() {
        let project = Project::default_laravel_project();
        let users_table = project.tables.iter().find(|t| t.name == "users").unwrap();

        let (filename, code) = factory::FactoryGenerator::generate(users_table, &project);
        assert_eq!(filename, "UserFactory.php");
        assert!(code.contains("class UserFactory extends Factory"));
        assert!(code.contains("'email' => fake()->unique()->safeEmail()"));
    }

    #[test]
    fn test_controller_generation() {
        let project = Project::default_laravel_project();
        let posts_table = project.tables.iter().find(|t| t.name == "posts").unwrap();

        let (filename, code) = controller::ControllerGenerator::generate_crud(posts_table, &project);
        assert_eq!(filename, "PostController.php");
        assert!(code.contains("class PostController extends Controller"));
        assert!(code.contains("public function index(): View"));
        assert!(code.contains("public function store(Request $request): RedirectResponse"));
        assert!(code.contains("public function update(Request $request, Post $post): RedirectResponse"));
    }

    #[test]
    fn test_generate_all() {
        let project = Project::default_laravel_project();
        let files = CodeGenerator::generate_all(&project);
        // 2 tables * 4 generators = 8 files
        assert_eq!(files.len(), 8);
    }
}
