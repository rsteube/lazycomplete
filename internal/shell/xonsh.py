@contextual_completer
def _lazycomplete_{{.CMD}}(context):
    """lazycomplete {{.CMD}}"""
    import xonsh
    import subprocess
    import builtins
    if not context.command.completing_command('{{.CMD}}'):
        return # not the expected command to complete
    builtins.__xonsh__.completers = builtins.__xonsh__.completers.copy()
    del builtins.__xonsh__.completers['{{.CMD}}']
    exec(compile(subprocess.run('{{.COMPLETER}}'.split(), stdout=subprocess.PIPE).stdout.decode('utf-8'), "", "exec"))
    return builtins.__xonsh__.completers['{{.CMD}}'](context)

_add_one_completer('{{.CMD}}', _lazycomplete_{{.CMD}}, 'start')

