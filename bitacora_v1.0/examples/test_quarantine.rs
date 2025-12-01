//! # Test 7.x.1.5 - QuarantineZone Integration Test
//!
//! **Task:** 7.x.1.5 - Tests para QuarantineZone
//! **Spec:** ROADMAP_V2/04_IMPLEMENTACION/PHASE_7X_DATA_IMPORT.md
//!
//! **Casos de prueba:**
//! - ✅ approve() workflow
//! - ✅ reject() workflow
//! - ✅ state transitions
//! - ✅ inspection básica
//! - ✅ encoding detection
//! - ✅ size validation
//! - ✅ MIME type detection
//! - ✅ expiry check

use bitacora::data_import::{QuarantineZone, QuarantineState, DataSource};
use chrono::Duration;

#[tokio::test]
async fn test_7x_1_1_quarantine_zone_creation() {
    println!("\n🧪 Test 7.x.1.1: QuarantineZone creation");
    
    let sample_data = b"[2023-01-01, 10:00:00] Eduardo: Hola mundo".to_vec();
    let quarantine = QuarantineZone::inspect(sample_data.clone(), DataSource::WhatsApp)
        .await
        .expect("Failed to create quarantine zone");
    
    assert_eq!(quarantine.source, DataSource::WhatsApp);
    assert_eq!(quarantine.raw_data, sample_data);
    assert_eq!(quarantine.state, QuarantineState::Safe);
    assert_eq!(quarantine.metadata.encoding, "UTF-8");
    assert_eq!(quarantine.metadata.line_count, Some(1));
    
    println!("  ✅ QuarantineZone created: {}", quarantine.summary());
}

#[tokio::test]
async fn test_7x_1_2_approve_workflow() {
    println!("\n🧪 Test 7.x.1.2: Approve workflow");
    
    let data = b"Test data for approval".to_vec();
    let mut quarantine = QuarantineZone::inspect(data, DataSource::Manual)
        .await
        .expect("Failed to create quarantine");
    
    assert_eq!(quarantine.state, QuarantineState::Safe);
    assert!(quarantine.is_safe());
    assert!(quarantine.is_ready_for_digestion());
    
    quarantine.approve();
    
    assert_eq!(quarantine.state, QuarantineState::Approved);
    assert!(quarantine.is_ready_for_digestion());
    
    println!("  ✅ Approved workflow: {}", quarantine.state);
}

#[tokio::test]
async fn test_7x_1_3_reject_workflow() {
    println!("\n🧪 Test 7.x.1.3: Reject workflow");
    
    let data = b"Malicious data".to_vec();
    let mut quarantine = QuarantineZone::inspect(data, DataSource::Manual)
        .await
        .expect("Failed to create quarantine");
    
    quarantine.reject("Suspicious content detected".to_string());
    
    assert_eq!(quarantine.state, QuarantineState::Rejected);
    assert!(quarantine.is_rejected());
    assert!(!quarantine.is_ready_for_digestion());
    assert_eq!(
        quarantine.metadata.rejection_reason,
        Some("Suspicious content detected".to_string())
    );
    assert!(quarantine.metadata.flags.contains(&"Suspicious content detected".to_string()));
    
    println!("  ✅ Rejected workflow: {}", quarantine.state);
}

#[tokio::test]
async fn test_7x_1_4_state_transitions() {
    println!("\n🧪 Test 7.x.1.4: State transitions");
    
    let data = b"State transition test".to_vec();
    let mut quarantine = QuarantineZone::inspect(data, DataSource::Manual)
        .await
        .expect("Failed to create quarantine");
    
    // Initial state: Safe
    assert_eq!(quarantine.state, QuarantineState::Safe);
    println!("  → Initial state: {}", quarantine.state);
    
    // Transition: Safe → Approved
    quarantine.approve();
    assert_eq!(quarantine.state, QuarantineState::Approved);
    println!("  → After approve: {}", quarantine.state);
    
    // Transition: Approved → Rejected (can reject even after approval if suspicious)
    quarantine.reject("Late detection of issue".to_string());
    assert_eq!(quarantine.state, QuarantineState::Rejected);
    println!("  → After reject: {}", quarantine.state);
    
    println!("  ✅ State transitions valid");
}

