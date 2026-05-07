import re

css = open('assets/main.css').read()

replacements = {
    r'rgba\(25, 28, 41, 0\.9\)': 'var(--header-bg)',
    r'rgba\(25, 28, 41, 0\.95\)': 'var(--sidebar-bg)',
    r'rgba\(0, 0, 0, 0\.2\)': 'var(--input-bg)',
    r'rgba\(255, 255, 255, 0\.05\)': 'var(--button-hover-bg)',
    r'rgba\(255, 255, 255, 0\.1\)': 'var(--button-bg-light)',
    r'rgba\(255, 255, 255, 0\.2\)': 'var(--button-bg-lighter)',
    r'rgba\(99, 102, 241, 0\.1\)': 'var(--new-agent-btn-bg)',
    r'rgba\(99, 102, 241, 0\.2\)': 'var(--new-agent-btn-hover-bg)',
    r'rgba\(99, 102, 241, 0\.4\)': 'var(--new-agent-btn-border)',
    r'0 0 0 2px rgba\(99, 102, 241, 0\.2\)': '0 0 0 2px var(--focus-ring)',
    r'0 4px 14px 0 rgba\(99, 102, 241, 0\.39\)': '0 4px 14px 0 var(--button-shadow)',
    r'#e2e8f0': 'var(--text-primary)',
    r'#a5b4fc': 'var(--text-accent-light)',
    r'rgba\(239, 68, 68, 0\.1\)': 'var(--danger-bg)',
    r'rgba\(239, 68, 68, 0\.2\)': 'var(--danger-border)',
    r'#ef4444': 'var(--danger-text)',
    r'rgba\(0, 0, 0, 0\.6\)': 'var(--modal-overlay)',
    r'rgba\(99, 102, 241, 0\.15\)': 'var(--bg-gradient-1)',
    r'rgba\(236, 72, 153, 0\.1\)': 'var(--bg-gradient-2)',
    r'rgba\(99, 102, 241, 0\.5\)': 'var(--accent-color-50)',
    r'rgba\(0,0,0,0\.2\)': 'var(--shadow-color-1)',
    r'rgba\(255,255,255,0\.1\)': 'var(--button-bg-light)',
    r'rgba\(255,255,255,0\.05\)': 'var(--button-hover-bg)',
    r'#fff': 'var(--white)',
}

root_vars = """
  --header-bg: rgba(25, 28, 41, 0.9);
  --sidebar-bg: rgba(25, 28, 41, 0.95);
  --input-bg: rgba(0, 0, 0, 0.2);
  --button-hover-bg: rgba(255, 255, 255, 0.05);
  --button-bg-light: rgba(255, 255, 255, 0.1);
  --button-bg-lighter: rgba(255, 255, 255, 0.2);
  --new-agent-btn-bg: rgba(99, 102, 241, 0.1);
  --new-agent-btn-hover-bg: rgba(99, 102, 241, 0.2);
  --new-agent-btn-border: rgba(99, 102, 241, 0.4);
  --focus-ring: rgba(99, 102, 241, 0.2);
  --button-shadow: rgba(99, 102, 241, 0.39);
  --text-accent-light: #a5b4fc;
  --danger-bg: rgba(239, 68, 68, 0.1);
  --danger-border: rgba(239, 68, 68, 0.2);
  --danger-text: #ef4444;
  --modal-overlay: rgba(0, 0, 0, 0.6);
  --bg-gradient-1: rgba(99, 102, 241, 0.15);
  --bg-gradient-2: rgba(236, 72, 153, 0.1);
  --accent-color-50: rgba(99, 102, 241, 0.5);
  --shadow-color-1: rgba(0, 0, 0, 0.2);
  --white: #fff;
"""

light_vars = """
.light-mode {
  --bg-color: #f8fafc;
  --panel-bg: rgba(255, 255, 255, 0.85);
  --panel-border: rgba(0, 0, 0, 0.08);
  --text-primary: #0f172a;
  --text-secondary: #475569;
  --accent-color: #4f46e5;
  --accent-hover: #4338ca;
  --user-msg-bg: #4f46e5;
  --agent-msg-bg: rgba(241, 245, 249, 0.9);
  --shadow-sm: 0 4px 6px -1px rgba(0, 0, 0, 0.05), 0 2px 4px -1px rgba(0, 0, 0, 0.03);
  --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  --glow: 0 0 20px rgba(79, 70, 229, 0.15);
  --header-bg: rgba(255, 255, 255, 0.9);
  --sidebar-bg: rgba(255, 255, 255, 0.95);
  --input-bg: rgba(0, 0, 0, 0.03);
  --button-hover-bg: rgba(0, 0, 0, 0.05);
  --button-bg-light: rgba(0, 0, 0, 0.05);
  --button-bg-lighter: rgba(0, 0, 0, 0.1);
  --new-agent-btn-bg: rgba(79, 70, 229, 0.1);
  --new-agent-btn-hover-bg: rgba(79, 70, 229, 0.15);
  --new-agent-btn-border: rgba(79, 70, 229, 0.4);
  --focus-ring: rgba(79, 70, 229, 0.2);
  --button-shadow: rgba(79, 70, 229, 0.25);
  --text-accent-light: #4f46e5;
  --danger-bg: rgba(239, 68, 68, 0.1);
  --danger-border: rgba(239, 68, 68, 0.2);
  --danger-text: #dc2626;
  --modal-overlay: rgba(255, 255, 255, 0.5);
  --bg-gradient-1: rgba(79, 70, 229, 0.08);
  --bg-gradient-2: rgba(236, 72, 153, 0.05);
  --accent-color-50: rgba(79, 70, 229, 0.5);
  --shadow-color-1: rgba(0, 0, 0, 0.05);
  --white: #0f172a;
}
"""

css_lines = css.split('\n')
in_root = False
for i, line in enumerate(css_lines):
    if ':root {' in line:
        in_root = True
    elif in_root and '}' in line:
        css_lines.insert(i, root_vars)
        break

css_modified = '\n'.join(css_lines)

for old, new in replacements.items():
    css_modified = re.sub(old, new, css_modified)

css_modified += "\n" + light_vars

with open('assets/main.css', 'w') as f:
    f.write(css_modified)

