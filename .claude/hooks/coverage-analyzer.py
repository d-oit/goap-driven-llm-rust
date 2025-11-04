#!/usr/bin/env python3
"""
PostToolUse hook for coverage analysis.

Analyzes test coverage metrics and reports:
- Current coverage percentage
- Coverage trends
- Missing coverage areas
- Integration with verification-agent

Target: 82% coverage minimum
"""

import json
import sys
import subprocess
import re

TARGET_COVERAGE = 82.0  # Percentage

def run_coverage_analysis():
    """Run coverage analysis using cargo-tarpaulin."""
    try:
        # Try running with tarpaulin for detailed coverage
        result = subprocess.run(
            ['cargo', 'tarpaulin', '--out', 'json', '--all-features'],
            capture_output=True,
            text=True,
            timeout=180
        )
        
        if result.returncode == 0:
            return True, result.stdout, ''
        else:
            # Fallback to basic coverage check
            result = subprocess.run(
                ['cargo', 'test', '--', '--coverage'],
                capture_output=True,
                text=True,
                timeout=120
            )
            return result.returncode == 0, result.stdout, result.stderr
    except FileNotFoundError:
        # tarpaulin not installed
        return False, '', 'cargo-tarpaulin not found - install with: cargo install cargo-tarpaulin'
    except subprocess.TimeoutExpired:
        return False, '', 'Coverage analysis timeout'
    except Exception as e:
        return False, '', str(e)

def parse_coverage_output(output):
    """Parse coverage output to extract metrics."""
    coverage_data = {}
    
    # Try to parse as JSON (tarpaulin format)
    try:
        data = json.loads(output)
        if isinstance(data, dict):
            coverage_data['total_coverage'] = data.get('coverage_percent', 0.0)
    except json.JSONDecodeError:
        # Parse as text output
        coverage_match = re.search(r'coverage:\s+(\d+\.?\d*)%', output, re.IGNORECASE)
        if coverage_match:
            coverage_data['total_coverage'] = float(coverage_match.group(1))
        
        # Extract per-module coverage
        module_matches = re.findall(r'(\w+)\s+(\d+\.?\d*)%', output)
        coverage_data['modules'] = {name: float(pct) for name, pct in module_matches}
    
    return coverage_data

def analyze_coverage_gaps(coverage_data):
    """Analyze coverage gaps and provide recommendations."""
    gaps = []
    recommendations = []
    
    total_coverage = coverage_data.get('total_coverage', 0.0)
    
    if total_coverage < TARGET_COVERAGE:
        gaps.append(f"✗ Coverage {total_coverage:.1f}% below target {TARGET_COVERAGE}%")
        
        # Check module-level gaps
        if 'modules' in coverage_data:
            low_modules = [
                (name, pct) for name, pct in coverage_data['modules'].items()
                if pct < TARGET_COVERAGE
            ]
            
            if low_modules:
                gaps.append("\nModules below target:")
                for name, pct in low_modules:
                    gaps.append(f"  - {name}: {pct:.1f}%")
                
                recommendations.append("\nRecommendations:")
                recommendations.append("  • Add unit tests for uncovered code")
                recommendations.append("  • Add integration tests for module interactions")
                recommendations.append("  • Use verification-agent for test planning")
    
    return gaps, recommendations

def main():
    """Main hook execution."""
    input_data = json.load(sys.stdin)
    
    tool_name = input_data.get('tool_name', '')
    tool_input = input_data.get('tool_input', {})
    
    # Process test-related commands
    command = tool_input.get('command', '')
    
    if 'cargo test' in command or 'coverage' in command.lower():
        # Run coverage analysis
        success, output, error = run_coverage_analysis()
        
        if not success:
            print(f"⚠ Coverage analysis skipped: {error}", file=sys.stderr)
            sys.exit(0)  # Don't block on missing tool
        
        # Parse results
        coverage_data = parse_coverage_output(output)
        
        if 'total_coverage' in coverage_data:
            coverage_pct = coverage_data['total_coverage']
            
            print(f"\n📊 Coverage Analysis", file=sys.stderr)
            print(f"   Total: {coverage_pct:.1f}% (Target: {TARGET_COVERAGE}%)", file=sys.stderr)
            
            # Analyze gaps
            gaps, recommendations = analyze_coverage_gaps(coverage_data)
            
            if gaps:
                for gap in gaps:
                    print(gap, file=sys.stderr)
                
                for rec in recommendations:
                    print(rec, file=sys.stderr)
                
                # Exit with warning if coverage is low
                if coverage_pct < TARGET_COVERAGE:
                    sys.exit(1)
            else:
                print("✓ Coverage target met!", file=sys.stderr)
    
    sys.exit(0)

if __name__ == '__main__':
    main()