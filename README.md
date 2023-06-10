# Dev Tracker
Tracking project development progress for solo developers.

## Building and installing
To build Dev Tracker, clone the git repository and then run `cargo build`:
```bash
$ git clone https://github.com/keithsharp/dev-tracker.git
Cloning into 'dev-tracker'...
remote: Enumerating objects: 226, done.
remote: Counting objects: 100% (226/226), done.
remote: Compressing objects: 100% (152/152), done.
remote: Total 226 (delta 111), reused 186 (delta 71), pack-reused 0
Receiving objects: 100% (226/226), 60.14 KiB | 669.00 KiB/s, done.
Resolving deltas: 100% (111/111), done.
$ cd dev-tracker
$ cargo build
...
   Compiling dev-tracker-core v0.1.0 (/private/tmp/dev-tracker/dev-tracker-core)
   Compiling dev-tracker-cli v0.1.0 (/private/tmp/dev-tracker/dev-tracker-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 46.53s
$ cargo run -- list projects
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/dt list projects`
No projects in database
```
You can also install Dev Tracker using `cargo install`:
```bash
$ cargo install --path ./dev-tracker-cli
  Installing dev-tracker-cli v0.1.0 (/private/tmp/dev-tracker/dev-tracker-cli)
...
    Finished release [optimized] target(s) in 3.34s
  Installing /Users/kms/.cargo/bin/dt
   Installed package `dev-tracker-cli v0.1.0 (/private/tmp/dev-tracker/dev-tracker-cli)` (executable `dt`)
$ dt list projects
No projects in database
```
You'll need to have `.cargo/bin/` on your `PATH` to run the `dt` command following installation.  You can uninstall Dev Tracker with:
```bash
$ cargo uninstall dev-tracker-cli
    Removing /home/kms/.cargo/bin/dt
