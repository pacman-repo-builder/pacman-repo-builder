#compdef strip-ansi

autoload -U is-at-least

_strip-ansi() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::TEXT -- Input text. Use stdin if not specified:_files' \
&& ret=0
    
}

(( $+functions[_strip-ansi_commands] )) ||
_strip-ansi_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'strip-ansi commands' commands "$@"
}

_strip-ansi "$@"