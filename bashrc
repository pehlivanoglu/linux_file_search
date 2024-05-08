# --------------------------------linux_file_search------------------------------
# Enable extended debugging to control command execution
shopt -s extdebug

# Function to check if the command includes a file in brackets and execute a custom command
function check_for_file_in_args() {
    # Regex to detect file patterns enclosed in brackets, e.g., [filename.extension]
    local file_regex='\[([[:alnum:]_\.\-\/]+\.[[:alnum:]]+)\]$'

    # Check if the command is likely triggered by tab completion
    if [[ -n "$COMP_LINE" && $COMP_POINT -lt ${#COMP_LINE} ]]; then
        return 0  # Skip processing if triggered by tab completion
    fi

    # Temporarily clear the DEBUG trap to avoid recursion when running commands within the trap
    trap - DEBUG

    # Check if the command includes a filename formatted in brackets
    if [[ "$BASH_COMMAND" =~ $file_regex ]]; then
        echo "Executing custom command for file: ${BASH_REMATCH[1]}"
        # Execute the custom command
        ~/CS350/project/file_search/target/debug/search "${BASH_REMATCH[1]}"
        # Block the original command from executing by returning 1
        trap 'check_for_file_in_args' DEBUG
        return 1
    fi

    # Reinstate the DEBUG trap for the next command
    trap 'check_for_file_in_args' DEBUG
}

# Set the DEBUG trap to intercept and check each command
trap 'check_for_file_in_args' DEBUG

# Ensure the DEBUG trap and other shell options are propagated correctly
set -T
# -------------------------------------------------------------------------------