#[tokio::test]
async fn test_7x_1_5_invalid_utf8() {
    println!("\n🧪 Test 7.x.1.5: Invalid UTF-8 encoding");
    
    let invalid_data = vec![0xFF, 0xFE, 0xFD, 0xFC];
    let quarantine = QuarantineZone::inspect(invalid_data, DataSource::Manual)
        .await
        .expect("Failed to create quarantine");
    
    assert_eq!(quarantine.state, QuarantineState::Suspicious);
    assert!(quarantine.is_suspicious());
    assert!(quarantine.metadata.flags.contains(&"invalid_utf8".to_string()));
    
    // Cannot get text content from invalid UTF-8
    let text_result = quarantine.get_text_content();
    assert!(text_result.is_err());
    
    println!("  ✅ Invalid UTF-8 detected: {}", quarantine.state);
}

#[tokio::test]
async fn test_7x_1_6_file_too_large() {
    println!("\n🧪 Test 7.x.1.6: File size validation");
    
    let large_data = vec![0u8; 101_000_000];  // 101MB (exceeds 100MB limit)
    let result = QuarantineZone::inspect(large_data, DataSource::Manual).await;
    
    assert!(result.is_err());
    
    match result {
        Err(e) => {
            println!("  ✅ Large file rejected: {}", e);
            assert!(e.to_string().contains("File too large"));
        }
        Ok(_) => panic!("Expected FileTooLarge error"),
    }
}

#[tokio::test]
async fn test_7x_1_7_whatsapp_mime_detection() {
    println!("\n🧪 Test 7.x.1.7: WhatsApp MIME type detection");
    
    let whatsapp_data = b"[2023-01-01, 10:00:00] Eduardo: Test\n[2023-01-01, 10:01:00] AI: Response".to_vec();
    let quarantine = QuarantineZone::inspect(whatsapp_data, DataSource::WhatsApp)
        .await
        .expect("Failed to create quarantine");
    
    assert_eq!(quarantine.metadata.mime_type, "text/plain; source=whatsapp");
    assert_eq!(quarantine.metadata.line_count, Some(2));
    
    println!("  ✅ WhatsApp MIME detected: {}", quarantine.metadata.mime_type);
}

#[tokio::test]
async fn test_7x_1_8_sha256_hash_computation() {
    println!("\n🧪 Test 7.x.1.8: SHA-256 hash computation");
    
    let data = b"Hello, world!".to_vec();
    let quarantine = QuarantineZone::inspect(data, DataSource::Manual)
        .await
        .expect("Failed to create quarantine");
    
    // SHA-256 of "Hello, world!"
    let expected_hash = "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3";
    assert_eq!(quarantine.metadata.content_hash, expected_hash);
    
    println!("  ✅ SHA-256 hash: {}", quarantine.metadata.content_hash);
}

#[tokio::test]
async fn test_7x_1_9_expiry_check() {
    println!("\n🧪 Test 7.x.1.9: Quarantine expiry");
    
    let data = b"Expiry test".to_vec();
    let mut quarantine = QuarantineZone::inspect(data, DataSource::Manual)
        .await
        .expect("Failed to create quarantine");
    
    // Initially not expired
    assert!(!quarantine.metadata.is_expired());
    
    // Manually set quarantine time to 25 hours ago
    quarantine.metadata.quarantined_at = 
        chrono::Utc::now() - Duration::hours(25);
    
    // Now should be expired (default 24h duration)
    assert!(quarantine.metadata.is_expired());
    
    println!("  ✅ Expiry check working");
}

#[tokio::test]
async fn test_7x_1_10_get_text_content() {
    println!("\n🧪 Test 7.x.1.10: Get text content");
    
    let data = b"UTF-8 text content".to_vec();
    let quarantine = QuarantineZone::inspect(data, DataSource::Manual)
        .await
        .expect("Failed to create quarantine");
    
    let text = quarantine.get_text_content()
        .expect("Failed to get text content");
    
    assert_eq!(text, "UTF-8 text content");
    
    println!("  ✅ Text content: {}", text);
}

