use std::collections::HashMap;
use std::env;
use std::process;
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    match parent_process_name().as_str() {
        "bash" => println!("{}", bash(completers())),
        "elvish" => println!("{}", elvish(completers())),
        "fish" => println!("{}", fish(completers())),
        "oil.ovm" => println!("{}", bash(completers())),
        "pwsh" => println!("{}", powershell(completers())),
        "powershell" => println!("{}", powershell(completers())),
        "powershell.exe" => println!("{}", powershell(completers())),
        p if p.starts_with("python") => println!("{}", xonsh(completers())),
        "zsh" => println!("{}", zsh(completers())),
        p => println!("unknown shell: {}", p),
    }
}

fn completers() -> HashMap<String, String> {
    let mut completers = HashMap::new();

    let args: Vec<String> = env::args().collect();
    if args.len() % 2 == 0 {
        panic!("invalid amount of arguments")
    }

    for n in 1..args.len() {
        if n % 2 == 1 {
            completers.insert(args[n].to_string(), args[n + 1].to_string());
        }
    }
    return completers;
}

fn bash(c: HashMap<String, String>) -> String {
    let mut elems: Vec<String> = Vec::new();
    for (k, v) in &c {
        elems.append(&mut vec![format!(
            "_lazycomplete_{}() {{
  unset -f _lazycomplete_{}
  source <({})
  $\"$(complete -p {} | awk '{{print $3}}')\"
}}
complete -F _lazycomplete_{} {}
",
            k, k, v, k, k, k
        )]);
    }

    return elems.join("\n");
}

fn elvish(c: HashMap<String, String>) -> String {
    let mut elems: Vec<String> = Vec::new();
    for (k, v) in &c {
        elems.append(&mut vec![format!(
            "edit:completion:arg-completer[{}] = [@arg]{{
      edit:completion:arg-completer[{}] = [@arg]{{}}
      eval ({} | slurp)
      $edit:completion:arg-completer[{}] $@arg
}}
",
            k, k, v, k
        )]);
    }

    return elems.join("\n");
}

fn fish(c: HashMap<String, String>) -> String {
    let mut elems: Vec<String> = Vec::new();
    for (k, v) in &c {
        elems.append(&mut vec![format!(
            "function _lazycomplete_{}
   functions --erase _lazycomplete_{}
   complete -c '{}' --erase
   {} | source
   complete --do-complete=(commandline -cp)
end
complete -c '{}' --erase
complete -c '{}' -f -a '(_lazycomplete_{} {})'
",
            k, k, k, v, k, k, k, k
        )]);
    }

    return elems.join("\n");
}

fn powershell(c: HashMap<String, String>) -> String {
    let mut elems: Vec<String> = Vec::new();
    for (k, v) in &c {
        elems.append(&mut vec![format!(
            "Function _lazycomplete_{} {{
    param($wordToComplete, $commandAst, $cursorPosition)
    Register-ArgumentCompleter -Native -CommandName '{}' -ScriptBlock {{}}
    {} | out-string | invoke-expression
    [System.Management.Automation.CommandCompletion]::CompleteInput($commandAst.ToString().PadRight($cursorPosition, ' ').Substring(0, $cursorPosition), $cursorPosition, $null).CompletionMatches
}}
Register-ArgumentCompleter -Native -CommandName '{}' -ScriptBlock (Get-Item \"Function:_lazycomplete_{}\").ScriptBlock
",
            k, k, v, k, k
        )]);
    }

            return String::from("using namespace System.Management.Automation
using namespace System.Management.Automation.Language
") + &elems.join("\n");
}
fn xonsh(c: HashMap<String, String>) -> String {
    let mut elems: Vec<String> = Vec::new();
    for (k, v) in &c {
        elems.append(&mut vec![format!(
            "def _lazycomplete_{}(prefix, line, begidx, endidx, ctx):
    \"\"\"lazycomplete {}\"\"\"
    import xonsh
    import subprocess
    import builtins
    if not line.startswith('{} '):
        return # not the expected command to complete
    #del _lazycomplete_{} 
    builtins.__xonsh__.completers = builtins.__xonsh__.completers.copy()
    del builtins.__xonsh__.completers['{}']
    exec(compile(subprocess.run('{}'.split(), stdout=subprocess.PIPE).stdout.decode('utf-8'), \"\", \"exec\"))
    return builtins.__xonsh__.completers['{}'](prefix, line, begidx, endidx, ctx)
from xonsh.completers._aliases import _add_one_completer
_add_one_completer('{}', _lazycomplete_{}, 'start')
",
            k, k,k, k, k, v, k, k, k
        )]);
    }

    return elems.join("\n");
}

fn zsh(c: HashMap<String, String>) -> String {
    let mut elems: Vec<String> = Vec::new();
    for (k, v) in &c {
        elems.append(&mut vec![format!(
            "function _lazycomplete_{} {{
    compdef -d {}
    unfunction _lazycomplete_{}
    source <({})
}}
compdef _lazycomplete_{} {}
",
            k, k, k, v, k, k
        )]);
    }

    return elems.join("\n");
}

fn parent_process_name() -> String {
    let mut system = System::new();
    let parent_pid: i32;
    {
        system.refresh_process(process::id() as i32);
        let current = system.get_process(process::id() as i32).unwrap();
        parent_pid = current.parent().unwrap();
    }
    system.refresh_process(parent_pid as i32);

    let parent = system.get_process(parent_pid).unwrap();
    parent.name().to_string()
}
