# SPDX-FileCopyrightText: Copyright © 2023 {name}. All rights reserved.
# SPDX-License-Identifier: {license}

# See https://github.com/rust-lang/rustfmt/blob/master/Configurations.md
# for more configuration options

comment_width = 72                      # Maximum line width for comments
doc_comment_code_block_width = 72       # Maximum line width for code blocks in doc comments
edition = "2021"                        # Use a single edition only (Edition 2018 or Edition 2021)
empty_item_single_line = true           # Put empty items on a single line
force_explicit_abi = true               # Force explicit abi
format_code_in_doc_comments = true      # Format code snippets in doc comments
format_macro_bodies = true              # Format macro bodies
format_macro_matchers = true            # Format macro matchers
group_imports = "StdExternalCrate"      # Group imports by crate
hard_tabs = false                       # Use spaces instead of tabs
imports_granularity = "Module"          # Group imports by module
imports_layout = "HorizontalVertical"   # Layout imports horizontally and vertically
max_width = 72                          # Maximum line width
merge_derives = true                    # Merge derives
newline_style = "Unix"                  # Prevent carriage returns from being added to the end of lines
normalize_comments = true               # Normalize comments
normalize_doc_attributes = true         # Normalize doc attributes
overflow_delimited_expr = true          # Allow overflowing delimited expressions
remove_nested_parens = true             # Remove nested parens
reorder_imports = true                  # Reorder imports
reorder_modules = true                  # Reorder modules
tab_spaces = 4                          # Use 4 spaces for indentation
use_field_init_shorthand = true         # Use field initialization shorthand when possible
use_small_heuristics = "Max"            # Use max heuristics
use_try_shorthand = true                # Use try shorthand when possible
wrap_comments = true                    # Wrap comments when line width exceeds max max_width