#[tokio::test]
async fn test_7x_1_11_flags_management() {
    println!("\n🧪 Test 7.x.1.11: Flags management");
    
    let data = b"Test flags".to_vec();
    let mut quarantine = QuarantineZone::inspect(data, DataSource::Manual)
        .await
        .expect("Failed to create quarantine");
    
    // Initially no flags
    assert_eq!(quarantine.metadata.flags.len(), 0);
    
    // Add flags
    quarantine.metadata.add_flag("suspicious_pattern".to_string());
    quarantine.metadata.add_flag("unusual_size".to_string());
    
    assert_eq!(quarantine.metadata.flags.len(), 2);
    assert!(quarantine.metadata.flags.contains(&"suspicious_pattern".to_string()));
    assert!(quarantine.metadata.flags.contains(&"unusual_size".to_string()));
    
    // Duplicate flag should not be added
    quarantine.metadata.add_flag("suspicious_pattern".to_string());
    assert_eq!(quarantine.metadata.flags.len(), 2);
    
    println!("  ✅ Flags: {:?}", quarantine.metadata.flags);
}

#[tokio::test]
async fn test_7x_1_12_multiple_sources() {
    println!("\n🧪 Test 7.x.1.12: Multiple data sources");
    
    let sources = vec![
        DataSource::WhatsApp,
        DataSource::Telegram,
        DataSource::Email,
        DataSource::Calendar,
        DataSource::Spotify,
        DataSource::GitHub,
        DataSource::Twitter,
        DataSource::Manual,
    ];
    
    for source in sources {
        let data = format!("Test data from {:?}", source).into_bytes();
        let quarantine = QuarantineZone::inspect(data, source)
            .await
            .expect("Failed to create quarantine");
        
        assert_eq!(quarantine.source, source);
        assert_eq!(quarantine.state, QuarantineState::Safe);
        
        println!("  ✅ Source {} handled", source);
    }
}

#[tokio::test]
async fn test_7x_1_13_summary_output() {
    println!("\n🧪 Test 7.x.1.13: Summary output");
    
    let data = b"Summary test".to_vec();
    let quarantine = QuarantineZone::inspect(data, DataSource::WhatsApp)
        .await
        .expect("Failed to create quarantine");
    
    let summary = quarantine.summary();
    
    assert!(summary.contains("WhatsApp"));
    assert!(summary.contains("UTF-8"));
    assert!(summary.contains("Safe"));
    
    println!("  ✅ Summary: {}", summary);
}

// ================================================================
// INTEGRATION TEST: Full Quarantine Workflow
// ================================================================

#[tokio::test]
async fn test_7x_1_full_workflow() {
    println!("\n🚀 Integration Test: Full Quarantine Workflow");
    
    // Step 1: Inspect WhatsApp export
    println!("  → Step 1: Inspect WhatsApp export");
    let whatsapp_export = "[2023-01-01, 10:00:00] Eduardo: Inicio de conversacion\n\
                            [2023-01-01, 10:05:00] Maria: Hola Eduardo!\n\
                            [2023-01-01, 10:10:00] Eduardo: Como estas?".as_bytes().to_vec();
    
    let mut quarantine = QuarantineZone::inspect(whatsapp_export.clone(), DataSource::WhatsApp)
        .await
        .expect("Failed to inspect");
    
    assert_eq!(quarantine.state, QuarantineState::Safe);
    assert_eq!(quarantine.source, DataSource::WhatsApp);
    
    // Step 2: Verify metadata
    println!("  → Step 2: Verify metadata");
    assert_eq!(quarantine.metadata.encoding, "UTF-8");
    assert_eq!(quarantine.metadata.line_count, Some(3));
    assert_eq!(quarantine.metadata.size_bytes, whatsapp_export.len());
    assert!(!quarantine.metadata.content_hash.is_empty());
    
    // Step 3: Check readiness
    println!("  → Step 3: Check readiness");
    assert!(quarantine.is_ready_for_digestion());
    
    // Step 4: Approve
    println!("  → Step 4: Approve");
    quarantine.approve();
    assert_eq!(quarantine.state, QuarantineState::Approved);
    
    // Step 5: Extract text content
    println!("  → Step 5: Extract text content");
    let text = quarantine.get_text_content()
        .expect("Failed to get text");
    assert!(text.contains("Eduardo"));
    assert!(text.contains("Maria"));
    
    println!("  ✅ Full workflow completed successfully");
    println!("\n📊 Performance Metrics:");
    println!("  - Data size: {} bytes", quarantine.metadata.size_bytes);
    println!("  - Lines: {}", quarantine.metadata.line_count.unwrap());
    println!("  - Encoding: {}", quarantine.metadata.encoding);
    println!("  - MIME: {}", quarantine.metadata.mime_type);
    println!("  - State: {}", quarantine.state);
}
