// Script de inicialización para MongoDB
// Este script crea las colecciones básicas y configura índices

print("🔧 Inicializando base de datos Bitacora...");

// Cambiar a la base de datos bitacora_db
use bitacora_db;

// Crear usuario de aplicación
db.createUser({
  user: "bitacora_app",
  pwd: "app_password_2025",
  roles: [
    { role: "readWrite", db: "bitacora_db" }
  ]
});

print("✅ Usuario de aplicación creado");

// Crear colecciones con validación de schema
db.createCollection("users", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["user_id", "username", "created_at"],
      properties: {
        user_id: { bsonType: "string" },
        username: { bsonType: "string" },
        created_at: { bsonType: "date" }
      }
    }
  }
});

db.createCollection("projects", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["project_id", "user_id", "name", "created_at"],
      properties: {
        project_id: { bsonType: "string" },
        user_id: { bsonType: "string" },
        name: { bsonType: "string" },
        created_at: { bsonType: "date" }
      }
    }
  }
});

print("✅ Colecciones básicas creadas");

// Crear índices
db.users.createIndex({ "user_id": 1 }, { unique: true });
db.users.createIndex({ "username": 1 }, { unique: true });
db.projects.createIndex({ "project_id": 1 }, { unique: true });
db.projects.createIndex({ "user_id": 1 });

print("✅ Índices creados");
print("🎉 Base de datos inicializada correctamente");
