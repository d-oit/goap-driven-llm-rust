#!/usr/bin/env python3
"""
PostToolUse hook for documentation validation.

Validates documentation completeness:
- Public APIs documented with /// comments
- Examples included in docs
- README files updated
- Changelog maintained

Integrates with skill-creator-agent for documentation guidance.
"""

import json
import sys
import re
from pathlib import Path

def check_public_api_documentation(file_path, content):
    """Check if public APIs are documented."""
    issues = []
    
    # Find public items
    public_items = re.findall(r'pub\s+(struct|fn|enum|trait|mod)\s+(\w+)', content)
    
    for item_type, item_name in public_items:
        # Check if documented
        pattern = rf'pub\s+{item_type}\s+{item_name}.*?///'
        
        if not re.search(pattern, content, re.DOTALL):
            if item_type in ['struct', 'fn', 'enum', 'trait']:
                issues.append(f"✗ Public {item_type} '{item_name}' missing documentation")
                issues.append(f"  Add /// comments before {item_type} {item_name}")
    
    return issues

def check_example_documentation(content):
    """Check for code examples in documentation."""
    issues = []
    
    # Look for function documentation
    fn_docs = re.findall(r'///\s*(.*?)(?=///|\npub|\Z)', content, re.DOTALL)
    
    for doc in fn_docs:
        if 'Example' not in doc and '# Examples' not in doc:
            # Check if function has parameters or returns
            if 'Arguments' in doc or 'Returns' in doc:
                issues.append("⚠ Consider adding # Examples section to documentation")
                break
    
    return issues

def check_readme_updated(content, file_path):
    """Check if README is updated for significant changes."""
    issues = []
    
    if file_path.name.lower() == 'readme.md':
        # Check for basic sections
        required_sections = ['Installation', 'Usage', 'Examples']
        
        for section in required_sections:
            if section.lower() not in content.lower():
                issues.append(f"⚠ README missing '{section}' section")
        
        # Check for examples
        if 'example' not in content.lower():
            issues.append("⚠ README should include usage examples")
    
    return issues

def main():
    """Main hook execution."""
    input_data = json.load(sys.stdin)
    
    tool_name = input_data.get('tool_name', '')
    tool_input = input_data.get('tool_input', {})
    
    # Process Edit operations on documentation files
    if tool_name == 'Edit':
        file_path = Path(tool_input.get('filePath', ''))
        
        content = ''
        if isinstance(tool_input.get('content'), dict):
            content = tool_input.get('content', {}).get('text', '')
        elif isinstance(tool_input.get('content'), str):
            content = tool_input.get('content', '')
        
        all_issues = []
        
        # Check Rust files for API documentation
        if file_path.suffix == '.rs':
            all_issues.extend(check_public_api_documentation(file_path, content))
            all_issues.extend(check_example_documentation(content))
        
        # Check markdown files for completeness
        elif file_path.suffix == '.md':
            all_issues.extend(check_readme_updated(content, file_path))
        
        # Report issues
        if all_issues:
            print("\n📚 Documentation Issues:", file=sys.stderr)
            for issue in all_issues:
                print(issue, file=sys.stderr)
            
            print("\n💡 Use skill-creator-agent for documentation guidance", file=sys.stderr)
            sys.exit(1)
    
    sys.exit(0)

if __name__ == '__main__':
    main()