# Characterization Plan

Before refactoring, add exact parser-result/error cases that cover the selected
orchestration branches, all parsed layer datver shapes, and every `SOL-E-*`
error-code string. Tests must assert the real `parse_soil` public consumer and
preserve raw token-to-field association, numeric values, and compatibility
alias behavior.
