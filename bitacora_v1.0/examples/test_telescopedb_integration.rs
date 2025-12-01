//! # Test TelescopeDB Integration
//!
//! Pruebas de integración para TelescopeDB

use bitacora::telescopedb::TelescopeDB;
use std::path::PathBuf;
use std::fs;

/// Test 1: Creación de base de datos
#[test]
fn test_telescopedb_creation() {
    println!("\nTEST 1: Creación de TelescopeDB");
    
    let test_dir = PathBuf::from("/tmp/telescopedb_test_01");
    let _ = fs::remove_dir_all(&test_dir);
    
    let db = TelescopeDB::new(test_dir.clone()).expect("Failed to create TelescopeDB");
    
    assert!(test_dir.exists());
    println!("✅ TelescopeDB creada en: {:?}", test_dir);
    
    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

/// Test 2: Métodos básicos
#[test]
fn test_basic_methods() {
    println!("\nTEST 2: Métodos básicos de TelescopeDB");
    
    let test_dir = PathBuf::from("/tmp/telescopedb_test_02");
    let _ = fs::remove_dir_all(&test_dir);
    
    let db = TelescopeDB::new(test_dir.clone()).expect("Failed to create DB");
    
    // Verificar métricas
    let metrics = db.metrics();
    assert_eq!(metrics.total_cores, 0);
    assert_eq!(metrics.total_queries, 0);
    
    println!("✅ Cores iniciales: {}", metrics.total_cores);
    println!("✅ Queries iniciales: {}", metrics.total_queries);
    
    // Verificar tamaño
    assert_eq!(db.len(), 0);
    assert!(db.is_empty());
    
    println!("✅ DB está vacía: {}", db.is_empty());
    
    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

/// Test 3: Múltiples instancias
#[test]
fn test_multiple_instances() {
    println!("\nTEST 3: Múltiples instancias de TelescopeDB");
    
    let dirs = vec![
        PathBuf::from("/tmp/telescopedb_multi_01"),
        PathBuf::from("/tmp/telescopedb_multi_02"),
        PathBuf::from("/tmp/telescopedb_multi_03"),
    ];
    
    for dir in &dirs {
        let _ = fs::remove_dir_all(dir);
        let db = TelescopeDB::new(dir.clone()).expect("Failed to create DB");
        assert!(dir.exists());
        println!("   DB creada en: {:?}", dir);
    }
    
    println!("✅ {} instancias creadas", dirs.len());
    
    // Cleanup
    for dir in &dirs {
        let _ = fs::remove_dir_all(dir);
    }
}

/// Test 4: Persistencia del directorio
#[test]
fn test_directory_persistence() {
    println!("\nTEST 4: Persistencia del directorio");
    
    let test_dir = PathBuf::from("/tmp/telescopedb_test_04");
    let _ = fs::remove_dir_all(&test_dir);
    
    // Crear primera vez
    {
        let _db = TelescopeDB::new(test_dir.clone()).expect("Failed to create DB");
        assert!(test_dir.exists());
        println!("   Primera creación: OK");
    }
    
    // Reabrir (el directorio persiste)
    {
        let _db = TelescopeDB::new(test_dir.clone()).expect("Failed to reopen DB");
        assert!(test_dir.exists());
        println!("   Reapertura: OK");
    }
    
    println!("✅ Directorio persiste entre instancias");
    
    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

/// Test 5: Métricas iniciales
#[test]
fn test_initial_metrics() {
    println!("\nTEST 5: Métricas iniciales");
    
    let test_dir = PathBuf::from("/tmp/telescopedb_test_05");
    let _ = fs::remove_dir_all(&test_dir);
    
    let db = TelescopeDB::new(test_dir.clone()).expect("Failed to create DB");
    let metrics = db.metrics();
    
    println!("   Total cores: {}", metrics.total_cores);
    println!("   Total queries: {}", metrics.total_queries);
    println!("   Avg query time: {:.2} ms", metrics.avg_query_time_ms);
    println!("   Compression ratio: {:.2}x", metrics.compression_ratio);
    
    assert_eq!(metrics.total_cores, 0);
    assert_eq!(metrics.total_queries, 0);
    
    println!("✅ Métricas inicializadas correctamente");
    
    // Cleanup
    let _ = fs::remove_dir_all(&test_dir);
}

fn main() {
    println!("\n🔭 TelescopeDB Integration Test Suite");
    println!("═══════════════════════════════════════════════════");
    println!("ℹ️  Ejecuta: cargo test --example test_telescopedb_integration");
    println!("═══════════════════════════════════════════════════\n");
}
