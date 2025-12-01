//! # Test con datos REALES de WhatsApp
//!
//! **Chat:** Paula Roque
//! **Líneas:** 1,681
//! **Tamaño:** 116KB
//!
//! **Ejecutar:**
//! ```bash
//! cargo run --example test_real_whatsapp
//! ```

use bitacora::data_import::digestion::{DigestionPipeline, WhatsAppDigester};
use bitacora::data_import::{DataSource, QuarantineZone};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("\n🧪 Testing WhatsApp Digester con datos REALES");
    println!("{}", "=".repeat(60));
    
    // Read real WhatsApp export
    let file_path = "/home/edgi/Documents/Development/own/bitacora/bitacora_v1.0/data/imports/whatsapp/_chat.txt";
    
    println!("\n📂 Leyendo archivo: {}", file_path);
    let raw_data = match std::fs::read(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("❌ Error leyendo archivo: {}", e);
            return;
        }
    };
    
    let file_size = raw_data.len();
    println!("   ✓ Tamaño: {} bytes ({:.2} KB)", file_size, file_size as f64 / 1024.0);
    
    // Phase 1: Quarantine
    println!("\n📋 Phase 1: Quarantine");
    let start = std::time::Instant::now();
    let mut quarantine = match QuarantineZone::inspect(raw_data, DataSource::WhatsApp).await {
        Ok(q) => q,
        Err(e) => {
            eprintln!("❌ Quarantine failed: {}", e);
            return;
        }
    };
    let quarantine_duration = start.elapsed();
    
    println!("   ✓ Quarantine ID: {}", quarantine.id);
    println!("   ✓ State: {:?}", quarantine.state);
    println!("   ✓ Encoding: {}", quarantine.metadata.encoding);
    println!("   ✓ Lines: {:?}", quarantine.metadata.line_count);
    println!("   ✓ Duration: {:?}", quarantine_duration);
    
    // Approve
    quarantine.approve();
    println!("   ✓ Approved for digestion");
    
    // Phase 2: Digestion
    println!("\n🔬 Phase 2: Digestion");
    let digester = WhatsAppDigester::new();
    
    let start = std::time::Instant::now();
    let digested = match digester.digest(&quarantine).await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Digestion failed: {}", e);
            return;
        }
    };
    let digestion_duration = start.elapsed();
    
    println!("   ✓ Duration: {:?} ({}ms)", digestion_duration, digested.digestion_metadata.duration_ms);
    println!("   ✓ Entries: {}", digested.entries.len());
    println!("   ✓ Failed: {}", digested.digestion_metadata.failed_count);
    
    // Phase 3: Analysis
    println!("\n📊 Phase 3: Analysis");
    
    // Count by content type
    let mut text_count = 0;
    let mut photo_count = 0;
    let mut audio_count = 0;
    let mut video_count = 0;
    let mut attachment_count = 0;
    let mut system_count = 0;
    
    for entry in &digested.entries {
        match &entry.content_type {
            bitacora::data_import::digestion::ContentType::Text => text_count += 1,
            bitacora::data_import::digestion::ContentType::Multimedia { mime_type } => {
                if mime_type.starts_with("image") {
                    photo_count += 1;
                } else if mime_type.starts_with("audio") {
                    audio_count += 1;
                } else if mime_type.starts_with("video") {
                    video_count += 1;
                }
            }
            bitacora::data_import::digestion::ContentType::Attachment { .. } => attachment_count += 1,
            bitacora::data_import::digestion::ContentType::System => system_count += 1,
        }
    }
    
    println!("   ✓ Text messages: {}", text_count);
    println!("   ✓ Photos: {}", photo_count);
    println!("   ✓ Audio: {}", audio_count);
    println!("   ✓ Video: {}", video_count);
    println!("   ✓ Attachments: {}", attachment_count);
    println!("   ✓ System messages: {}", system_count);
    
    // Count by author
    let mut author_counts: HashMap<String, usize> = HashMap::new();
    for entry in &digested.entries {
        if let Some(author) = &entry.author {
            *author_counts.entry(author.clone()).or_insert(0) += 1;
        }
    }
    
    println!("\n👥 Authors:");
    for (author, count) in author_counts.iter() {
        println!("   - {}: {} messages", author, count);
    }
    
    // Date range
    if let (Some(first), Some(last)) = (digested.entries.first(), digested.entries.last()) {
        println!("\n📅 Date range:");
        println!("   - First: {}", first.timestamp.format("%Y-%m-%d %H:%M:%S"));
        println!("   - Last: {}", last.timestamp.format("%Y-%m-%d %H:%M:%S"));
        
        let duration_days = (last.timestamp - first.timestamp).num_days();
        println!("   - Duration: {} days", duration_days);
    }
    
    // Performance
    println!("\n⚡ Performance:");
    let messages_per_ms = digested.entries.len() as f64 / digested.digestion_metadata.duration_ms as f64;
    println!("   - Messages/ms: {:.2}", messages_per_ms);
    println!("   - Target: <30s for 1000 messages");
    println!("   - Actual: {}ms for {} messages", 
        digested.digestion_metadata.duration_ms, 
        digested.entries.len());
    
    let target_ms = 30_000;
    let speedup = target_ms as f64 / digested.digestion_metadata.duration_ms as f64;
    println!("   - Speedup vs target: {:.0}x faster", speedup);
    
    // Show sample entries
    println!("\n📝 Sample entries (first 10):");
    for (i, entry) in digested.entries.iter().take(10).enumerate() {
        println!("\n{}. [{}] {}:",
            i + 1,
            entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
            entry.author.as_ref().unwrap_or(&"System".to_string())
        );
        println!("   Type: {:?}", entry.content_type);
        let content_preview = if entry.content.len() > 80 {
            format!("{}...", &entry.content[..80])
        } else {
            entry.content.clone()
        };
        println!("   Content: {}", content_preview);
    }
    
    // Success rate
    let success_rate = digested.entries.len() as f64 / 
        (digested.entries.len() + digested.digestion_metadata.failed_count) as f64 * 100.0;
    
    println!("\n✅ Summary:");
    println!("   - Total entries: {}", digested.entries.len());
    println!("   - Success rate: {:.2}%", success_rate);
    println!("   - Performance: {:.0}x faster than target", speedup);
    println!("   - File size: {:.2} KB", file_size as f64 / 1024.0);
    println!("   - Processing speed: {:.2} KB/ms", 
        file_size as f64 / 1024.0 / digested.digestion_metadata.duration_ms as f64);
    
    if digested.digestion_metadata.failed_count > 0 {
        println!("\n⚠️  Failed entries: {}", digested.digestion_metadata.failed_count);
        println!("   Errors (first 10):");
        for error in digested.digestion_metadata.errors.iter().take(10) {
            println!("   - {}", error);
        }
    }
    
    println!("\n🎯 Test COMPLETE!\n");
}
