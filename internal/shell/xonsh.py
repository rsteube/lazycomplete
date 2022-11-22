@contextual_completer
def _lazycomplete_{{.CMD}}(context):
    """lazycomplete {{.CMD}}"""
    import xonsh
    import subprocess
    if not context.command.completing_command('{{.CMD}}'):
        return # not the expected command to complete
    XSH.completers = XSH.completers.copy()
    del XSH.completers['{{.CMD}}']
    exec(compile(subprocess.run('{{.COMPLETER}}'.split(), stdout=subprocess.PIPE).stdout.decode('utf-8'), "", "exec"))
    return XSH.completers['{{.CMD}}'](context)

_add_one_completer('{{.CMD}}', _lazycomplete_{{.CMD}}, 'start')