```

## Usage
Note all dates and times are stored in the database as UTC.  Input dates and times and displayed dates and times are in your local timezone.  You can also access the built in help with the command `dt help`.

### Data File
By default, `dt` looks for it's data file:
+ macOS - `$HOME/Library/Application Support/dev-tracker/dev-tracker.sqlite`
+ Linux - `$HOME/.config/dev-tracker/dev-tracker.sqlite` or `$XDG_CONFIG_HOME/dev-tracker/dev-tracker.sqlite`
+ Windows - should be in `{FOLDERID_RoamingAppData}`, but I've not tested this.

You can override the location by invoking `dt` with the `--data-file <PATH/TO/FILE>` option, for example:
```bash
dt --data-file /tmp/test.sqlite list projects
```
Or you can set the environment variable `DT_DATA_FILE`, for example:
```bash
export DT_DATA_FILE=/tmp/test.sqlite dt list projects
```

### `dt add project <NAME> [PATH]`
Creates a new project with `NAME` and optionally creates a repository for the project at `PATH`.  Repositories can also be added to a project using the `dt add repo <PROJECT> <PATH>` command.  Returns an error if there is an existing project with the same name.

### `dt add activity-type <NAME> [DESCRIPTION]`
Creates a new activity type with `NAME` and and optional `DESCRPTION`.  Returns an error if there is an existing activity type with the same name.

### `dt add repo <PROJECT> <PATH>`
Creates a new repository at `PATH` and associates it with `PROJECT`.  Returns an error is there is no such project or there is an existing repository with the same path.

### `dt cancel activity <PROJECT>`
Cancels the running activity for `PROJECT`.  Returns an error if there is no such project or no running activity on the project.

### `dt count <PROJECT>` 
Counts the lines of Rust code in `PROJECT`.  Returns an error is there is no such project.

### `dt delete project <NAME>`
Deletes the project with `NAME` and all of it's associated activities, repositories, and counts.  Returns an error if there is no such project.

### `dt delete activity <ID>`
Deletes the activity with `ID`.  Returns an error if there is no such activity.  To obtain the `ID` of an activity use the `-v` flag for the `dt list activities <PROJECT>` command.  Returns an error if there is no such activity.

### `dt delete activity-type <NAME>`
Deletes the activity type with `NAME`.  Returns an error if there is no such activity type.

### `dt delete count <ID>`
Deletes the count with `ID`.  To obtain the `ID` of a count use the `-v` flag for the `dt list counts <PROJECT>` command.  Returns an error if there is no such count.

### `dt delete repo <PATH>`
Deletes the repository with `PATH` and all of it's associated counts.  Returns an error if there is no such repository.

### `dt describe project <NAME>`
Describes the project with `NAME`.  Lists the repositories, a count of the activities for the project, and a count of the total lines of code  for the project.  Returns an error if there is no such project.

### `dt describe activity <ID>`
Describes the activity with `ID`.  Lists the project name, the activity type, the start and end times (or still running), and the duration in minutes.  To obtain the `ID` of an activity use the `-v` flag for the `dt list activities <PROJECT>` command.  Returns an error if there is no such activity.

### `dt describe count <ID>`
Describes the count with `ID`.  Lists project name, the repository path, the date and time of the count, and the lines of Rust code.  To obtain the `ID` of a count use the `-v` flag for the `dt list counts <PROJECT>` command.  Returns an error if there is no such count, project, or repository.

### `dt list projects [-v]`
List all of the projects in the database.  Use the optional `-v` flag to list the ID numbers for the projects.

### `dt list activities <PROJECT> [-v]`
List all of the activities for `PROJECT`.  Use the optional `-v` flag to list the ID numbers for the activities. Returns an error if there is no such project.

### `dt list activity-types [-v]`
List all of the activity types.  Use the optional `-v` flag to list the ID numbers for the activity types.

### `dt list counts [-v] <PROJECT>`
List all of the counts for `PROJECT`.  Use the optional `-v` flag to list the ID numbers for the counts. Returns an error if there is no such project.

### `dt list repos [-v]`
List all of the repositories for `PROJECT`.  Use the optional `-v` flag to list the ID numbers for the repositories.  Returns an error if there is no such project.

### `dt rename project <OLD_NAME> <NEW_NAME>`
Renames a project from `OLD_NAME` to `NEW_NAME`.  Returns an error if there is no project with name `NEW_NAME` or if there is an existing project with `NEW_NAME`.

### `dt rename activity-type <OLD_NAME> <NEW_NAME>`
Renames an activity type from `OLD_NAME` to `NEW_NAME`.  Returns an error if there is no activity type with name `NEW_NAME` or if there is an existing activity type with `NEW_NAME`.

### `dt start activity <PROJECT> <ACTIVITY_TYPE> [DESCRIPTION]`
Start recording an activity for `PROJECT` with an activity type of `ACTIVITY_TYPE` and an optional `DESCRIPTION`.  Returns an error if there is no such project or activity type, or if there already an activity in progress for the project.

### `dt stop activity <PROJECT>`
Stops the current activity for `PROJECT`.  Returns an error if there is no such project or of there is no current activity in progress.

### `dt update activity end <ID> <END>`
Updates the end time for the activity with `ID`.  `END` should be in the format `YYYY-MM-DDTHH:MM` where the date and time are in your local timezone and the time uses the 24-hour clock.  To obtain the `ID` of an activity use the `-v` flag for the `dt list activities <PROJECT>` command.  Returns an error if there is no such activity, if the new end time is before the activity start time, or if there is an error parsing the date and time from your local timezone into UTC.

### `dt update activity activity-type <ID> <ACTIVITY_TYPE>`
Updates the activity type for the activity with `ID`.  Returns an error if there is no such activity or no such activity type.

### `dt update activity description <ID> [DESCRIPTION]`
Updates the description for the activity with `ID`.  Use this command with no value for `DESCRIPTION` to remove an existing description.  To obtain the `ID` of an activity use the `-v` flag for the `dt list activities <PROJECT>` command.  Returns an error if there is no such activity.

### `dt update activity project <ID> <PROJECT>`
Updates the project for the activity with `ID`.  To obtain the `ID` of an activity use the `-v` flag for the `dt list activities <PROJECT>` command.  Returns an error if there is no such activity or no such project.

### `dt update activity-type <NAME> [DESCRIPTION]`
Updates the description for the activity type with `NAME`.  Use this command with no value for `DESCRIPTION` to remove an existing description.  Returns an error if there is no such activity type.

### `dt update repo <OLD_PATH> <NEW_PATH>`
Updates the path for the repository with `OLD_PATH`.  Returns an error if there is no such repository  with `OLD_PATH` or if there is an existing repository with `NEW_PATH`.

# Copyright and License
Copyright 2023, Keith Sharp, kms@passback.co.uk.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.