mod builtins;

use crate::terminal::CommandContext;
use std::collections::HashMap;

pub type CommandFn = fn(&[String], &mut CommandContext) -> Result<String, String>;

pub struct CommandRegistry {
    map: HashMap<String, CommandFn>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        register_builtins(&mut map);
        CommandRegistry { map }
    }

    pub fn get(&self, name: &str) -> Option<CommandFn> {
        self.map.get(name).copied()
    }
}

fn register_builtins(map: &mut HashMap<String, CommandFn>) {
    use crate::commands::builtins::*;

    // Навигация и файлы
    map.insert("cd".to_string(), cmd_cd);
    map.insert("chdir".to_string(), cmd_cd);
    map.insert("directory".to_string(), cmd_cd);
    map.insert("ls".to_string(), cmd_ls);
    map.insert("dir".to_string(), cmd_ls);
    map.insert("list".to_string(), cmd_ls);
    map.insert("pwd".to_string(), cmd_pwd);

    // Работа с файлами
    map.insert("cat".to_string(), cmd_cat);
    map.insert("type".to_string(), cmd_cat);
    map.insert("show".to_string(), cmd_cat);
    map.insert("echo".to_string(), cmd_echo);
    // Новые файловые команды
    map.insert("mkdir".to_string(), cmd_mkdir);
    map.insert("md".to_string(), cmd_mkdir);
    map.insert("makedir".to_string(), cmd_mkdir);
    map.insert("touch".to_string(), cmd_touch);
    map.insert("create".to_string(), cmd_touch);
    map.insert("cp".to_string(), cmd_cp);
    map.insert("copy".to_string(), cmd_cp);
    map.insert("mv".to_string(), cmd_mv);
    map.insert("move".to_string(), cmd_mv);
    map.insert("rename".to_string(), cmd_mv);
    map.insert("rm".to_string(), cmd_rm);
    map.insert("del".to_string(), cmd_rm);
    map.insert("delete".to_string(), cmd_rm);
    map.insert("find".to_string(), cmd_find);
    map.insert("search".to_string(), cmd_find);
    map.insert("tree".to_string(), cmd_tree);
    map.insert("du".to_string(), cmd_du);
    map.insert("df".to_string(), cmd_df);
    map.insert("diskspace".to_string(), cmd_df);
    map.insert("stat".to_string(), cmd_stat);
    map.insert("fileinfo".to_string(), cmd_stat);

    // Системная информация
    map.insert("whoami".to_string(), cmd_whoami);
    map.insert("date".to_string(), cmd_date);
    map.insert("time".to_string(), cmd_time);
    map.insert("sysinfo".to_string(), cmd_sysinfo);
    map.insert("system".to_string(), cmd_sysinfo);
    map.insert("info".to_string(), cmd_sysinfo);
    map.insert("neofetch".to_string(), cmd_sysinfo);
    map.insert("uptime".to_string(), cmd_uptime);
    map.insert("hostname".to_string(), cmd_hostname);

    // Процессы
    map.insert("ps".to_string(), cmd_ps);
    map.insert("processes".to_string(), cmd_ps);
    map.insert("tasklist".to_string(), cmd_ps);
    map.insert("kill".to_string(), cmd_kill);
    map.insert("terminate".to_string(), cmd_kill);
    map.insert("end".to_string(), cmd_kill);

    // Сеть
    map.insert("ping".to_string(), cmd_ping);
    map.insert("ip".to_string(), cmd_ipconfig);
    map.insert("ipconfig".to_string(), cmd_ipconfig);
    map.insert("netstat".to_string(), cmd_netstat);
    map.insert("curl".to_string(), cmd_curl);
    map.insert("download".to_string(), cmd_curl);

    // Текст
    map.insert("grep".to_string(), cmd_grep);
    map.insert("findstr".to_string(), cmd_grep);
    map.insert("head".to_string(), cmd_head);
    map.insert("tail".to_string(), cmd_tail);
    map.insert("wc".to_string(), cmd_wc);
    map.insert("sort".to_string(), cmd_sort);
    map.insert("uniq".to_string(), cmd_uniq);

    // Управление терминалом
    map.insert("clear".to_string(), cmd_clear);
    map.insert("cls".to_string(), cmd_clear);
    map.insert("history".to_string(), cmd_history);
    map.insert("alias".to_string(), cmd_alias);
    map.insert("set".to_string(), cmd_set);
    map.insert("bgcolor".to_string(), cmd_bgcolor);
    map.insert("fgcolor".to_string(), cmd_fgcolor);
    map.insert("calc".to_string(), cmd_calc);
    map.insert("math".to_string(), cmd_calc);
    map.insert("run".to_string(), cmd_run);
    map.insert("execute".to_string(), cmd_run);
    map.insert("start".to_string(), cmd_run);

    // Системные действия
    map.insert("shutdown".to_string(), cmd_shutdown);
    map.insert("reboot".to_string(), cmd_reboot);
    map.insert("startup".to_string(), cmd_startup);

    // Справка и выход
    map.insert("help".to_string(), cmd_help);
    map.insert("exit".to_string(), cmd_exit);
    map.insert("quit".to_string(), cmd_exit);
}