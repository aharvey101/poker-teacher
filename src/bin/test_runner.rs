use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Interactive testing tool for the poker game
/// This provides a menu-driven interface for testing different aspects of the game

fn main() {
    println!("🃏 Poker Game Interactive Testing Tool");
    println!("=====================================");
    
    loop {
        show_menu();
        
        let choice = get_user_input("Enter your choice (1-9): ");
        
        match choice.trim() {
            "1" => test_compilation(),
            "2" => run_unit_tests(),
            "3" => run_integration_tests(),
            "4" => test_game_startup(),
            "5" => test_mobile_ui(),
            "6" => run_performance_tests(),
            "7" => run_stress_tests(),
            "8" => generate_test_report(),
            "9" => {
                println!("Thanks for testing! 🎉");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
        
        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }
}

fn show_menu() {
    println!("\n📋 Test Menu:");
    println!("1. 🔧 Test Compilation");
    println!("2. 🧪 Run Unit Tests");
    println!("3. 🔗 Run Integration Tests");
    println!("4. 🚀 Test Game Startup");
    println!("5. 📱 Test Mobile UI");
    println!("6. ⚡ Run Performance Tests");
    println!("7. 💪 Run Stress Tests");
    println!("8. 📊 Generate Test Report");
    println!("9. 🚪 Exit");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn test_compilation() {
    println!("\n🔧 Testing Compilation...");
    
    let start = Instant::now();
    
    // Test debug build
    println!("Building debug version...");
    let debug_result = Command::new("cargo")
        .args(["build"])
        .output();
    
    match debug_result {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Debug build successful!");
            } else {
                println!("❌ Debug build failed:");
                println!("{}", String::from_utf8_lossy(&output.stderr));
                return;
            }
        }
        Err(e) => {
            println!("❌ Error running cargo build: {}", e);
            return;
        }
    }
    
    // Test release build
    println!("Building release version...");
    let release_result = Command::new("cargo")
        .args(["build", "--release"])
        .output();
    
    match release_result {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Release build successful!");
            } else {
                println!("❌ Release build failed:");
                println!("{}", String::from_utf8_lossy(&output.stderr));
                return;
            }
        }
        Err(e) => {
            println!("❌ Error running cargo build --release: {}", e);
            return;
        }
    }
    
    let duration = start.elapsed();
    println!("⏱️  Total build time: {:.2}s", duration.as_secs_f64());
}

