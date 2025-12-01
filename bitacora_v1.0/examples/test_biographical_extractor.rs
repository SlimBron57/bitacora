// Test: BiographicalExtractor with real WhatsApp data
// Purpose: Validate biographical information extraction (names, locations, age, occupation)

use bitacora::data_import::{
    digestion::{DigestionPipeline, WhatsAppDigester},
    extraction::{BiographicalExtractor, NutrientExtractor},
    DataSource, QuarantineZone,
};

use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  🧬 BiographicalExtractor - Real Data Test                ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    // ============================================================
    // PHASE 1: Quarantine
    // ============================================================
    
    println!("📦 [1/3] Quarantine Phase");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Read real WhatsApp export
    let source_path = PathBuf::from("data/imports/whatsapp/_chat.txt");
    
    if !source_path.exists() {
        eprintln!("❌ Error: Test data not found at {:?}", source_path);
        eprintln!("   Please ensure data/imports/whatsapp/_chat.txt exists");
        std::process::exit(1);
    }
    
    let raw_data = std::fs::read(&source_path)?;
    let quarantine = QuarantineZone::inspect(raw_data, bitacora::data_import::DataSource::WhatsApp).await?;
    
    println!("  ✓ File quarantined");
    println!();
    
    // ============================================================
    // PHASE 2: Digestion
    // ============================================================
    
    println!("🔄 [2/3] Digestion Phase (WhatsApp Parser)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let digester = WhatsAppDigester::new();
    
    let digested = digester.digest(&quarantine).await?;
    
    let duration_days = digested.digestion_metadata.stats.get("duration_days").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let participant_count = digested.digestion_metadata.stats.get("participant_count").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    
    println!("  ✓ Parsed {} entries", digested.entries.len());
    println!("  ✓ Duration: {} days", duration_days);
    println!("  ✓ Authors: {}", participant_count);
    println!();
    
    // ============================================================
    // PHASE 3: Biographical Extraction
    // ============================================================
    
    println!("🧬 [3/3] Biographical Extraction");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let extractor = BiographicalExtractor::new();
    
    let start = std::time::Instant::now();
    let extracted = extractor.extract(&digested).await?;
    let duration = start.elapsed();
    
    println!("  ⚡ Extraction time: {:?}", duration);
    println!();
    
    // ============================================================
    // RESULTS ANALYSIS
    // ============================================================
    
    println!("📊 Results");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    println!("📈 Extraction Metadata:");
    let total_entries = digested.entries.len();
    println!("  Total Entries:     {}", total_entries);
    println!("  Entries Processed: {}", extracted.extraction_metadata.entries_processed);
    println!("  Entries Skipped:   {}", extracted.extraction_metadata.entries_skipped);
    println!("  Nutrients Found:   {}", extracted.extraction_metadata.nutrients_extracted);
    println!("  Duration:          {}ms", extracted.extraction_metadata.duration_ms);
    println!();
    
    println!("📊 Statistics:");
    for (key, value) in &extracted.extraction_metadata.stats {
        println!("  {}: {}", key, value);
    }
    println!();
    
    // ============================================================
    // DETAILED RESULTS BY FIELD
    // ============================================================
    
    println!("🔍 Detailed Results by Field");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    // 1. Names
    let name_nutrients: Vec<_> = extracted
        .nutrients
        .iter()
        .filter(|n| n.nutrient_type == "name")
        .collect();
    
    if !name_nutrients.is_empty() {
        println!("👤 Names Detected ({} unique):", name_nutrients.len());
        for nutrient in &name_nutrients {
            let default_zero = "0".to_string();
            let evidence_count = nutrient.metadata.get("evidence_count").unwrap_or(&default_zero);
            println!(
                "  • {} (confidence: {:.2}, evidence: {})",
                nutrient.value, nutrient.confidence, evidence_count
            );
        }
        println!();
    } else {
        println!("👤 Names: None detected");
        println!();
    }
    
    // 2. Locations
    let location_nutrients: Vec<_> = extracted
        .nutrients
        .iter()
        .filter(|n| n.nutrient_type == "location")
        .collect();
    
    if !location_nutrients.is_empty() {
        println!("📍 Locations Detected ({} unique):", location_nutrients.len());
        for nutrient in &location_nutrients {
            let default_zero = "0".to_string();
            let evidence_count = nutrient.metadata.get("evidence_count").unwrap_or(&default_zero);
            println!(
                "  • {} (confidence: {:.2}, evidence: {})",
                nutrient.value, nutrient.confidence, evidence_count
            );
        }
        println!();
    } else {
        println!("📍 Locations: None detected");
        println!();
    }
    
    // 3. Age
    let age_nutrients: Vec<_> = extracted
        .nutrients
        .iter()
        .filter(|n| n.nutrient_type == "age")
        .collect();
    
    if !age_nutrients.is_empty() {
        println!("🎂 Age Information ({} mentions):", age_nutrients.len());
        for nutrient in &age_nutrients {
            let default_na = "N/A".to_string();
            let timestamp = nutrient.metadata.get("timestamp").unwrap_or(&default_na);
            println!(
                "  • {} years old (confidence: {:.2}, mentioned: {})",
                nutrient.value, nutrient.confidence, timestamp
            );
        }
        println!();
    } else {
        println!("🎂 Age: Not mentioned");
        println!();
    }
    
    // 4. Occupation Indicators
    let occupation_nutrients: Vec<_> = extracted
        .nutrients
        .iter()
        .filter(|n| n.nutrient_type == "occupation_indicator")
        .collect();
    
    if !occupation_nutrients.is_empty() {
        println!("💼 Occupation Indicators ({} keywords):", occupation_nutrients.len());
        
        // Sort by frequency
        let mut sorted: Vec<_> = occupation_nutrients.clone();
        sorted.sort_by(|a, b| {
            let default_zero = "0".to_string();
            let freq_a: usize = a.metadata.get("frequency").unwrap_or(&default_zero).parse().unwrap_or(0);
            let freq_b: usize = b.metadata.get("frequency").unwrap_or(&default_zero).parse().unwrap_or(0);
            freq_b.cmp(&freq_a)
        });
        
        for nutrient in sorted.iter().take(10) {
            let default_zero = "0".to_string();
            let freq = nutrient.metadata.get("frequency").unwrap_or(&default_zero);
            println!(
                "  • \"{}\" mentioned {} times (confidence: {:.2})",
                nutrient.value, freq, nutrient.confidence
            );
        }
        
        if sorted.len() > 10 {
            println!("  ... and {} more", sorted.len() - 10);
        }
        println!();
    } else {
        println!("💼 Occupation: No indicators found");
        println!();
    }
    
    // ============================================================
    // CONFIDENCE ANALYSIS
    // ============================================================
    
    println!("🎯 Confidence Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    let mut high_confidence = 0;
    let mut medium_confidence = 0;
    let mut low_confidence = 0;
    
    for nutrient in &extracted.nutrients {
        if nutrient.confidence >= 0.7 {
            high_confidence += 1;
        } else if nutrient.confidence >= 0.5 {
            medium_confidence += 1;
        } else {
            low_confidence += 1;
        }
    }
    
    let total = extracted.nutrients.len() as f64;
    if total > 0.0 {
        println!("  High (≥0.7):   {} ({:.1}%)", high_confidence, high_confidence as f64 / total * 100.0);
        println!("  Medium (≥0.5): {} ({:.1}%)", medium_confidence, medium_confidence as f64 / total * 100.0);
        println!("  Low (<0.5):    {} ({:.1}%)", low_confidence, low_confidence as f64 / total * 100.0);
    }
    println!();
    
    // ============================================================
    // PERFORMANCE ASSESSMENT
    // ============================================================
    
    println!("⚡ Performance Assessment");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    let messages_per_ms = extracted.extraction_metadata.entries_processed as f64 
        / extracted.extraction_metadata.duration_ms as f64;
    
    println!("  Throughput:       {:.2} messages/ms", messages_per_ms);
    println!("  Nutrients/Entry:  {:.2}", 
        extracted.nutrients.len() as f64 / extracted.extraction_metadata.entries_processed as f64);
    
    // Target: <10s for 1000 entries
    let target_ms = 10_000; // 10 seconds
    let target_entries = 1000;
    let target_throughput = target_entries as f64 / target_ms as f64;
    
    let speedup = messages_per_ms / target_throughput;
    
    println!("  Target:           {:.2} messages/ms", target_throughput);
    println!("  Performance:      {:.0}x faster than target", speedup);
    println!();
    
    // ============================================================
    // EXTRACTION READINESS ASSESSMENT
    // ============================================================
    
    println!("🎯 Extraction Readiness Assessment");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    let has_names = !name_nutrients.is_empty();
    let has_locations = !location_nutrients.is_empty();
    let has_age = !age_nutrients.is_empty();
    let has_occupation = !occupation_nutrients.is_empty();
    
    println!("  Names:       {}", if has_names { "✓ Present" } else { "✗ Missing" });
    println!("  Locations:   {}", if has_locations { "✓ Present" } else { "✗ Missing" });
    println!("  Age:         {}", if has_age { "✓ Present" } else { "✗ Missing" });
    println!("  Occupation:  {}", if has_occupation { "✓ Present" } else { "✗ Missing" });
    println!();
    
    let fields_present = [has_names, has_locations, has_age, has_occupation]
        .iter()
        .filter(|&&x| x)
        .count();
    
    let readiness = fields_present as f64 / 4.0 * 100.0;
    
    println!("  Overall Readiness: {:.0}% ({}/4 fields)", readiness, fields_present);
    
    if readiness >= 75.0 {
        println!("  Status: ✅ EXCELLENT - Rich biographical data");
    } else if readiness >= 50.0 {
        println!("  Status: ✓ GOOD - Sufficient biographical data");
    } else if readiness >= 25.0 {
        println!("  Status: ⚠ FAIR - Partial biographical data");
    } else {
        println!("  Status: ✗ POOR - Limited biographical data");
    }
    println!();
    
    // ============================================================
    // FINAL SUMMARY
    // ============================================================
    
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  ✅ BiographicalExtractor Test Complete                   ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    println!("Summary:");
    println!("  • {} entries processed", extracted.extraction_metadata.entries_processed);
    println!("  • {} biographical nutrients extracted", extracted.nutrients.len());
    println!("  • {:.0}x faster than target performance", speedup);
    println!("  • {:.0}% extraction readiness", readiness);
    println!();
    
    Ok(())
}
