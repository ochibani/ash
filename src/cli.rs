use clap::{Arg, ArgAction, Command};

pub fn cli() -> Command {
        // Recognize argument and call appropriate function
        let matches = Command::new("ash")
                .about("Any Snapshot Hierarchical OS")
                .subcommand_required(true)
                .arg_required_else_help(true)
        // Auto upgrade
        .subcommand(Command::new("auto-upgrade")
                    .aliases(["au", "autoup"])
                    .about("Update a snapshot quietly")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Base rebuild
        .subcommand(Command::new("base-rebuild")
                    .aliases(["br", "rb"])
                    .about("Rebuild the base snapshot"),)
        // Base update
        .subcommand(Command::new("base-update")
                    .aliases(["bu", "ub"])
                    .about("Update the base snapshot")
                    .arg(Arg::new("noconfirm")
                         .long("noconfirm")
                         .short('y')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("noconfirm flag"),),)
        // Boot update command
        .subcommand(Command::new("boot")
                    .alias("boot-update")
                    .about("update boot of a snapshot")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Branch
        .subcommand(Command::new("branch")
                    .alias("add-branch")
                    .about("Create a new branch from snapshot")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("Snapshot number"),)
                    .arg(Arg::new("DESCRIPTION")
                         .long("description")
                         .alias("desc")
                         .short('d')
                         .num_args(1..)
                         .required(false)
                         .help("description for the snapshot"),),)
        // Check update
        .subcommand(Command::new("check")
                    .about("Check update"))
        // Chroot
        .subcommand(Command::new("chroot")
                    .aliases(["ch", "chr"])
                    .about("Open a root shell inside a snapshot")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Clone
        .subcommand(Command::new("clone")
                    .alias("cl")
                    .about("Create a copy of a snapshot (as top-level tree node)")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("DESCRIPTION")
                         .long("description")
                         .alias("desc")
                         .short('d')
                         .num_args(1..)
                         .required(false)
                         .help("description for the snapshot"),),)
        // Clone a branch
        .subcommand(Command::new("clone-branch")
                    .aliases(["cb", "cbranch"])
                    .about("Copy snapshot under same parent branch (clone as a branch)")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Clone recursively
        .subcommand(Command::new("clone-tree")
                    .alias("ct")
                    .about("clone a whole tree recursively")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Clone under a branch
        .subcommand(Command::new("clone-under")
                    .aliases(["cu", "ubranch"])
                    .about("Copy snapshot under specified parent (clone under a branch)")
                    .arg_required_else_help(true)
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(true)
                         .help("snapshot number"),)
                    .arg(Arg::new("BRANCH")
                         .long("branch")
                         .alias("br")
                         .short('b')
                         .value_parser(clap::value_parser!(i32))
                         .required(true)
                         .help("branch number"),),)
        // Current snapshot
        .subcommand(Command::new("current")
                    .alias("c")
                    .external_subcommand_value_parser(clap::value_parser!(i32))
                    .about("Show current snapshot number"))
        // Delete
        .subcommand(Command::new("del")
                    .aliases(["delete", "rem", "remove", "rm", "rm-snapshot"])
                    .about("Remove snapshot(s)/tree(s) and any branches recursively")
                    .arg_required_else_help(true)
                    .arg(Arg::new("quiet")
                         .long("quiet")
                         .short('q')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("force delete snapshot(s)"),)
                    .arg(Arg::new("nuke")
                         .long("nuke")
                         .short('n')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("nuke everything leftover from snapshot(s)"),)
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshots")
                         .alias("snaps")
                         .short('s')
                         .num_args(1..)
                         .value_parser(clap::value_parser!(i32))
                         .required(true)
                         .help("snapshot number"),),)
        // Deploy
        .subcommand(Command::new("deploy")
                    .aliases(["d", "dep"])
                    .about("Deploy a snapshot for next boot")
                    .arg(Arg::new("secondary")
                         .long("secondary")
                         .alias("sec")
                         .short('S')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("Deploy into secondary snapshot slot"),)
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Description
        .subcommand(Command::new("desc")
                    .about("set a description for a snapshot")
                    .arg_required_else_help(true)
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("DESCRIPTION")
                         .long("description")
                         .alias("desc")
                         .short('d')
                         .num_args(1..)
                         .required(true)
                         .help("description to be added"),),)
        // Diff two snapshots
        .subcommand(Command::new("diff")
                    .alias("dif")
                    .about("Show package diff between snapshots")
                    .arg_required_else_help(true)
                    .arg(Arg::new("SNAPSHOT-1")
                         .long("snapshot1")
                         .alias("snap1")
                         .short('f')
                         .value_parser(clap::value_parser!(i32))
                         .required(true)
                         .help("source snapshot"),)
                    .arg(Arg::new("SNAPSHOT-2")
                         .long("snapshot2")
                         .alias("snap2")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(true)
                         .help("target snapshot"),),)
        // Switch distros
        .subcommand(Command::new("efi-update")
                    .aliases(["efiup", "update-efi"])
                    .about("Switch to another distribution"))
        // Edit Ash configuration
        .subcommand(Command::new("edit")
                    .alias("edit-conf")
                    .about("Edit configuration for a snapshot")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .help("snapshot number"),),)
        // Edit live profile
        .subcommand(Command::new("edit-profile")
                    .alias("edit-prof")
                    .about("Edit snapshot profile")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // etc update
        .subcommand(Command::new("etc-update")
                    .alias("etc")
                    .about("update /etc"))
        // Export snapshot
        .subcommand(Command::new("export")
                    .alias("exp")
                    .about("Send the snapshot to export directory")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .required(false)
                         .value_parser(clap::value_parser!(i32))
                         .help("snapshot number"),)
                    .arg(Arg::new("DESTINATION")
                         .long("destination")
                         .alias("dest")
                         .short('d')
                         .value_parser(clap::value_parser!(String))
                         .required(false)
                         .help("set output directory"),),)
        // Fix db commands
        .subcommand(Command::new("fixdb")
                    .alias("fix")
                    .about("fix package database of a snapshot")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .required(false)
                         .value_parser(clap::value_parser!(i32))
                         .help("snapshot number"),),)
        // Switch to Windows (semi plausible deniability)
        .subcommand(Command::new("hide")
                    .about("Hide AshOS and switch to Windows"))
        // Hollow a node
        .subcommand(Command::new("hollow")
                    .about("Make a snapshot hollow (vulnerable)")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Immutability disable
        .subcommand(Command::new("immdis")
                    .aliases(["disable-mutability", "disableimm", "disimm", "dmut", "immdisable"])
                    .about("Disable immutability of a snapshot")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Immutability enable
        .subcommand(Command::new("immen")
                    .aliases(["enable-mutability", "enableimm", "enimm", "immenable", "mut"])
                    .about("Enable immutability of a snapshot")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Install command
        .subcommand(Command::new("install")
                    .alias("in")
                    .about("install package(s) inside a snapshot")
                    .arg(Arg::new("live")
                         .long("live")
                         .short('l')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("enable live install for snapshot"),)
                    .arg(Arg::new("noconfirm")
                         .long("noconfirm")
                         .short('y')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("noconfirm flag"),)
                    .arg(Arg::new("force")
                         .long("force")
                         .short('f')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("force download profile (Ignore cache)"),)
                    .arg(Arg::new("secondary")
                         .long("secondary")
                         .short('S')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .conflicts_with("live")
                         .help("deploy as secondary"),)
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("PACKAGE")
                         .long("package")
                         .alias("pkg")
                         .short('p')
                         .num_args(1..)
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PROFILE", "USER_PROFILE"])
                         .conflicts_with_all(["force", "secondary"])
                         .help("package(s) to be installed"),)
                    .arg(Arg::new("PROFILE")
                         .long("profile")
                         .alias("prof")
                         .short('P')
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PACKAGE", "USER_PROFILE"])
                         .conflicts_with("PACKAGE")
                         .help("profile(s) to be installed"),)
                    .arg(Arg::new("USER_PROFILE")
                         .long("user-profile")
                         .alias("uprof")
                         .short('U')
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PACKAGE", "PROFILE"])
                         .conflicts_with_all(["PACKAGE", "PROFILE"])
                         .help("profile(s) to be installed"),),)
        // Package list
        .subcommand(Command::new("list")
                    .alias("ls")
                    .about("Get list of installed packages in a snapshot")
                    .arg(Arg::new("exclude-dependency")
                         .long("exclude-dependency")
                         .short('e')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("exclude dependencies"),)
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Live chroot
        .subcommand(Command::new("live-chroot")
                    .aliases(["lchroot", "lc"])
                    .about("Open a shell inside currently booted snapshot with read-write access. Changes are discarded on new deployment"))
        // New
        .subcommand(Command::new("new")
                    .alias("new-tree")
                    .about("Create a new base snapshot")
                    .arg(Arg::new("DESCRIPTION")
                         .long("description")
                         .alias("desc")
                         .short('d')
                         .num_args(1..)
                         .required(false)
                         .help("description for the snapshot"),),)
        // Rebuild
        .subcommand(Command::new("rebuild")
                    .alias("reb")
                    .about("Create a new snapshot using another snapshot as the base.")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("DESCRIPTION")
                         .long("description")
                         .alias("desc")
                         .short('d')
                         .num_args(1..)
                         .required(false)
                         .help("description for the snapshot"),))
        // Refresh
        .subcommand(Command::new("refresh")
                    .alias("ref")
                    .about("Refresh package manager db of a snapshot")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Reset
        .subcommand(Command::new("reset")
                    .about("Deploy the base snapshot and remove all the other snapshots"))
        // Rollback
        .subcommand(Command::new("rollback")
                    .about("Revert the deployment to the last booted snapshot"))
        // Chroot run
        .subcommand(Command::new("run")
                    .about("Run command(s) inside another snapshot (chrooted)")
                    .arg_required_else_help(true)
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("COMMAND")
                         .long("command")
                         .alias("cmd")
                         .short('c')
                         .num_args(1..)
                         .value_parser(clap::value_parser!(String))
                         .required(true)
                         .help("command"),),)
        // Subvolumes list
        .subcommand(Command::new("sub")
                    .aliases(["subs", "subvol", "subvols", "subvolumes"])
                    .about("List subvolumes of active snapshot (currently booted)"))
        // Tree sync
        .subcommand(Command::new("sync")
                    .aliases(["tree-sync", "tsync"])
                    .about("Sync packages and configuration changes recursively (requires an internet connection)")
                    .arg(Arg::new("TREENAME")
                         .long("tree-name")
                         .alias("tname")
                         .short('t')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("force")
                         .long("force-offline")
                         .short('f')
                         .action(ArgAction::SetTrue)
                         .required(false)
                         .help("Snapshots would not get updated (potentially riskier)"),)
                    .arg(Arg::new("live")
                         .long("live")
                         .short('l')
                         .action(ArgAction::SetTrue)
                         .required(false)
                         .help("disable live sync"),),)
        // Tree install
        .subcommand(Command::new("tinstall")
                    .alias("tree-in")
                    .about("install package(s) or profile(s) from a tree recursively")
                    .arg(Arg::new("noconfirm")
                         .long("noconfirm")
                         .short('y')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("noconfirm flag"),)
                    .arg(Arg::new("force")
                         .long("force")
                         .short('f')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("force download profile (Ignore cache)"),)
                    .arg(Arg::new("secondary")
                         .long("secondary")
                         .short('S')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("deploy as secondary"),)
                    .arg(Arg::new("TREENAME")
                         .long("tree-name")
                         .alias("tname")
                         .short('t')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("PACKAGE")
                         .long("package")
                         .alias("pkg")
                         .short('p')
                         .num_args(1..)
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PROFILE", "USER_PROFILE"])
                         .conflicts_with_all(["force", "secondary"])
                         .help("package(s) to be installed"),)
                    .arg(Arg::new("PROFILE")
                         .long("profile")
                         .alias("prof")
                         .short('P')
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PACKAGE", "USER_PROFILE"])
                         .conflicts_with("PACKAGE")
                         .help("profile(s) to be installed"),)
                    .arg(Arg::new("USER_PROFILE")
                         .long("user-profile")
                         .alias("uprof")
                         .short('U')
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PACKAGE", "PROFILE"])
                         .conflicts_with_all(["PACKAGE", "PROFILE"])
                         .help("profile(s) to be installed"),),)
        // tmp (clear tmp)
        .subcommand(Command::new("tmp")
                    .alias("tmpclear")
                    .about("Show ash tree"))
        // Tree
        .subcommand(Command::new("tree")
                    .alias("t")
                    .about("Show ash tree"))
        // Tree remove
        .subcommand(Command::new("tremove")
                    .alias("tree-rm")
                    .about("Uninstall package(s) or profile(s) from a tree recursively")
                    .arg(Arg::new("TREENAME")
                         .long("tree-name")
                         .alias("tname")
                         .short('t')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("noconfirm")
                         .long("noconfirm")
                         .short('y')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("noconfirm flag"),)
                    .arg(Arg::new("PACKAGE")
                         .long("package")
                         .alias("pkg")
                         .short('p')
                         .num_args(1..)
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present("PROFILE")
                         .conflicts_with_all(["PROFILE", "USER_PROFILE"])
                         .help("package(s) to be uninstalled"),)
                    .arg(Arg::new("PROFILE")
                         .long("profile")
                         .alias("prof")
                         .short('P')
                         .num_args(1..)
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PACKAGE", "USER_PROFILE"])
                         .help("profile(s) to be uninstalled"),)
                    .arg(Arg::new("USER_PROFILE")
                         .long("user-profile")
                         .alias("uprof")
                         .short('U')
                         .num_args(1..)
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PACKAGE", "PROFILE"])
                         .help("profile(s) to be uninstalled"),),)
        // Tree run
        .subcommand(Command::new("trun")
                    .alias("tree-run")
                    .about("Execute command(s) inside another snapshot and all snapshots below it")
                    .arg(Arg::new("TREENAME")
                         .long("tree-name")
                         .alias("tname")
                         .short('t')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("COMMAND")
                         .long("command")
                         .alias("cmd")
                         .short('c')
                         .num_args(1..)
                         .required(true)
                         .help("command(s) to run"),),)
        // Tree upgrade
        .subcommand(Command::new("tupgrade")
                    .aliases(["tree-upgrade", "tup"])
                    .about("Update all packages in a snapshot recursively")
                    .arg(Arg::new("TREENAME")
                         .long("tree-name")
                         .alias("tname")
                         .short('t')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Uninstall package(s) from a snapshot
        .subcommand(Command::new("uninstall")
                    .aliases(["unin", "uninst", "unins", "un"])
                    .about("Uninstall package(s) from a snapshot")
                    .arg(Arg::new("live")
                         .long("live")
                         .short('l')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("enable live uninstall for snapshot"),)
                    .arg(Arg::new("noconfirm")
                         .long("noconfirm")
                         .short('y')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("noconfirm flag"),)
                  .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),)
                    .arg(Arg::new("PACKAGE")
                         .long("package")
                         .alias("pkg")
                         .short('p')
                         .num_args(1..)
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PROFILE", "USER_PROFILE"])
                         .help("package(s) to be uninstalled"),)
                    .arg(Arg::new("PROFILE")
                         .long("profile")
                         .alias("prof")
                         .short('P')
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PACKAGE", "USER_PROFILE"])
                         .conflicts_with("PACKAGE")
                         .help("profile(s) to be uninstalled"),)
                    .arg(Arg::new("USER_PROFILE")
                         .long("user-profile")
                         .alias("uprof")
                         .short('U')
                         .value_parser(clap::value_parser!(String))
                         .required_unless_present_any(["PACKAGE", "PROFILE"])
                         .conflicts_with_all(["PACKAGE", "PROFILE"])
                         .help("profile(s) to be uninstalled"),),)
        // Unlock a snapshot
        .subcommand(Command::new("unlock")
                    .alias("ul")
                    .about("Unlock a snapshot")
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Upgrade a snapshot
        .subcommand(Command::new("upgrade")
                    .alias("up")
                    .about("Update all packages in a snapshot")
                    .arg(Arg::new("noconfirm")
                         .long("noconfirm")
                         .short('y')
                         .required(false)
                         .action(ArgAction::SetTrue)
                         .help("noconfirm flag"),)
                    .arg(Arg::new("SNAPSHOT")
                         .long("snapshot")
                         .alias("snap")
                         .short('s')
                         .value_parser(clap::value_parser!(i32))
                         .required(false)
                         .help("snapshot number"),),)
        // Ash version
        .subcommand(Command::new("version")
                    .alias("v")
                    .about("Print ash version"))
        // Which snapshot(s) contain a package
        .subcommand(Command::new("whichsnap")
                    .alias("ws")
                    .about("Which snapshot has a package installed")
                    .arg_required_else_help(true)
                    .arg(Arg::new("PACKAGE")
                         .long("package")
                         .alias("pkg")
                         .short('p')
                         .num_args(1..)
                         .value_parser(clap::value_parser!(String))
                         .required(true)
                         .help("a package"),),)
        // Which deployment is active
        .subcommand(Command::new("whichtmp")
                    .aliases(["which", "whichdep"])
                    .about("Show which deployment snapshot is in use"));
        return matches;
}
