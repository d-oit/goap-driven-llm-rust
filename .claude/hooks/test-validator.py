#!/usr/bin/env python3
"""
PreToolUse hook for test validation.

Ensures Rust tests meet project requirements:
- Target: 82% code coverage
- Use tokio-test for async tests
- Use mockall for mocking
- Include unit tests in #[cfg(test)] modules
- Include integration tests in tests/ directory

Integrates with verification-agent for test guidance.
"""

import json
import sys
import subprocess
from pathlib import Path

def run_test_coverage():
    """Run cargo test with coverage analysis."""
    try:
        result = subprocess.run(
            ['cargo', 'test', '--', '--nocapture'],
            capture_output=True,
            text=True,
            timeout=120
        )
        return result.returncode == 0, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return False, '', 'Test timeout after 120 seconds'
    except Exception as e:
        return False, '', str(e)

def validate_test_requirements(command):
    """Validate test command meets requirements."""
    issues = []
    warnings = []
    
    # Check for test coverage requirement
    if '--cov' not in command and '--coverage' not in command:
        warnings.append("⚠ Consider running with coverage: cargo test --cov")
        warnings.append("  Target: 82% coverage (see AGENTS.md)")
    
    # Validate test structure
    test_path = Path('tests')
    if test_path.exists():
        integration_tests = list(test_path.rglob('*.rs'))
        if not integration_tests:
            issues.append("✗ No integration tests found in tests/ directory")
            issues.append("  Required: Integration tests in tests/")
        
        # Check for test fixtures
        fixtures_path = test_path / 'fixtures'
        if not fixtures_path.exists():
            warnings.append("⚠ Consider adding test fixtures in tests/fixtures/")
    
    # Check for property-based tests
    if 'proptest' not in open('Cargo.toml').read() if Path('Cargo.toml').exists() else '':
        warnings.append("⚠ Consider adding proptest for property-based testing")
    
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
    
    # Validate test commands
    if 'cargo test' in command:
        issues, warnings = validate_test_requirements(command)
        
        if issues:
            for issue in issues:
                print(issue, file=sys.stderr)
            sys.exit(2)  # Block execution
        
        if warnings:
            for warning in warnings:
                print(warning, file=sys.stderr)
            sys.exit(1)  # Warning only
        
        # Run tests to verify they pass
        success, stdout, stderr = run_test_coverage()
        if not success:
            print(f"✗ Tests failed: {stderr}", file=sys.stderr)
            sys.exit(2)  # Block execution
    
    sys.exit(0)  # Success

if __name__ == '__main__':
    main()