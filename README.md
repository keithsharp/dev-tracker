# Dev Tracker
Tracking project development progress for solo developers.

## Usage
Note all dates and times are stored in the database as UTC.  Input dates and times and displayed dates and times are in your local timezone.  You can also access the built in help with the command `dt help`.

### `dt add project <NAME> [PATH]`
Creates a new project with `NAME` and optionally creates a repository for the project at `PATH`.  Repositories can also be added to a project using the `dt add repo <PROJECT> <PATH>` command.  Returns an error if there is an existing project with the same name.

### `dt add activity-type <NAME> [DESCRIPTION]`
Creates a new activity type with `NAME` and and optional `DESCRPTION`.  Returns an error if there is an existing activity type with the same name.

### `dt add repo <PROJECT> <PATH>`
Creates a new repository at `PATH` and associates it with `PROJECT`.  Returns an error is there is no such project or there is an existing repository with the same path.

### `dt cancel activity <PROJECT>`
Cancels the running activity for `PROJECT`.  Returns an error if there is no such project or no running activity on the project.

### `dt delete project <NAME>`
Deletes the project with `NAME` and all of it's associated activities and repositories.  Returns an error if there is no such project.

### `dt delete activity <ID>`
Deletes the activity with `ID`.  Returns an error if there is no such activity.  To obtain the `ID` of an activity use the `-v` flag for the `dt list activities <PROJECT>` command.  Returns an error if there is no such activity.

### `dt delete activity-type <NAME>`
Deletes the activity type with `NAME`.  Returns an error if there is no such activity type.

### `dt delete repo <PATH>`
Deletes the repository with `PATH`.  Returns an error if there is no such repository.

### `dt describe project <NAME>`
Describes the project with `NAME`.  Lists the repositories, a count of the activities for the project, and a count of the total lines of code  for the project.  Returns an error if there is no such project.

### `dt describe activity <ID>`
Describes the activity with `ID`.  Lists the project name, the activity type, the start and end times (or still running), and the duration in minutes.  To obtain the `ID` of an activity use the `-v` flag for the `dt list activities <PROJECT>` command.  Returns an error if there is no such activity.

### `dt list projects [-v]`
List all of the projects in the database.  Use the optional `-v` flag to list the ID numbers for the projects.

### `dt list activities <PROJECT> [-v]`
List all of the activities for `PROJECT`.  Use the optional `-v` flag to list the ID numbers for the activities. Returns an error if there is no such project.

### `dt list activity-types [-v]`
List all of the activity types.  Use the optional `-v` flag to list the ID numbers for the activity types.

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

# TODO
1. LoC counting for each project and repository, display in describe project.
2. Remove the config file option.  Add a `--data-file` option to point to the database, have it configurable using an environment variable.
3. Add a `--json` flag (global/local) to make the output JSON.
4. Consider adding abbreviation aliases for other nouns.
5. Use `clap_complete` to generate shell completions, probably need `xtask`.
6. Translation/internationalization of the CLI.
7. Polish:
   1. Add help to Clap
   2. Documentation of `dev-tracker-core`.
   3. Tests for `dev-tracker-core`.


# Copyright and License
Copyright 2023, Keith Sharp, kms@passback.co.uk.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.