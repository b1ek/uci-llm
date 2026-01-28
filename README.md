# Rust UCI boilerplate
boilerplate for making chess engines with rust. basic UCI protocol is already implemented for you and your algorithm goes in `src/command/go.rs`.

imo the most hard part of any proper UCI implementation is gracefully stopping and exiting which is what this boilerplate does for you out of the box. how you write your go command is up to you but you don't have to figure out how to implement stoppings and whatnot.

# license
```txt
Rust UCI boilerplate
Copyright (C) 2026 blek <me@blek.codes>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
