use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    Id,
    BigIncrements,
    String,
    Text,
    LongText,
    Integer,
    BigInteger,
    SmallInteger,
    TinyInteger,
    UnsignedBigInteger,
    UnsignedInteger,
    Boolean,
    Decimal,
    Float,
    Double,
    Date,
    DateTime,
    Timestamp,
    Time,
    Json,
    Uuid,
    Binary,
}

impl ColumnType {
    pub fn all() -> Vec<ColumnType> {
        vec![
            ColumnType::Id,
            ColumnType::BigIncrements,
            ColumnType::String,
            ColumnType::Text,
            ColumnType::LongText,
            ColumnType::Integer,
            ColumnType::BigInteger,
            ColumnType::SmallInteger,
            ColumnType::TinyInteger,
            ColumnType::UnsignedBigInteger,
            ColumnType::UnsignedInteger,
            ColumnType::Boolean,
            ColumnType::Decimal,
            ColumnType::Float,
            ColumnType::Double,
            ColumnType::Date,
            ColumnType::DateTime,
            ColumnType::Timestamp,
            ColumnType::Time,
            ColumnType::Json,
            ColumnType::Uuid,
            ColumnType::Binary,
        ]
    }

    pub fn to_migration_method(&self) -> &'static str {
        match self {
            ColumnType::Id => "id",
            ColumnType::BigIncrements => "bigIncrements",
            ColumnType::String => "string",
            ColumnType::Text => "text",
            ColumnType::LongText => "longText",
            ColumnType::Integer => "integer",
            ColumnType::BigInteger => "bigInteger",
            ColumnType::SmallInteger => "smallInteger",
            ColumnType::TinyInteger => "tinyInteger",
            ColumnType::UnsignedBigInteger => "unsignedBigInteger",
            ColumnType::UnsignedInteger => "unsignedInteger",
            ColumnType::Boolean => "boolean",
            ColumnType::Decimal => "decimal",
            ColumnType::Float => "float",
            ColumnType::Double => "double",
            ColumnType::Date => "date",
            ColumnType::DateTime => "dateTime",
            ColumnType::Timestamp => "timestamp",
            ColumnType::Time => "time",
            ColumnType::Json => "json",
            ColumnType::Uuid => "uuid",
            ColumnType::Binary => "binary",
        }
    }

    pub fn to_php_type(&self) -> &'static str {
        match self {
            ColumnType::Id | ColumnType::BigIncrements | ColumnType::Integer | ColumnType::BigInteger |
            ColumnType::SmallInteger | ColumnType::TinyInteger | ColumnType::UnsignedBigInteger |
            ColumnType::UnsignedInteger => "int",
            ColumnType::String | ColumnType::Text | ColumnType::LongText | ColumnType::Uuid => "string",
            ColumnType::Boolean => "bool",
            ColumnType::Decimal | ColumnType::Float | ColumnType::Double => "float",
            ColumnType::Date | ColumnType::DateTime | ColumnType::Timestamp => "\\Illuminate\\Support\\Carbon",
            ColumnType::Time => "string",
            ColumnType::Json => "array",
            ColumnType::Binary => "string",
        }
    }

    pub fn to_eloquent_cast(&self) -> Option<&'static str> {
        match self {
            ColumnType::Boolean => Some("'boolean'"),
            ColumnType::Date => Some("'date'"),
            ColumnType::DateTime | ColumnType::Timestamp => Some("'datetime'"),
            ColumnType::Decimal | ColumnType::Float | ColumnType::Double => Some("'decimal:2'"),
            ColumnType::Json => Some("'array'"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub unique: bool,
    pub comment: Option<String>,
    pub length: Option<u32>,
}

impl Column {
    pub fn new(name: impl Into<String>, col_type: ColumnType) -> Self {
        Self {
            name: name.into(),
            col_type,
            nullable: false,
            default_value: None,
            unique: false,
            comment: None,
            length: None,
        }
    }

    pub fn id() -> Self {
        Self::new("id", ColumnType::Id)
    }

    pub fn string(name: impl Into<String>) -> Self {
        Self::new(name, ColumnType::String)
    }

    pub fn foreign_id(name: impl Into<String>) -> Self {
        Self::new(name, ColumnType::UnsignedBigInteger)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationType {
    HasOne,
    HasMany,
    BelongsTo,
    BelongsToMany,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relationship {
    pub name: String,
    pub rel_type: RelationType,
    pub target_model: String,
    pub foreign_key: Option<String>,
    pub local_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub model_name: String,
    pub columns: Vec<Column>,
    pub relationships: Vec<Relationship>,
    pub timestamps: bool,
    pub soft_deletes: bool,
}

impl Table {
    pub fn new(name: impl Into<String>, model_name: impl Into<String>) -> Self {
        let mut t = Self {
            name: name.into(),
            model_name: model_name.into(),
            columns: Vec::new(),
            relationships: Vec::new(),
            timestamps: true,
            soft_deletes: false,
        };
        t.columns.push(Column::id());
        t
    }

    pub fn add_column(&mut self, col: Column) {
        self.columns.push(col);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub namespace: String,
    pub tables: Vec<Table>,
}

#[cfg(test)]
mod tests;

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: "App".to_string(),
            tables: Vec::new(),
        }
    }

    pub fn default_laravel_project() -> Self {
        let mut project = Self::new("Laravel App");

        // users table
        let mut users = Table::new("users", "User");
        users.add_column(Column::string("name"));
        let mut email_col = Column::string("email");
        email_col.unique = true;
        users.add_column(email_col);
        let mut email_verified = Column::new("email_verified_at", ColumnType::Timestamp);
        email_verified.nullable = true;
        users.add_column(email_verified);
        users.add_column(Column::string("password"));
        let mut remember_token = Column::string("remember_token");
        remember_token.nullable = true;
        remember_token.length = Some(100);
        users.add_column(remember_token);

        // posts table
        let mut posts = Table::new("posts", "Post");
        posts.add_column(Column::foreign_id("user_id"));
        posts.add_column(Column::string("title"));
        posts.add_column(Column::string("slug"));
        posts.add_column(Column::new("content", ColumnType::LongText));
        let mut is_published = Column::new("is_published", ColumnType::Boolean);
        is_published.default_value = Some("false".into());
        posts.add_column(is_published);

        posts.relationships.push(Relationship {
            name: "user".into(),
            rel_type: RelationType::BelongsTo,
            target_model: "User".into(),
            foreign_key: Some("user_id".into()),
            local_key: Some("id".into()),
        });

        users.relationships.push(Relationship {
            name: "posts".into(),
            rel_type: RelationType::HasMany,
            target_model: "Post".into(),
            foreign_key: Some("user_id".into()),
            local_key: Some("id".into()),
        });

        project.tables.push(users);
        project.tables.push(posts);
        project
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json = self.to_json().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, json)
    }

    pub fn load_from_file(path: &std::path::Path) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Self::from_json(&json).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}
