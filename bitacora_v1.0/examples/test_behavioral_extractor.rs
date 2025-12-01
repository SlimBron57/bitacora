//! # 🧬 BehavioralExtractor - Real Data Test
//!
//! **Purpose:** Validate BehavioralExtractor with real WhatsApp data
//!
//! **Test Flow:**
//! 1. 📦 Quarantine: Import Paula Roque chat
//! 2. 🔄 Digestion: Parse with WhatsAppDigester
//! 3. 🧬 Extraction: Extract behavioral patterns with BehavioralExtractor
//! 4. 📊 Analysis: Validate 4 pattern types
//!
//! **Expected Patterns:**
//! - Communication style (formal/casual/mixed)
//! - Message length pattern (very short → very long)
//! - Engagement style (bursty/steady/delayed)
//! - Punctuation pattern (enthusiastic/formal/balanced)
//!
//! **Quality Criteria:**
//! - All 4 core patterns detected
//! - Confidence ≥0.5 for each pattern
//! - Extraction <10s for 1,354 entries
//! - Meaningful insights from patterns

use bitacora::data_import::{
    QuarantineZone, DataSource,
    BehavioralExtractor, NutrientExtractor,
};
use bitacora::data_import::digestion::{WhatsAppDigester, DigestionPipeline};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  🧬 BehavioralExtractor - Real Data Test                  ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // ============================================================
    // PHASE 1: QUARANTINE
    // ============================================================
    
    println!("📦 [1/3] Quarantine Phase");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let import_path = Path::new("data/imports/whatsapp/_chat.txt");
    if !import_path.exists() {
        eprintln!("❌ Test file not found: {}", import_path.display());
        std::process::exit(1);
    }

    let raw_data = std::fs::read(import_path)?;
    let quarantine = QuarantineZone::inspect(raw_data, DataSource::WhatsApp).await?;
    
    println!("  ✓ File quarantined");
    println!();

    // ============================================================
    // PHASE 2: DIGESTION
    // ============================================================
    
    println!("🔄 [2/3] Digestion Phase (WhatsApp Parser)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let digester = WhatsAppDigester::default();
    let digested = digester.digest(&quarantine).await?;
    
    println!("  ✓ Parsed {} entries", digested.entries.len());
    println!("  ✓ Source: {:?}", digested.source);
    println!();

    // ============================================================
    // PHASE 3: BEHAVIORAL EXTRACTION
    // ============================================================
    
    println!("🧬 [3/3] Behavioral Pattern Extraction");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let start = std::time::Instant::now();
    let extractor = BehavioralExtractor;
    let extracted = extractor.extract(&digested).await?;
    let extraction_time = start.elapsed();
    
    println!("  ⚡ Extraction time: {:?}", extraction_time);
    println!();

    // ============================================================
    // RESULTS ANALYSIS
    // ============================================================
    
    println!("📊 Results");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    println!("📈 Extraction Metadata:");
    println!("  Total Entries:     {}", digested.entries.len());
    println!("  Entries Processed: {}", extracted.extraction_metadata.entries_processed);
    println!("  Entries Skipped:   {}", extracted.extraction_metadata.entries_skipped);
    println!("  Nutrients Found:   {}", extracted.nutrients.len());
    println!("  Duration:          {}ms", extraction_time.as_millis());
    println!();

    println!("📊 Statistics:");
    for (key, value) in &extracted.extraction_metadata.stats {
        println!("  {}: {}", key, value);
    }
    println!();

    // ============================================================
    // DETAILED PATTERN ANALYSIS
    // ============================================================
    
    println!("🔍 Detailed Behavioral Patterns");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // 1. Communication style
    let comm_style = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "communication_style");
    
    if let Some(nutrient) = comm_style {
        let default_zero = "0".to_string();
        let default_total = "0".to_string();
        let formal_score = nutrient.metadata.get("formal_score").unwrap_or(&default_zero);
        let casual_score = nutrient.metadata.get("casual_score").unwrap_or(&default_zero);
        let total = nutrient.metadata.get("total_messages").unwrap_or(&default_total);
        
        println!("🗣️  Communication Style:");
        println!("  Style: 📍 {}", nutrient.value);
        println!("  Formal Score: {}", formal_score);
        println!("  Casual Score: {}", casual_score);
        println!("  Confidence: {:.2}", nutrient.confidence);
        println!("  Total Messages: {}", total);
        println!();
    }
    
    // 2. Message length pattern
    let msg_length = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "message_length");
    
    if let Some(nutrient) = msg_length {
        let default_zero = "0".to_string();
        let default_total = "0".to_string();
        let avg_length = nutrient.metadata.get("average_length").unwrap_or(&default_zero);
        let total = nutrient.metadata.get("total_messages").unwrap_or(&default_total);
        
        println!("📏 Message Length Pattern:");
        println!("  Pattern: 📍 {}", nutrient.value);
        println!("  Average Length: {} chars", avg_length);
        println!("  Confidence: {:.2}", nutrient.confidence);
        println!("  Total Messages: {}", total);
        println!();
    }

    // 3. Engagement style
    let engagement = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "engagement_style");
    
    if let Some(nutrient) = engagement {
        let default_zero = "0".to_string();
        let default_gaps = "0".to_string();
        let burst_pct = nutrient.metadata.get("burst_percentage").unwrap_or(&default_zero);
        let gap_pct = nutrient.metadata.get("long_gap_percentage").unwrap_or(&default_zero);
        let gaps = nutrient.metadata.get("total_gaps_analyzed").unwrap_or(&default_gaps);
        
        println!("⚡ Engagement Style:");
        println!("  Style: 📍 {}", nutrient.value);
        println!("  Burst Percentage: {}", burst_pct);
        println!("  Long Gap Percentage: {}", gap_pct);
        println!("  Confidence: {:.2}", nutrient.confidence);
        println!("  Gaps Analyzed: {}", gaps);
        println!();
    }

    // 4. Punctuation pattern
    let punctuation = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "punctuation_pattern");
    
    if let Some(nutrient) = punctuation {
        let default_zero = "0".to_string();
        let excl = nutrient.metadata.get("exclamations_per_msg").unwrap_or(&default_zero);
        let quest = nutrient.metadata.get("questions_per_msg").unwrap_or(&default_zero);
        let period = nutrient.metadata.get("periods_per_msg").unwrap_or(&default_zero);
        let emoji = nutrient.metadata.get("emojis_per_msg").unwrap_or(&default_zero);
        let enthusiasm = nutrient.metadata.get("enthusiasm_score").unwrap_or(&default_zero);
        
        println!("❗ Punctuation Pattern:");
        println!("  Pattern: 📍 {}", nutrient.value);
        println!("  Exclamations/msg: {}", excl);
        println!("  Questions/msg: {}", quest);
        println!("  Periods/msg: {}", period);
        println!("  Emojis/msg: {}", emoji);
        println!("  Enthusiasm Score: {}", enthusiasm);
        println!("  Confidence: {:.2}", nutrient.confidence);
        println!();
    }

    // ============================================================
    // PATTERN INSIGHTS
    // ============================================================
    
    println!("💡 Pattern Insights");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Communication style insight
    if let Some(style_nutrient) = comm_style {
        match style_nutrient.value.as_str() {
            "Formal" => println!("  🎩 Formal communicator - Uses proper grammar and punctuation"),
            "Casual" => println!("  😎 Casual communicator - Informal, uses emojis and abbreviations"),
            "Mixed" => println!("  🔄 Adaptive communicator - Switches between formal and casual"),
            _ => println!("  ❓ Unknown communication style"),
        }
    }

    // Message length insight
    if let Some(length_nutrient) = msg_length {
        match length_nutrient.value.as_str() {
            "VeryShort" => println!("  💬 Very brief messages - Quick, minimal text"),
            "Short" => println!("  📝 Short messages - Concise communication"),
            "Medium" => println!("  📄 Medium messages - Balanced detail"),
            "Long" => println!("  📖 Long messages - Detailed, thorough"),
            "VeryLong" => println!("  📚 Very long messages - Comprehensive, essay-style"),
            _ => {},
        }
    }

    // Engagement style insight
    if let Some(engagement_nutrient) = engagement {
        match engagement_nutrient.value.as_str() {
            "Bursty" => println!("  🌊 Bursty engagement - Messages come in rapid bursts"),
            "Steady" => println!("  🔄 Steady engagement - Even distribution throughout day"),
            "Delayed" => println!("  ⏰ Delayed engagement - Long gaps between messages"),
            _ => {},
        }
    }

    // Punctuation insight
    if let Some(punct_nutrient) = punctuation {
        match punct_nutrient.value.as_str() {
            "Enthusiastic" => println!("  ✨ Enthusiastic style - High energy, emojis, exclamations"),
            "Formal" => println!("  📐 Formal punctuation - Proper periods, minimal enthusiasm"),
            "Balanced" => println!("  ⚖️  Balanced punctuation - Mix of formal and casual"),
            _ => {},
        }
    }

    println!();

    // ============================================================
    // PERFORMANCE ASSESSMENT
    // ============================================================
    
    println!("⚡ Performance Assessment");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let throughput = digested.entries.len() as f64 / extraction_time.as_millis() as f64;
    let nutrients_per_entry = extracted.nutrients.len() as f64 / digested.entries.len() as f64;
    let target_ms_per_1k = 10_000.0; // 10s for 1000 entries
    let actual_ms_per_1k = (extraction_time.as_millis() as f64 / digested.entries.len() as f64) * 1000.0;
    let performance_factor = target_ms_per_1k / actual_ms_per_1k;

    println!("  Throughput:       {:.2} entries/ms", throughput);
    println!("  Nutrients/Entry:  {:.3}", nutrients_per_entry);
    println!("  Target:           0.10 entries/ms");
    println!("  Performance:      {:.0}x faster than target", performance_factor);
    println!();

    // ============================================================
    // QUALITY ASSESSMENT
    // ============================================================
    
    println!("🎯 Extraction Quality Assessment");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let has_comm_style = comm_style.is_some();
    let has_msg_length = msg_length.is_some();
    let has_engagement = engagement.is_some();
    let has_punctuation = punctuation.is_some();

    println!("  Communication Style: {}", if has_comm_style { "✓ Present" } else { "✗ Missing" });
    println!("  Message Length:      {}", if has_msg_length { "✓ Present" } else { "✗ Missing" });
    println!("  Engagement Style:    {}", if has_engagement { "✓ Present" } else { "✗ Missing" });
    println!("  Punctuation Pattern: {}", if has_punctuation { "✓ Present" } else { "✗ Missing" });
    println!();

    let core_patterns = [has_comm_style, has_msg_length, has_engagement, has_punctuation]
        .iter()
        .filter(|&&x| x)
        .count();
    
    let quality_score = (core_patterns as f64 / 4.0) * 100.0;
    
    println!("  Core Patterns: {}/4", core_patterns);
    println!("  Quality Score: {:.0}%", quality_score);
    
    if quality_score >= 100.0 {
        println!("  Status: ✅ EXCELLENT - All behavioral patterns detected");
    } else if quality_score >= 75.0 {
        println!("  Status: ✅ GOOD - Most patterns detected");
    } else if quality_score >= 50.0 {
        println!("  Status: ⚠️  PARTIAL - Some patterns missing");
    } else {
        println!("  Status: ❌ INCOMPLETE - Major patterns missing");
    }
    println!();

    // ============================================================
    // TEST COMPLETE
    // ============================================================
    
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  ✅ BehavioralExtractor Test Complete                     ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Summary:");
    println!("  • {} entries analyzed", digested.entries.len());
    println!("  • {} behavioral nutrients extracted", extracted.nutrients.len());
    println!("  • {:.0}x faster than target performance", performance_factor);
    println!("  • {:.0}% quality score", quality_score);
    println!();

    Ok(())
}
