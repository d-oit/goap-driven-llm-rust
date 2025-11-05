#!/usr/bin/env python3
"""
PreToolUse hook for Rust code quality validation.

Validates Rust code changes against project standards:
- Error handling patterns (Result<T, Error> with thiserror/anyhow)
- Async patterns using Tokio
- Module structure (public_api.rs, internal.rs, tests.rs)
- Documentation requirements for public APIs
- Test coverage requirements (82% target)

Integrates with rust-clean-code-agent for guidance.
"""

import json
import sys
import re
from pathlib import Path

def validate_rust_code_quality(tool_input, file_path):
    """Validate Rust code against project quality standards."""
    issues = []
    warnings = []
    
    # Check if file is Rust source
    if not file_path.suffix == '.rs':
        return issues, warnings
    
    # Read file content
    try:
        content = tool_input.get('content', '')
        if isinstance(content, dict):
            content = content.get('text', '')
    except:
        content = ''
    
    # Validate error handling patterns
    if 'unwrap()' in content and 'test' not in str(file_path).lower():
        issues.append("✗ Found unwrap() without error handling (use ? operator)")
        issues.append("  Reference: AGENTS.md error handling pattern")
        issues.append("  Pattern: fn operation() -> Result<T, Error> { ... }")
    
    if 'panic!' in content and 'test' not in str(file_path).lower():
        issues.append("✗ Found panic! in production code (return errors instead)")
        issues.append("  Reference: AGENTS.md error handling standards")
    
    # Validate async patterns
    if re.search(r'async\s+fn', content) and 'tokio::' not in content:
        issues.append("⚠ Async functions should use Tokio runtime")
        issues.append("  Reference: AGENTS.md async patterns")
    
    # Check module structure for non-test files
    if 'test' not in str(file_path).lower() and 'mod.rs' not in str(file_path):
        # Check for public API documentation
        if re.search(r'pub\s+(struct|fn|enum|mod)', content):
            if not re.search(r'///.*', content):
                warnings.append("⚠ Public API missing documentation")
                warnings.append("  Add /// comments for all public items")
                warnings.append("  Reference: AGENTS.md documentation standards")
    
    # Check for Result type usage
    if re.search(r'fn\s+\w+\([^)]*\)\s*->\s*\w+', content):
        if 'Result' not in content and 'Option' not in content:
            if not any(keyword in content for keyword in ['test', 'main']):
                issues.append("⚠ Function should return Result<T, Error> for fallible operations")
                issues.append("  Reference: AGENTS.md error handling")
    
    return issues, warnings

def main():
    """Main hook execution."""
    input_data = json.load(sys.stdin)
    
    tool_name = input_data.get('tool_name', '')
    tool_input = input_data.get('tool_input', {})
    
    # Only process Edit operations on Rust files
    if tool_name != 'Edit':
        sys.exit(0)
    
    file_path = Path(tool_input.get('filePath', ''))
    
    issues, warnings = validate_rust_code_quality(tool_input, file_path)
    
    # Report issues
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        sys.exit(2)  # Block execution
    
    if warnings:
        for warning in warnings:
            print(warning, file=sys.stderr)
        sys.exit(1)  # Warning only
    
    sys.exit(0)  # Success

if __name__ == '__main__':
    main()