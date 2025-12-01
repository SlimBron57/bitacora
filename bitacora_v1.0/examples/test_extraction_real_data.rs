//! # Test de Extraction con datos REALES
//!
//! **Validación:** InterestExtractor + EmotionalExtractor
//! **Data:** Paula Roque chat (839 mensajes de texto)
//!
//! **Ejecutar:**
//! ```bash
//! cargo run --example test_extraction_real_data
//! ```

use bitacora::data_import::digestion::{DigestionPipeline, WhatsAppDigester};
use bitacora::data_import::extraction::{EmotionalExtractor, InterestExtractor, NutrientExtractor};
use bitacora::data_import::{DataSource, QuarantineZone};

#[tokio::main]
async fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  🧬 NUTRIENT EXTRACTION - Real WhatsApp Data               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ==============================================================
    // 🔹 PHASE 1: Load & Digest (reuse from digestion test)
    // ==============================================================
    let file_path = "/home/edgi/Documents/Development/own/bitacora/bitacora_v1.0/data/imports/whatsapp/_chat.txt";
    
    print!("📂 Loading file... ");
    let raw_data = match std::fs::read(file_path) {
        Ok(data) => {
            println!("✓");
            data
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            return;
        }
    };

    print!("📋 Quarantine + Digestion... ");
    let mut quarantine = match QuarantineZone::inspect(raw_data, DataSource::WhatsApp).await {
        Ok(q) => q,
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            return;
        }
    };
    quarantine.approve();

    let digester = WhatsAppDigester::new();
    let digested = match digester.digest(&quarantine).await {
        Ok(d) => {
            println!("✓ ({} entries)", d.entries.len());
            d
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            return;
        }
    };

    // Count text entries
    let text_count = digested
        .entries
        .iter()
        .filter(|e| matches!(e.content_type, bitacora::data_import::digestion::ContentType::Text))
        .count();
    
    println!("   • Text messages available: {}", text_count);

    println!("\n{}", "═".repeat(66));
    
    // ==============================================================
    // 🔹 PHASE 2: Interest Extraction
    // ==============================================================
    println!("\n📊 INTEREST EXTRACTION (Dimension 2/7)");
    println!("{}", "─".repeat(66));

    let interest_extractor = InterestExtractor::new();
    
    print!("🔬 Extracting interests... ");
    let start = std::time::Instant::now();
    let interests = match interest_extractor.extract(&digested).await {
        Ok(nutrients) => {
            let duration = start.elapsed();
            println!("✓ ({}ms)", duration.as_millis());
            nutrients
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            return;
        }
    };

    println!("\n📈 Results:");
    println!("   • Nutrients extracted: {}", interests.nutrients.len());
    println!("   • Entries processed: {}", interests.extraction_metadata.entries_processed);
    println!("   • Entries skipped: {}", interests.extraction_metadata.entries_skipped);
    println!("   • Avg confidence: {:.2}", interests.extraction_metadata.avg_confidence);
    println!("   • Duration: {}ms", interests.extraction_metadata.duration_ms);

    // Stats
    if let Some(keywords) = interests.extraction_metadata.stats.get("unique_keywords") {
        println!("   • Unique keywords: {}", keywords);
    }
    if let Some(urls) = interests.extraction_metadata.stats.get("unique_urls") {
        println!("   • URLs detected: {}", urls);
    }

    // Top keywords
    println!("\n🔝 Top 20 Keywords:");
    let keyword_nutrients: Vec<_> = interests
        .nutrients
        .iter()
        .filter(|n| n.nutrient_type == "keyword")
        .take(20)
        .collect();
    
    for (i, nutrient) in keyword_nutrients.iter().enumerate() {
        let default_freq = "0".to_string();
        let freq = nutrient.metadata.get("frequency").unwrap_or(&default_freq);
        println!("   {}. {} (freq: {}, confidence: {:.2})", 
            i + 1, 
            nutrient.value, 
            freq,
            nutrient.confidence
        );
    }

    // URLs sample
    let url_nutrients: Vec<_> = interests
        .nutrients
        .iter()
        .filter(|n| n.nutrient_type == "url")
        .take(10)
        .collect();
    
    if !url_nutrients.is_empty() {
        println!("\n🔗 Sample URLs (first 10):");
        for (i, nutrient) in url_nutrients.iter().enumerate() {
            println!("   {}. {}", i + 1, nutrient.value);
        }
    }

    // ==============================================================
    // 🔹 PHASE 3: Emotional Extraction
    // ==============================================================
    println!("\n\n📊 EMOTIONAL EXTRACTION (Dimension 3/7)");
    println!("{}", "─".repeat(66));

    let emotional_extractor = EmotionalExtractor::new();
    
    print!("🔬 Extracting emotions... ");
    let start = std::time::Instant::now();
    let emotions = match emotional_extractor.extract(&digested).await {
        Ok(nutrients) => {
            let duration = start.elapsed();
            println!("✓ ({}ms)", duration.as_millis());
            nutrients
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            return;
        }
    };

    println!("\n📈 Results:");
    println!("   • Nutrients extracted: {}", emotions.nutrients.len());
    println!("   • Entries processed: {}", emotions.extraction_metadata.entries_processed);
    println!("   • Entries skipped: {}", emotions.extraction_metadata.entries_skipped);
    println!("   • Avg confidence: {:.2}", emotions.extraction_metadata.avg_confidence);
    println!("   • Duration: {}ms", emotions.extraction_metadata.duration_ms);

    // Stats
    if let Some(positive) = emotions.extraction_metadata.stats.get("positive_messages") {
        println!("   • Positive messages: {}", positive);
    }
    if let Some(negative) = emotions.extraction_metadata.stats.get("negative_messages") {
        println!("   • Negative messages: {}", negative);
    }
    if let Some(neutral) = emotions.extraction_metadata.stats.get("neutral_messages") {
        println!("   • Neutral messages: {}", neutral);
    }
    if let Some(pct) = emotions.extraction_metadata.stats.get("positive_percentage") {
        println!("   • Positive percentage: {}%", pct);
    }

    // Sentiment distribution
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut neutral_count = 0;

    for nutrient in &emotions.nutrients {
        match nutrient.value.as_str() {
            "Positive" => positive_count += 1,
            "Negative" => negative_count += 1,
            "Neutral" => neutral_count += 1,
            _ => {}
        }
    }

    println!("\n📊 Sentiment Distribution:");
    println!("   • Positive: {} ({:.1}%)", 
        positive_count,
        positive_count as f64 / emotions.nutrients.len() as f64 * 100.0
    );
    println!("   • Negative: {} ({:.1}%)", 
        negative_count,
        negative_count as f64 / emotions.nutrients.len() as f64 * 100.0
    );
    println!("   • Neutral: {} ({:.1}%)", 
        neutral_count,
        neutral_count as f64 / emotions.nutrients.len() as f64 * 100.0
    );

    // Sample positive messages
    println!("\n😊 Sample Positive Messages (first 5):");
    let positive_nutrients: Vec<_> = emotions
        .nutrients
        .iter()
        .filter(|n| n.value == "Positive")
        .take(5)
        .collect();
    
    for (i, nutrient) in positive_nutrients.iter().enumerate() {
        // Find original message
        if let Some(entry) = digested.entries.iter().find(|e| e.id == nutrient.source_entry_id) {
            let preview = if entry.content.len() > 60 {
                format!("{}...", &entry.content[..60])
            } else {
                entry.content.clone()
            };
            
            let default_zero = "0".to_string();
            let score = nutrient.metadata.get("score").unwrap_or(&default_zero);
            let intensity = nutrient.metadata.get("intensity").unwrap_or(&default_zero);
            
            println!("   {}. {} (score: {}, intensity: {})", 
                i + 1, 
                preview,
                score,
                intensity
            );
        }
    }

    // Sample negative messages
    println!("\n😔 Sample Negative Messages (first 5):");
    let negative_nutrients: Vec<_> = emotions
        .nutrients
        .iter()
        .filter(|n| n.value == "Negative")
        .take(5)
        .collect();
    
    for (i, nutrient) in negative_nutrients.iter().enumerate() {
        if let Some(entry) = digested.entries.iter().find(|e| e.id == nutrient.source_entry_id) {
            let preview = if entry.content.len() > 60 {
                format!("{}...", &entry.content[..60])
            } else {
                entry.content.clone()
            };
            
            let default_zero = "0".to_string();
            let score = nutrient.metadata.get("score").unwrap_or(&default_zero);
            let intensity = nutrient.metadata.get("intensity").unwrap_or(&default_zero);
            
            println!("   {}. {} (score: {}, intensity: {})", 
                i + 1, 
                preview,
                score,
                intensity
            );
        }
    }

    // ==============================================================
    // 🔹 PHASE 4: Summary
    // ==============================================================
    println!("\n\n{}", "═".repeat(66));
    println!("✅ EXTRACTION COMPLETE - Phase 7.x.3 Validation");
    println!("{}", "═".repeat(66));

    println!("\n📊 Summary:");
    println!("   • Original entries: {}", digested.entries.len());
    println!("   • Text entries: {}", text_count);
    println!("   • Interest nutrients: {}", interests.nutrients.len());
    println!("   • Emotional nutrients: {}", emotions.nutrients.len());
    println!("   • Total nutrients: {}", interests.nutrients.len() + emotions.nutrients.len());

    let total_duration = interests.extraction_metadata.duration_ms + emotions.extraction_metadata.duration_ms;
    println!("\n⚡ Performance:");
    println!("   • Total extraction time: {}ms", total_duration);
    println!("   • Target: <10s (10,000ms)");
    println!("   • Achieved: {}ms", total_duration);
    
    if total_duration < 10_000 {
        let speedup = 10_000.0 / total_duration as f64;
        println!("   • Speedup vs target: {:.0}x faster ✅", speedup);
    }

    println!("\n🎯 Next Steps:");
    println!("   1. Implement BiographicalExtractor (Task 7.x.3.2)");
    println!("   2. Implement RelationshipExtractor (Task 7.x.3.5)");
    println!("   3. Implement TemporalExtractor (Task 7.x.3.6)");
    println!("   4. Integration with TopicGraph + EmotionalSpace");

    println!("\n{}", "═".repeat(66));
}
