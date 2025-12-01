// Test: TemporalExtractor with real WhatsApp data
// Purpose: Validate temporal pattern extraction (routines, frequency, life events)

use bitacora::data_import::{
    digestion::{DigestionPipeline, WhatsAppDigester},
    extraction::{NutrientExtractor, TemporalExtractor},
    DataSource, QuarantineZone,
};

use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  ⏰ TemporalExtractor - Real Data Test                    ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    // ============================================================
    // PHASE 1: Quarantine
    // ============================================================
    
    println!("📦 [1/3] Quarantine Phase");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let source_path = PathBuf::from("data/imports/whatsapp/_chat.txt");
    
    if !source_path.exists() {
        eprintln!("❌ Error: Test data not found at {:?}", source_path);
        eprintln!("   Please ensure data/imports/whatsapp/_chat.txt exists");
        std::process::exit(1);
    }
    
    let raw_data = std::fs::read(&source_path)?;
    let quarantine = QuarantineZone::inspect(raw_data, DataSource::WhatsApp).await?;
    
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
    // PHASE 3: Temporal Extraction
    // ============================================================
    
    println!("⏰ [3/3] Temporal Pattern Extraction");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let extractor = TemporalExtractor::new();
    
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
    println!("  Total Entries:     {}", digested.entries.len());
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
    // DETAILED RESULTS BY PATTERN TYPE
    // ============================================================
    
    println!("🔍 Detailed Temporal Patterns");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    // 1. Time of day pattern
    let time_of_day = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "time_of_day");
    
    if let Some(nutrient) = time_of_day {
        let default_unknown = "unknown".to_string();
        let default_zero = "0".to_string();
        let most_active = nutrient.metadata.get("most_active").unwrap_or(&default_unknown);
        let morning = nutrient.metadata.get("morning_pct").unwrap_or(&default_zero);
        let afternoon = nutrient.metadata.get("afternoon_pct").unwrap_or(&default_zero);
        let evening = nutrient.metadata.get("evening_pct").unwrap_or(&default_zero);
        let night = nutrient.metadata.get("night_pct").unwrap_or(&default_zero);
        
        println!("⏰ Time of Day Pattern:");
        println!("  Most Active: 📍 {}", most_active);
        println!("  Morning (6am-12pm):    {}", morning);
        println!("  Afternoon (12pm-6pm):  {}", afternoon);
        println!("  Evening (6pm-11pm):    {}", evening);
        println!("  Night (11pm-6am):      {}", night);
        println!();
    }
    
    // 2. Weekday/weekend pattern
    let weekday_weekend = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "weekday_weekend");
    
    if let Some(nutrient) = weekday_weekend {
        let default_zero = "0".to_string();
        let weekday_pct = nutrient.metadata.get("weekday_pct").unwrap_or(&default_zero);
        let weekend_pct = nutrient.metadata.get("weekend_pct").unwrap_or(&default_zero);
        
        println!("📅 Weekday/Weekend Pattern:");
        println!("  Weekday: {}", weekday_pct);
        println!("  Weekend: {}", weekend_pct);
        println!();
    }
    
    // 3. Frequency pattern
    let frequency = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "frequency");
    
    if let Some(nutrient) = frequency {
        let default_zero = "0".to_string();
        let messages_per_day = nutrient.metadata.get("messages_per_day").unwrap_or(&default_zero);
        
        println!("📈 Communication Frequency:");
        println!("  Pattern: {}", nutrient.value);
        println!("  Messages/day: {}", messages_per_day);
        println!();
    }
    
    // 4. Response time
    let response_time = extracted.nutrients.iter()
        .find(|n| n.nutrient_type == "response_time");
    
    if let Some(nutrient) = response_time {
        println!("⏱️  Response Time:");
        println!("  Average: {}", nutrient.value);
        println!();
    } else {
        println!("⏱️  Response Time: Not available (single author or insufficient data)");
        println!();
    }
    
    // 5. Life events
    let life_events: Vec<_> = extracted.nutrients.iter()
        .filter(|n| n.nutrient_type == "life_event")
        .collect();
    
    if !life_events.is_empty() {
        println!("🎉 Life Events Detected ({}):", life_events.len());
        
        let default_unknown = "unknown".to_string();
        let default_na = "N/A".to_string();
        let default_empty = "".to_string();
        for event in life_events.iter().take(10) {
            let event_type = event.metadata.get("event_type").unwrap_or(&default_unknown);
            let timestamp = event.metadata.get("timestamp").unwrap_or(&default_na);
            let preview = event.metadata.get("description_preview").unwrap_or(&default_empty);
            
            println!("  • {} - {} (confidence: {:.2})", 
                event_type, timestamp, event.confidence);
            if !preview.is_empty() {
                println!("    Preview: {}", preview);
            }
        }
        
        if life_events.len() > 10 {
            println!("  ... and {} more", life_events.len() - 10);
        }
        println!();
    } else {
        println!("🎉 Life Events: None detected");
        println!();
    }
    
    // ============================================================
    // PATTERN INSIGHTS
    // ============================================================
    
    println!("💡 Pattern Insights");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    // Most active time
    if let Some(time_nutrient) = time_of_day {
        let default_unknown = "unknown".to_string();
        let most_active = time_nutrient.metadata.get("most_active").unwrap_or(&default_unknown);
        match most_active.as_str() {
            "morning" => println!("  🌅 Morning person - Most active 6am-12pm"),
            "afternoon" => println!("  ☀️  Afternoon communicator - Most active 12pm-6pm"),
            "evening" => println!("  🌆 Evening chatter - Most active 6pm-11pm"),
            "night" => println!("  🌙 Night owl - Most active 11pm-6am"),
            _ => println!("  ❓ Activity pattern unclear"),
        }
    }
    
    // Frequency insight
    if let Some(freq_nutrient) = frequency {
        match freq_nutrient.value.as_str() {
            "MultipleDaily" => println!("  💬 Very active conversation - 20+ messages/day"),
            "Daily" => println!("  📱 Daily communication - 5-20 messages/day"),
            "Weekly" => println!("  📅 Regular check-ins - Weekly communication"),
            "Monthly" => println!("  📆 Occasional contact - Monthly communication"),
            "Sporadic" => println!("  🔕 Sporadic communication - Rare messages"),
            _ => {}
        }
    }
    
    // Weekday/weekend insight
    if let Some(weekday_nutrient) = weekday_weekend {
        if let Ok(weekday_pct) = weekday_nutrient.metadata.get("weekday_pct")
            .unwrap_or(&"0".to_string()).parse::<f64>() {
            
            if weekday_pct > 0.7 {
                println!("  💼 Weekday-focused communication (work-related?)");
            } else if weekday_pct < 0.3 {
                println!("  🎉 Weekend-focused communication (social?)");
            } else {
                println!("  ⚖️  Balanced weekday/weekend communication");
            }
        }
    }
    
    println!();
    
    // ============================================================
    // PERFORMANCE ASSESSMENT
    // ============================================================
    
    println!("⚡ Performance Assessment");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    let entries_per_ms = extracted.extraction_metadata.entries_processed as f64 
        / extracted.extraction_metadata.duration_ms as f64;
    
    println!("  Throughput:       {:.2} entries/ms", entries_per_ms);
    println!("  Nutrients/Entry:  {:.3}", 
        extracted.nutrients.len() as f64 / extracted.extraction_metadata.entries_processed as f64);
    
    // Target: <10s for 1000 entries
    let target_ms = 10_000; // 10 seconds
    let target_entries = 1000;
    let target_throughput = target_entries as f64 / target_ms as f64;
    
    let speedup = entries_per_ms / target_throughput;
    
    println!("  Target:           {:.2} entries/ms", target_throughput);
    println!("  Performance:      {:.0}x faster than target", speedup);
    println!();
    
    // ============================================================
    // EXTRACTION QUALITY ASSESSMENT
    // ============================================================
    
    println!("🎯 Extraction Quality Assessment");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    
    let has_time_of_day = time_of_day.is_some();
    let has_weekday = weekday_weekend.is_some();
    let has_frequency = frequency.is_some();
    let has_response_time = response_time.is_some();
    let has_life_events = !life_events.is_empty();
    
    println!("  Time of Day:     {}", if has_time_of_day { "✓ Present" } else { "✗ Missing" });
    println!("  Weekday Pattern: {}", if has_weekday { "✓ Present" } else { "✗ Missing" });
    println!("  Frequency:       {}", if has_frequency { "✓ Present" } else { "✗ Missing" });
    println!("  Response Time:   {}", if has_response_time { "✓ Present" } else { "○ Optional (N/A)" });
    println!("  Life Events:     {}", if has_life_events { "✓ Present" } else { "○ Optional (none found)" });
    println!();
    
    let core_patterns = [has_time_of_day, has_weekday, has_frequency]
        .iter()
        .filter(|&&x| x)
        .count();
    
    let quality = core_patterns as f64 / 3.0 * 100.0;
    
    println!("  Core Patterns: {}/3", core_patterns);
    println!("  Quality Score: {:.0}%", quality);
    
    if quality >= 90.0 {
        println!("  Status: ✅ EXCELLENT - Complete temporal analysis");
    } else if quality >= 75.0 {
        println!("  Status: ✓ GOOD - Strong temporal patterns");
    } else if quality >= 50.0 {
        println!("  Status: ⚠ FAIR - Basic patterns detected");
    } else {
        println!("  Status: ✗ POOR - Insufficient data");
    }
    println!();
    
    // ============================================================
    // FINAL SUMMARY
    // ============================================================
    
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  ✅ TemporalExtractor Test Complete                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    println!("Summary:");
    println!("  • {} entries analyzed", extracted.extraction_metadata.entries_processed);
    println!("  • {} temporal nutrients extracted", extracted.nutrients.len());
    println!("  • {:.0}x faster than target performance", speedup);
    println!("  • {:.0}% quality score", quality);
    println!();
    
    Ok(())
}
