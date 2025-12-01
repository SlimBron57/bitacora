//! # 🧬 RelationshipExtractor - Real Data Test
//!
//! **Purpose:** Validate RelationshipExtractor with real WhatsApp data
//!
//! **Test Flow:**
//! 1. 📦 Quarantine: Import Paula Roque chat
//! 2. 🔄 Digestion: Parse with WhatsAppDigester
//! 3. 🧬 Extraction: Extract social patterns with RelationshipExtractor
//! 4. 📊 Analysis: Validate participants, interactions, relationship type, balance
//!
//! **Expected Patterns:**
//! - Participants list (unique authors)
//! - Interaction strength per participant
//! - Relationship type classification (Romantic/Family/Friend/Professional/Ambiguous)
//! - Communication balance (Balanced/Moderately_Balanced/Unbalanced/Monologue)
//!
//! **Quality Criteria:**
//! - All participants detected
//! - Interaction strengths calculated
//! - Relationship type classified (if 2+ authors)
//! - Communication balance analyzed
//! - Extraction <10s for 1,354 entries

use bitacora::data_import::{
    QuarantineZone, DataSource,
    RelationshipExtractor, NutrientExtractor,
};
use bitacora::data_import::digestion::{WhatsAppDigester, DigestionPipeline};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  🧬 RelationshipExtractor - Real Data Test                ║");
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
    // PHASE 3: RELATIONSHIP EXTRACTION
    // ============================================================
    
    println!("🧬 [3/3] Social Patterns Extraction");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let start = std::time::Instant::now();
    let extractor = RelationshipExtractor;
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
    
    println!("🔍 Detailed Social Patterns");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // 1. Participants
    let participants = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "participants");
    
    if let Some(nutrient) = participants {
        let default_zero = "0".to_string();
        let count = nutrient.metadata.get("participant_count").unwrap_or(&default_zero);
        let total = nutrient.metadata.get("total_messages").unwrap_or(&default_zero);
        
        println!("👥 Participants:");
        println!("  People: 📍 {}", nutrient.value);
        println!("  Count: {}", count);
        println!("  Total Messages: {}", total);
        println!("  Confidence: {:.2}", nutrient.confidence);
        println!();
    }
    
    // 2. Interaction strengths
    let interactions: Vec<_> = extracted.nutrients.iter()
        .filter(|n| n.nutrient_type == "interaction_strength")
        .collect();
    
    if !interactions.is_empty() {
        println!("💬 Interaction Strengths:");
        for nutrient in interactions.iter() {
            let default_zero = "0".to_string();
            let count = nutrient.metadata.get("message_count").unwrap_or(&default_zero);
            let pct = nutrient.metadata.get("percentage").unwrap_or(&default_zero);
            
            println!("  {} - {} messages ({}%)", nutrient.value, count, pct);
        }
        println!();
    }

    // 3. Relationship type
    let relationship = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "relationship_type");
    
    if let Some(nutrient) = relationship {
        let default_zero = "0".to_string();
        let participants_str = nutrient.metadata.get("participants").unwrap_or(&default_zero);
        let romantic = nutrient.metadata.get("romantic_score").unwrap_or(&default_zero);
        let family = nutrient.metadata.get("family_score").unwrap_or(&default_zero);
        let friend = nutrient.metadata.get("friend_score").unwrap_or(&default_zero);
        let professional = nutrient.metadata.get("professional_score").unwrap_or(&default_zero);
        
        println!("💕 Relationship Type:");
        println!("  Type: 📍 {}", nutrient.value);
        println!("  Between: {}", participants_str);
        println!("  Scores:");
        println!("    Romantic: {}", romantic);
        println!("    Family: {}", family);
        println!("    Friend: {}", friend);
        println!("    Professional: {}", professional);
        println!("  Confidence: {:.2}", nutrient.confidence);
        println!();
    }

    // 4. Communication balance
    let balance = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "communication_balance");
    
    if let Some(nutrient) = balance {
        let default_zero = "0".to_string();
        let ratio = nutrient.metadata.get("balance_ratio").unwrap_or(&default_zero);
        let author_count = nutrient.metadata.get("author_count").unwrap_or(&default_zero);
        let most = nutrient.metadata.get("most_active_count").unwrap_or(&default_zero);
        let least = nutrient.metadata.get("least_active_count").unwrap_or(&default_zero);
        
        println!("⚖️  Communication Balance:");
        println!("  Type: 📍 {}", nutrient.value);
        println!("  Balance Ratio: {}", ratio);
        println!("  Authors: {}", author_count);
        println!("  Most Active: {} messages", most);
        println!("  Least Active: {} messages", least);
        println!("  Confidence: {:.2}", nutrient.confidence);
        println!();
    }

    // ============================================================
    // PATTERN INSIGHTS
    // ============================================================
    
    println!("💡 Social Pattern Insights");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Participants insight
    if let Some(p) = participants {
        let count = p.metadata.get("participant_count").unwrap_or(&"0".to_string()).parse::<usize>().unwrap_or(0);
        match count {
            1 => println!("  📝 Monologue - Single author (personal notes or diary)"),
            2 => println!("  💬 Dyadic conversation - Two-person dialogue"),
            3..=5 => println!("  👥 Small group - Close circle communication"),
            _ => println!("  🎪 Large group - Community or broadcast style"),
        }
    }

    // Relationship insight
    if let Some(rel) = relationship {
        match rel.value.as_str() {
            "Romantic" => println!("  💕 Romantic relationship - Love language detected"),
            "Family" => println!("  👨‍👩‍👧 Family bond - Familial references present"),
            "Friend" => println!("  🤝 Friendship - Social and casual interaction"),
            "Professional" => println!("  💼 Professional relationship - Work-related communication"),
            "Ambiguous" => println!("  ❓ Ambiguous relationship - No clear type indicators"),
            _ => {},
        }
    }

    // Balance insight
    if let Some(bal) = balance {
        match bal.value.as_str() {
            "Balanced" => println!("  ⚖️  Equal participation - Healthy two-way communication"),
            "Moderately_Balanced" => println!("  📊 Moderate balance - Slightly uneven participation"),
            "Unbalanced" => println!("  ⚠️  Unbalanced - One person dominates the conversation"),
            "Monologue" => println!("  📢 Monologue - Single voice (notes or broadcasting)"),
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

    let has_participants = participants.is_some();
    let has_interactions = !interactions.is_empty();
    let has_relationship = relationship.is_some();
    let has_balance = balance.is_some();

    println!("  Participants:     {}", if has_participants { "✓ Present" } else { "✗ Missing" });
    println!("  Interactions:     {}", if has_interactions { "✓ Present" } else { "✗ Missing" });
    println!("  Relationship:     {}", if has_relationship { "✓ Present" } else { "○ Optional (N/A)" });
    println!("  Balance:          {}", if has_balance { "✓ Present" } else { "✗ Missing" });
    println!();

    let core_patterns = [has_participants, has_interactions, has_balance]
        .iter()
        .filter(|&&x| x)
        .count();
    
    let quality_score = (core_patterns as f64 / 3.0) * 100.0;
    
    println!("  Core Patterns: {}/3", core_patterns);
    println!("  Quality Score: {:.0}%", quality_score);
    
    if quality_score >= 100.0 {
        println!("  Status: ✅ EXCELLENT - All social patterns detected");
    } else if quality_score >= 67.0 {
        println!("  Status: ✅ GOOD - Most patterns detected");
    } else if quality_score >= 33.0 {
        println!("  Status: ⚠️  PARTIAL - Some patterns missing");
    } else {
        println!("  Status: ❌ INCOMPLETE - Major patterns missing");
    }
    println!();

    // ============================================================
    // TEST COMPLETE
    // ============================================================
    
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  ✅ RelationshipExtractor Test Complete                   ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Summary:");
    println!("  • {} entries analyzed", digested.entries.len());
    println!("  • {} social nutrients extracted", extracted.nutrients.len());
    println!("  • {:.0}x faster than target performance", performance_factor);
    println!("  • {:.0}% quality score", quality_score);
    println!();

    Ok(())
}
