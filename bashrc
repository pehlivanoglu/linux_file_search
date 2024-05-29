
# --------------------------------linux_file_search------------------------------
function check_for_file_in_args() {
    # Regex to detect file patterns enclosed in brackets, e.g., [filename.extension]
    local file_regex='\[(.*?)\]'

    # Check if the command is likely triggered by tab completion
    if [[ -n "$COMP_LINE" && $COMP_POINT -lt ${#COMP_LINE} ]]; then
        return 0  # Skip processing if triggered by tab completion
    fi

    # Temporarily clear the DEBUG trap to avoid recursion when running commands within the trap
    trap - DEBUG

    # Check if the command includes a filename formatted in brackets
    if [[ "$BASH_COMMAND" =~ $file_regex ]]; then
        sudo ~/CS350/project/linux_file_search/target/debug/search "${BASH_COMMAND}"
        trap 'check_for_file_in_args' DEBUG
        return 1  # Prevent the original command from executing
    fi

    # Reinstate the DEBUG trap for the next command
    trap 'check_for_file_in_args' DEBUG
}

# Set the DEBUG trap to intercept and check each command
trap 'check_for_file_in_args' DEBUG

# Ensure shell options are propagated correctly
shopt -s extdebug

# -------------------------------------------------------------------------------