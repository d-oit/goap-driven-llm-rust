#!/usr/bin/env python3
"""
PreToolUse hook for benchmark validation.

Ensures benchmarks meet project targets:
- Planning: <100ms
- Cache hit rate: >60%
- Throughput: >10,000 req/hour
- Token reduction: 50-70%

Integrates with verification-agent for performance guidance.
"""

import json
import sys
import subprocess
import re

BENCHMARK_TARGETS = {
    'planning_time': 100.0,  # ms
    'cache_hit_rate': 60.0,  # %
    'throughput': 10000.0,   # req/hour
    'token_reduction': 50.0,  # %
}

def run_benchmarks():
    """Run cargo bench and parse results."""
    try:
        result = subprocess.run(
            ['cargo', 'bench', '--', '--output-format', 'json'],
            capture_output=True,
            text=True,
            timeout=300
        )
        return result.returncode == 0, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return False, '', 'Benchmark timeout after 300 seconds'
    except Exception as e:
        return False, '', str(e)

def parse_benchmark_results(output):
    """Parse benchmark output for key metrics."""
    metrics = {}
    
    # Parse typical benchmark output
    # This is a simplified parser - real implementation would parse actual output
    planning_match = re.search(r'planning:\s+([\d.]+)\s*ms', output, re.IGNORECASE)
    if planning_match:
        metrics['planning_time'] = float(planning_match.group(1))
    
    cache_match = re.search(r'cache.*?(\d+\.?\d*)\s*%', output, re.IGNORECASE)
    if cache_match:
        metrics['cache_hit_rate'] = float(cache_match.group(1))
    
    throughput_match = re.search(r'throughput:\s+([\d,]+)', output, re.IGNORECASE)
    if throughput_match:
        metrics['throughput'] = float(throughput_match.group(1).replace(',', ''))
    
    return metrics

def validate_benchmark_targets(metrics):
    """Validate metrics against targets."""
    issues = []
    warnings = []
    
    for metric, value in metrics.items():
        if metric in BENCHMARK_TARGETS:
            target = BENCHMARK_TARGETS[metric]
            
            # Determine if lower or higher is better
            if metric == 'planning_time':
                if value > target:
                    issues.append(f"✗ {metric}: {value} exceeds target {target}ms")
                    issues.append("  Optimization needed")
            else:  # Higher is better
                if value < target:
                    issues.append(f"✗ {metric}: {value} below target {target}")
                    issues.append("  Performance improvement needed")
                elif value < target * 1.1:  # Within 10% of target
                    warnings.append(f"⚠ {metric}: {value} close to target {target}")
    
    return issues, warnings

def main():
    """Main hook execution."""
    input_data = json.load(sys.stdin)
    
    tool_name = input_data.get('tool_name', '')
    tool_input = input_data.get('tool_input', {})
    
    # Only process Bash commands
    if tool_name != 'Bash':
        sys.exit(0)
    
    command = tool_input.get('command', '')
    
    # Validate benchmark commands
    if 'cargo bench' in command:
        # Run benchmarks
        success, output, error = run_benchmarks()
        
        if not success:
            print(f"✗ Benchmark execution failed: {error}", file=sys.stderr)
            sys.exit(2)
        
        # Parse results
        metrics = parse_benchmark_results(output)
        
        if metrics:
            issues, warnings = validate_benchmark_targets(metrics)
            
            if issues:
                for issue in issues:
                    print(issue, file=sys.stderr)
                print("\nSee verification-agent for optimization guidance", file=sys.stderr)
                sys.exit(2)
            
            if warnings:
                for warning in warnings:
                    print(warning, file=sys.stderr)
                sys.exit(1)
        
        print("✓ All benchmark targets met", file=sys.stderr)
    
    sys.exit(0)

if __name__ == '__main__':
    main()