fn run_unit_tests() {
    println!("\n🧪 Running Unit Tests...");
    
    let start = Instant::now();
    
    let result = Command::new("cargo")
        .args(["test", "--lib"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    
    match result {
        Ok(status) => {
            let duration = start.elapsed();
            if status.success() {
                println!("✅ Unit tests completed successfully!");
            } else {
                println!("❌ Some unit tests failed!");
            }
            println!("⏱️  Test time: {:.2}s", duration.as_secs_f64());
        }
        Err(e) => {
            println!("❌ Error running unit tests: {}", e);
        }
    }
}

fn run_integration_tests() {
    println!("\n🔗 Running Integration Tests...");
    
    let start = Instant::now();
    
    // Run integration tests
    let result = Command::new("cargo")
        .args(["test", "--test", "integration_tests"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    
    match result {
        Ok(status) => {
            if status.success() {
                println!("✅ Integration tests passed!");
            } else {
                println!("❌ Integration tests failed!");
            }
        }
        Err(e) => {
            println!("❌ Error running integration tests: {}", e);
        }
    }
    
    // Run poker-specific tests
    let poker_result = Command::new("cargo")
        .args(["test", "--test", "poker_tests"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    
    match poker_result {
        Ok(status) => {
            if status.success() {
                println!("✅ Poker tests passed!");
            } else {
                println!("❌ Poker tests failed!");
            }
        }
        Err(e) => {
            println!("❌ Error running poker tests: {}", e);
        }
    }
    
    let duration = start.elapsed();
    println!("⏱️  Integration test time: {:.2}s", duration.as_secs_f64());
}

fn test_game_startup() {
    println!("\n🚀 Testing Game Startup...");
    
    println!("Starting game (will run for 10 seconds)...");
    
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "teach-poker"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    
    match child {
        Ok(mut process) => {
            // Let it run for 10 seconds
            std::thread::sleep(Duration::from_secs(10));
            
            // Kill the process
            match process.kill() {
                Ok(_) => println!("✅ Game started successfully and was terminated after 10 seconds!"),
                Err(e) => println!("⚠️  Game started but couldn't be terminated cleanly: {}", e),
            }
            
            let _ = process.wait();
        }
        Err(e) => {
            println!("❌ Failed to start game: {}", e);
        }
    }
}

fn test_mobile_ui() {
    println!("\n📱 Testing Mobile UI...");
    
    println!("Checking mobile UI components compilation...");
    
    // Test mobile-specific compilation
    let result = Command::new("cargo")
        .args(["check", "--features", "mobile"])
        .output();
    
    match result {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Mobile UI components compile successfully!");
                
                // Try to run with mobile UI for a short time
                println!("Testing mobile UI runtime (5 seconds)...");
                
                let mut child = Command::new("cargo")
                    .args(["run"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::piped())
                    .spawn();
                
                match child {
                    Ok(mut process) => {
                        std::thread::sleep(Duration::from_secs(5));
                        let _ = process.kill();
                        let _ = process.wait();
                        println!("✅ Mobile UI runtime test completed!");
                    }
                    Err(e) => {
                        println!("⚠️  Could not test mobile UI runtime: {}", e);
                    }
                }
            } else {
                println!("❌ Mobile UI compilation failed:");
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("❌ Error checking mobile UI: {}", e);
        }
    }
}

fn run_performance_tests() {
    println!("\n⚡ Running Performance Tests...");
    
    // Test hand evaluation performance
    println!("Testing hand evaluation performance...");
    
    let result = Command::new("cargo")
        .args(["test", "--release", "performance"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    
    match result {
        Ok(status) => {
            if status.success() {
                println!("✅ Performance tests completed!");
            } else {
                println!("⚠️  Some performance tests may need attention");
            }
        }
        Err(e) => {
            println!("❌ Error running performance tests: {}", e);
        }
    }
}

fn run_stress_tests() {
    println!("\n💪 Running Stress Tests...");
    
    println!("Running all tests multiple times to check for race conditions...");
    
    for i in 1..=5 {
        println!("Stress test iteration {}/5...", i);
        
        let result = Command::new("cargo")
            .args(["test", "--", "--test-threads=1"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        
        match result {
            Ok(status) => {
                if status.success() {
                    println!("✅ Iteration {} passed", i);
                } else {
                    println!("❌ Iteration {} failed!", i);
                    break;
                }
            }
            Err(e) => {
                println!("❌ Error in stress test iteration {}: {}", i, e);
                break;
            }
        }
    }
    
    println!("Stress testing completed!");
}

fn generate_test_report() {
    println!("\n📊 Generating Test Report...");
    
    let mut report = String::new();
    report.push_str("# Poker Game Test Report\n");
    report.push_str(&format!("Generated: {}\n\n", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()));
    
    // Get project info
    report.push_str("## Project Information\n");
    
    // Count lines of code
    let find_result = Command::new("find")
        .args(["src", "-name", "*.rs", "-exec", "wc", "-l", "{}", "+"])
        .output();
    
    if let Ok(output) = find_result {
        let lines_output = String::from_utf8_lossy(&output.stdout);
        if let Some(last_line) = lines_output.lines().last() {
            report.push_str(&format!("- Lines of Rust code: {}\n", 
                last_line.trim().split_whitespace().next().unwrap_or("Unknown")));
        }
    }
    
    // Count Rust files
    let file_count_result = Command::new("find")
        .args(["src", "-name", "*.rs"])
        .output();
    
    if let Ok(output) = file_count_result {
        let file_count = String::from_utf8_lossy(&output.stdout).lines().count();
        report.push_str(&format!("- Number of Rust files: {}\n", file_count));
    }
    
    report.push_str("\n## Test Results\n");
    
    // Run tests and capture results
    let test_result = Command::new("cargo")
        .args(["test", "--", "--format", "pretty"])
        .output();
    
    match test_result {
        Ok(output) => {
            let test_output = String::from_utf8_lossy(&output.stdout);
            report.push_str("```\n");
            report.push_str(&test_output);
            report.push_str("```\n");
        }
        Err(_) => {
            report.push_str("Could not capture test results\n");
        }
    }
    
    // Write report to file
    match std::fs::write("test_report.md", report) {
        Ok(_) => println!("✅ Test report generated: test_report.md"),
        Err(e) => println!("❌ Error generating report: {}", e),
    }
}
